use crate::airplay::airplay_device_flood;

mod airplay;
mod upnp;
mod utils;

fn main() {
    airplay_device_flood("Zwduwidwidncnwudg8qjsowndqsw9wdnqud9wqjd", 600);
}

enum CommandResult<'a> {
    Success {message: &'a str},
    Failure {message: &'a str},
}
trait Command {
    fn run(&self, input: &str) -> Result<CommandResult, CommandResult>;
    fn info(&self) -> &str;
    fn name(&self) -> &str;
}

struct CommandRouter {
    name: String,
    commands: Vec<Box<dyn Command>>,
    routers: Vec<CommandRouter>,
}

impl CommandRouter {

    fn name(&self) -> &str {
        &self.name
    }

    fn parse(&self,input: &str) -> Result<CommandResult, CommandResult> {
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

        Err(CommandResult::Failure { message: "Command does not exist. Use help to see available commands!" })
    }
}
