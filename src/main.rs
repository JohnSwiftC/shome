use std::thread;

mod commands;
mod utils;

fn main() {
    let mut main_router = CommandRouter::new("main");
    main_router.set_info("The main module.");
    let mut airplay_router = CommandRouter::new("airplay");
    airplay_router.set_info("A module for interacting with and as AirPlay devices.");
    let mut upnp_router = CommandRouter::new("upnp");

    let airplay_flood = commands::AirplayFlood {};

    airplay_router.register(airplay_flood);

    main_router.register_router(airplay_router);

    match main_router.parse("airplay help") {
        Ok(CommandResult::Success { message }) => println!("{}", message),
        Err(CommandResult::Failure { message }) => println!("ERROR: {}", message),
        _ => (),
    }

    loop {}

    //airplay_device_flood("Zwduwidwidncnwudg8qjsowndqsw9wdnqud9wqjd", 600);
}

enum CommandResult {
    Success { message: String },
    Failure { message: String },
}
trait Command {
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
        if input.trim() == "" {
            return Err(CommandResult::Failure {
                message: format!(
                    "{} is a command router/module, \
                    not a command. Append 'help' to your command \
                    to see commands and sub-modules",
                    self.name
                ),
            });
        }

        if input.trim() == "help" {
            return Ok(CommandResult::Success {
                message: format!("\
            {}\n\
            {}
                ", self.info, self.generate_help()) 
            })
        }

        let (first, rest) = match input.split_once(" ") {
            Some((first, rest)) => (first, rest),
            None => (input, ""),
        };

        for command in &self.commands {
            if command.name() == first {
                return command.run(rest);
            }
        }

        for router in &self.routers {
            if router.name() == first {
                return router.parse(rest);
            }
        }

        Err(CommandResult::Failure {
            message: "Command does not exist. Use help to see available commands and modules!".to_owned(),
        })
    }

    fn generate_help(&self) -> String {
        let mut ret = String::new();

        ret.push_str("Commands\n\n");

        for c in &self.commands {
            ret.push_str(&format!("- {}\n", c.name()));
        }

        ret.push_str("\nModules/Routers\n\n");

        for r in &self.routers {
            ret.push_str(&format!("- {}\n", r.name()));
        }

        ret
    }
}
