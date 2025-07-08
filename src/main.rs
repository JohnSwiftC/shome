use std::io::{stdin, stdout, Write};
use std::sync::mpsc;

mod core;
mod utils;

use core::{Command, CommandResult, CommandRouter};

fn main() {
    // Routers
    let mut main_router = CommandRouter::new("main");
    main_router.set_info("The main module.");

    // Register main commands and submodules

    main_router.register_router(core::airplay::router());

    // Input Loop
    let mut stdout = stdout();
    let stdin = stdin();
    let mut line = String::new();
    loop {
        stdout
            .write_all("shome > ".as_bytes())
            .expect("failed to write to stdout, panic now");
        stdout.flush().expect("failed to flush stdout, panic now");
        stdin
            .read_line(&mut line)
            .expect("failed to read from stdin, panic now");

        match main_router.parse(&line) {
            Ok(CommandResult::Success { message }) => println!("{}", message),
            Ok(CommandResult::SuccessWithJob { message, job }) => {

            },
            Err(CommandResult::Failure { message }) => println!("ERROR: {}", message),
            _ => (),
        }

        line = String::new();
    }
}
