use std::fs::File;
use std::io::Write;
use std::sync::mpsc;
use std::thread;

use std::net::UdpSocket;

use crate::{
    core::{EngineContext, Job},
    utils, Command, CommandResult,
};
use utils::create_log_file;

fn read_ssdp_to_log(file: &mut File, socket: &mut UdpSocket) {
    let mut buf = vec![0; 4096];
    let (bytes, sender) = match socket.recv_from(&mut buf) {
        Ok((bytes, sender)) => (bytes, sender),
        Err(e) => {
            eprintln!("ERROR: failed to read from udp socket, consider kill upnp-search job");
            return;
        }
    };
    let bufstr = String::from_utf8_lossy(&buf[..bytes]);
    if let Err(e) = write!(file, "Packet from {}\n{}", sender, bufstr) {
        eprintln!("ERROR: failed to write to log file, consider killing upnp-search job");
    }
}

pub struct UPnPSearch {}
impl Command for UPnPSearch {
    fn name(&self) -> &str {
        "search"
    }

    fn info(&self) -> &str {
        "Used to passively read UPnP devices broadcasting themselves\n\
        over SSDP, and then output those results to a log file.\n\
        Flags:\n\
        -o or --output <string> : name of log file (default: shomelog-#.txt)"
    }

    fn run(&self, input: &str, context: &EngineContext) -> Result<CommandResult, CommandResult> {
        let mut args = input.split_whitespace();
        let mut log_file_name = "shomelog";
        let mut log_file: File;

        while let Some(curr) = args.next() {
            match curr {
                "-o" | "--output" => {
                    if let Some(n) = args.next() {
                        log_file_name = n.trim();
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

        log_file = create_log_file(log_file_name)?;
        let upnp_manager_arc = context.upnp_device_manager.clone();

        // TODO, rewrite the actual functionality here, add flag for optional
        // log file, ensure killing this job goes smoothly
        // create shared sockets in the context to prevent the double opening
        // error

        let mut socket = UdpSocket::bind("0.0.0.0:1900").map_err(|e| CommandResult::Failure {
            message: "udp socket already open at port 1900, consider killing related upnp jobs"
                .to_owned(),
        })?;

        let (sender, receiver) = mpsc::channel();

        let _ = thread::spawn(move || {
            let mut log_file = log_file;
            let mut socket = socket;
            while let Err(_) = receiver.try_recv() {
                read_ssdp_to_log(&mut log_file, &mut socket);
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
