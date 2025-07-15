use std::io::{stdin, stdout, Write};

mod core;
mod utils;

use core::{Command, CommandResult, CommandRouter, JobManager, upnp::DeviceManager};

use crate::core::upnp;

fn main() {
    // Routers
    let mut main_router = CommandRouter::new("main");
    main_router.set_info("The main module.");

    // Register main commands and submodules

    main_router.register_router(core::airplay::router());
    main_router.register_router(core::upnp::router());
    main_router.register(KillDummy {});
    main_router.register(ListDummy {});

    // Managers

    let mut job_manager = JobManager::new();
    let mut upnp_device_manager = upnp::DeviceManager::new();

    // Input Loop
    let mut stdout = stdout();
    let stdin = stdin();
    let mut line: String;
    loop {
        line = String::new();
        stdout
            .write_all("shome > ".as_bytes())
            .expect("failed to write to stdout, panic now");
        stdout.flush().expect("failed to flush stdout, panic now");
        stdin
            .read_line(&mut line)
            .expect("failed to read from stdin, panic now");

        // Special commands that interact with structures in main
        // i dont think a framework for this is important because of the very
        // specific things that these commands do and that functionality
        // shouldnt be needed for normal commands
        let (first, rest) = match line.split_once(" ") {
            Some((first, rest)) => (first, rest),
            None => (line.as_str(), ""),
        };

        match first.trim() {
            "kill" => {
                if rest.trim() == "help" {
                    println!(
                        "Used to kill a currently running job\n\
                    run 'list jobs' to get the indexes of currently running jobs\n\
                    Usage:\nkill <index>"
                    );
                    continue;
                }

                let index = match rest.trim().parse::<usize>() {
                    Ok(i) => i,
                    Err(_) => {
                        println!("ERROR: kill takes a non-negative integer as an argument");
                        continue;
                    }
                };

                match job_manager.kill(index) {
                    Ok(CommandResult::Success { message }) => println!("{}", message),
                    Err(CommandResult::Failure { message }) => eprintln!("ERROR: {}", message),
                    _ => (),
                }

                continue;
            }

            "list" => {
                match rest.trim() {
                    "jobs" => println!("{}", job_manager.list_current_jobs()),
                    "" | "help" => println!(
                        "Shows specific lists\n\
                    Usage: list <list>\n\
                    Possible lists:\n\
                    - jobs"
                    ),
                    _ => eprintln!("{} is not a valid item to list", rest.trim()),
                }

                continue;
            }

            _ => (),
        }

        match main_router.parse(&line) {
            Ok(CommandResult::Success { message }) => println!("{}", message),
            Ok(CommandResult::SuccessWithJob { message, job }) => {
                println!("{}", message);
                job_manager.insert(job);
            }
            Err(CommandResult::Failure { message }) => eprintln!("ERROR: {}", message),
            _ => (),
        }
    }
}

// These two commands do nothing except ensure that the names show up in the main router's
// help menu. These are written manually and work outside the main router
struct KillDummy {}
impl Command for KillDummy {
    fn run(&self, _input: &str) -> Result<CommandResult, CommandResult> {
        Err(CommandResult::Failure {
            message: "internal error, kill should not be ran with a router".to_owned(),
        })
    }

    fn info(&self) -> &str {
        ""
    }
    fn name(&self) -> &str {
        "kill"
    }
}

struct ListDummy {}
impl Command for ListDummy {
    fn run(&self, _input: &str) -> Result<CommandResult, CommandResult> {
        Err(CommandResult::Failure {
            message: "internal error, kill should not be ran with a router".to_owned(),
        })
    }

    fn info(&self) -> &str {
        ""
    }
    fn name(&self) -> &str {
        "list"
    }
}
