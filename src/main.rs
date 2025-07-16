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
