use crate::{core::EngineContext, Command, CommandResult};

struct AirplaySearch {}
impl Command for AirplaySearch {
    fn name(&self) -> &str {
        "search"
    }

    fn info(&self) -> &str {
        "Searches for AirPlay devices and adds them to the airplay list."
    }

    fn run(&self, input: &str, context: &EngineContext) -> Result<CommandResult, CommandResult> {
        

        Ok(CommandResult::Success { message: "AirPlay search job started".to_owned() })
    }
}