use crate::{Command, CommandResult};

mod airplay;
mod upnp;

struct AirplayFlood;
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
                            message: format!("amount unable to be parsed {}", e),
                        })?;
                    } else {
                        return Err(CommandResult::Failure { message: "amount flag used but no amount specified".to_owned() })
                    }
                }

                "-n" | "--name" => {
                    if let Some(n) = args.next() {
                        name = n.to_owned();
                    } else {
                        return Err(CommandResult::Failure { message: "name flag used but no name specified".to_owned() })
                    }
                }

                &_ => {
                    return Err(CommandResult::Failure {
                        message: "unrecognized flag used".to_owned(),
                    })
                }
            }
        }



        Ok(CommandResult::Success {
            message: "AirPlay flood started!".to_owned(),
        })
    }
}
