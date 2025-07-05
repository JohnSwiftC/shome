use std::thread;
use std::io::{stdin, stdout, Write};

mod commands;
mod utils;

fn main() {

    // Routers
    let mut main_router = CommandRouter::new("main");
    main_router.set_info("The main module.");

    let mut airplay_router = CommandRouter::new("airplay");
    airplay_router.set_info("A module for interacting with and as AirPlay devices.");

    let mut upnp_router = CommandRouter::new("upnp");

    // Commands
    let airplay_flood = commands::AirplayFlood {};

    // Give ownership in order
    airplay_router.register(airplay_flood);
    main_router.register_router(airplay_router);

    // Input Loop
    let mut stdout = stdout();
    let mut stdin = stdin();
    let mut line = String::new();
    loop {
        stdout.write_all("shome > ".as_bytes()).expect("failed to write to stdout, panic now");
        stdout.flush().expect("failed to flush stdout, panic now");
        stdin.read_line(&mut line).expect("failed to read from stdin, panic now");

        match main_router.parse(&line) {
            Ok(CommandResult::Success { message }) => println!("{}", message),
            Err(CommandResult::Failure { message }) => println!("ERROR: {}", message),
            _ => (),
        }

        line = String::new();
    }

    //airplay_device_flood("Zwduwidwidncnwudg8qjsowndqsw9wdnqud9wqjd", 600);
}

enum CommandResult {
    Success { message: String },
    Failure { message: String },
}
trait Command {
    fn process(&self, input: &str) -> Result<CommandResult, CommandResult> {
         match input {
            "help" => {
                return Ok(CommandResult::Success { message: self.info().to_owned() })
            },
            &_ => (),
         }

         self.run(input)
    }
    fn run(&self, input: &str) -> Result<CommandResult, CommandResult>;
    fn info(&self) -> &str;
    fn name(&self) -> &str;
}

struct CommandRouter {
    name: &'static str,
    info: &'static str,
    commands: Vec<Box<dyn Command>>,
    routers: Vec<CommandRouter>,
}

impl CommandRouter {
    fn new(name: &'static str) -> Self {
        Self {
            name,
            info: "No info for this router",
            commands: Vec::new(),
            routers: Vec::new(),
        }
    }

    fn register<T: Command + 'static>(&mut self, command: T) {
        self.commands.push(Box::new(command));
    }

    fn register_router(&mut self, router: CommandRouter) {
        self.routers.push(router);
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn set_info(&mut self, info: &'static str) {
        self.info = info;
    }

    fn parse(&self, input: &str) -> Result<CommandResult, CommandResult> {
        let input = input.trim();

        if input == "" {
            return Err(CommandResult::Failure {
                message: format!(
                    "{} is a command router/module, \
                     not a command. Append 'help' to your command \
                     to see commands and sub-modules",
                    self.name
                ),
            });
        }

        if input == "help" {
            return Ok(CommandResult::Success {
                message: format!(
                    "\
            {}\n\
            {}
                ",
                    self.info,
                    self.generate_help()
                ),
            });
        }

        let (first, rest) = match input.split_once(" ") {
            Some((first, rest)) => (first, rest),
            None => (input, ""),
        };

        for command in &self.commands {
            if command.name() == first {
                return command.process(rest);
            }
        }

        for router in &self.routers {
            if router.name() == first {
               return router.parse(rest);
            }
        }

        Err(CommandResult::Failure {
            message: "Command does not exist. Use help to see available commands and modules!"
                .to_owned(),
        })
    }

    fn generate_help(&self) -> String {
        let mut ret = String::new();

        ret.push_str("Commands\n");

        if self.commands.len() == 0 {
            ret.push_str("There are no commands in this module.\n")
        }

        for c in &self.commands {
            ret.push_str(&format!("- {}\n", c.name()));
        }

        ret.push_str("Modules/Routers\n");

        if self.routers.len() == 0 {
            ret.push_str("There are no submodules in this module.\n");
        }

        for r in &self.routers {
            ret.push_str(&format!("- {}\n", r.name()));
        }

        ret
    }
}
