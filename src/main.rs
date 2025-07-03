use std::thread;

mod commands;
mod utils;

fn main() {
    let mut main_router = CommandRouter::new("main");
    let mut airplay_router = CommandRouter::new("airplay");
    let mut upnp_router = CommandRouter::new("upnp");

    let airplay_flood = commands::AirplayFlood{};

    airplay_router.register(airplay_flood);

    main_router.register_router(airplay_router);

    match main_router.parse("airplay flood -a 5") {
        Ok(CommandResult::Success { message }) => println!("{}", message),
        Err(CommandResult::Failure { message }) => println!("{}", message),
        _ => (),
    }

    loop {

    }

    //airplay_device_flood("Zwduwidwidncnwudg8qjsowndqsw9wdnqud9wqjd", 600);
}

enum CommandResult {
    Success {
        message: String,
    },
    Failure {
        message: String,
    },
}
trait Command {
    fn run(&self, input: &str) -> Result<CommandResult, CommandResult>;
    fn info(&self) -> &str;
    fn name(&self) -> &str;
}

struct CommandRouter {
    name: &'static str,
    commands: Vec<Box<dyn Command>>,
    routers: Vec<CommandRouter>,
}

impl CommandRouter {
    fn new(name: &'static str) -> Self {
        Self {
            name,
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

    fn parse(&self, input: &str) -> Result<CommandResult, CommandResult> {
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
            message: "Command does not exist. Use help to see available commands!".to_owned(),
        })
    }
}
