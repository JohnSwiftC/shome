use std::io::{stdin, stdout, Write};
use std::sync::{Arc, Mutex};
use std::task::Context;
use std::thread;

mod core;
mod utils;

use core::{Command, CommandResult, CommandRouter, JobManager, upnp::DeviceManager};

use crate::core::{upnp, EngineContext};

fn main() {
    // Routers
    let mut main_router = CommandRouter::new("main");
    main_router.set_info("The main module.");

    // Register main commands and submodules

    main_router.register_router(core::airplay::router());
    main_router.register_router(core::upnp::router());
    main_router.register(List {});
    main_router.register(Kill {});

    // Context TODO: actually use contect managers in list command
    // TODO: MAKE LIST COMMAND AND KILL REAL COMMANDS

    let context = EngineContext::new();

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


        match main_router.parse(&line, &context) {
            Ok(CommandResult::Success { message }) => println!("{}", message),
            Err(CommandResult::Failure { message }) => eprintln!("ERROR: {}", message),
            _ => (),
        }
    }
}

struct List {}

impl Command for List {
    fn name(&self) -> &str {
        "list"
    }

    fn info(&self) -> &str {
        "used to list out specific internal lists, like jobs\n\
        Possible lists:\n\
        jobs\n\
        upnp\n\
        Usage:\n\
        list <list> : returns a formated string of the list"
    }
    
    fn run(&self, input: &str, context: &EngineContext) -> Result<CommandResult, CommandResult> {
        match input.trim() {
            "jobs" => {
                let lock = context.job_manager.lock().expect("Failed to lock job manager, quitting...");
                return Ok(
                    CommandResult::Success { message: lock.list_current_jobs() }
                )
            }
            "upnp" => {
                let lock = context.upnp_device_manager.lock().expect("Failed to lock UPnP device manager, quitting...");
                return Ok(
                    CommandResult::Success { message: lock.list_current_devices() }
                )
            }
            bad_in => {
                return Err(CommandResult::Failure { message: format!("{} is not a valid list", bad_in) })
            }
        }
    }
}

struct Kill {}
impl Command for Kill {
    fn name(&self) -> &str {
        "kill"
    }

    fn info(&self) -> &str {
        "Used to kill a currently running job, based on index.\n\
        Use 'list jobs' to see a list of currently running jobs.\n\
        Usage:\n\
        kill <index> : kills the job at index"
    }

    fn run(&self, input: &str, context: &EngineContext) -> Result<CommandResult, CommandResult> {
        let index: usize = input.trim().parse().map_err(|e| {
            CommandResult::Failure { message: format!("not a valid integer: {}", e) }
        })?;

        let mut lock = context.job_manager.lock().expect("failed to lock job manager, quitting...");
        lock.kill(index)
    }
}
