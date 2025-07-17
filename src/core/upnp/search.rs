use std::fs::File;
use std::io::Write;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use std::net::UdpSocket;

use crate::core::upnp::UPnPFriendlyIP;
use crate::{
    core::{upnp::DeviceManager, EngineContext, Job},
    utils, Command, CommandResult,
};
use utils::create_log_file;

pub struct UPnPSearch {}
impl Command for UPnPSearch {
    fn name(&self) -> &str {
        "search"
    }

    fn info(&self) -> &str {
        "Used to passively read UPnP devices broadcasting themselves\n\
        over SSDP, and then output those results to the upnp list.\n\
        Flags:\n\
        -o or --output <string> : also output to a named log file"
    }

    fn run(&self, input: &str, context: &EngineContext) -> Result<CommandResult, CommandResult> {
        let mut args = input.split_whitespace();
        let mut log_file_name = None;
        let mut log_file: Option<File>;

        while let Some(curr) = args.next() {
            match curr {
                "-o" | "--output" => {
                    if let Some(n) = args.next() {
                        log_file_name = Some(n.trim());
                    } else {
                        return Err(CommandResult::Failure {
                            message: "output flag used but no value given".to_owned(),
                        });
                    }
                }

                &_ => {
                    return Err(CommandResult::Failure {
                        message: "unrecognized flag used".to_owned(),
                    });
                }
            }
        }

        log_file = match log_file_name {
            Some(name) => Some(create_log_file(name)?),
            None => None,
        };

        let upnp_manager_arc = context.upnp_device_manager.clone();

        let socket = UdpSocket::bind("0.0.0.0:1900").map_err(|e| CommandResult::Failure {
            message: "udp socket already open at port 1900, consider killing related upnp jobs"
                .to_owned(),
        })?;

        let (sender, receiver) = mpsc::channel();

        let _ = thread::spawn(move || {
            let mut log_file = log_file;
            let socket = socket;
            while let Err(_) = receiver.try_recv() {
                let mut buf = vec![0; 4096];
                let (bytes, sender) = match socket.recv_from(&mut buf) {
                    Ok((bytes, sender)) => (bytes, sender),
                    Err(e) => {
                        eprintln!("ERROR: failed to read from udp socket, consider kill upnp-search job");
                        return;
                    }
                };
                let bufstr = String::from_utf8_lossy(&buf[..bytes]);

                if let Some(ref mut f) = log_file {
                    if let Err(e) = write!(f, "Packet from {}\n{}", sender, bufstr) {
                        eprintln!("ERROR: failed to write to log file, consider killing upnp-search job");
                    }
                }

                {
                    let mut lock = upnp_manager_arc.lock().expect("failed to lock upnp manager, quitting... (search)");
                    lock.insert(UPnPFriendlyIP {
                        ip: sender.ip(),
                        name: "some device".to_owned(),
                    });
                }

            }
        });

        {
            let mut lock = context
                .job_manager
                .lock()
                .expect("Failure when locking job manager, quitting...");
            let job = Job {
                name: "upnp-search".to_owned(),
                sender: sender,
            };
            lock.insert(job);
        }

        Ok(CommandResult::Success {
            message: "UPnP search job created!".to_owned(),
        })
    }
}
