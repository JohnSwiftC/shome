use crate::{
    core::{EngineContext, Job},
    utils, Command, CommandResult,
};

struct UPnPDos {}
impl Command for UPnPDos {
    fn name(&self) -> &str {
        "dos"
    }

    fn info(&self) -> &str {
        "DoS based on the SSDP Search protocol"
    }

    fn run(&self, input: &str, context: &EngineContext) -> Result<CommandResult, CommandResult> {
        Ok(CommandResult::Success {
            message: "".to_owned(),
        })
    }
}
