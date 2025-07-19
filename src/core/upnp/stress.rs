use crate::{
    core::{EngineContext, Job},
    utils, Command, CommandResult,
};

struct UPnPStress {}
impl Command for UPnPStress {
    fn name(&self) -> &str {
        "dos"
    }

    fn info(&self) -> &str {
        "Stress testing based on the SSDP Search protocol\n\
        Flags:\n\
        -i or --index <index> (required) : device to target, from 'list upnp'"
    }

    fn run(&self, input: &str, context: &EngineContext) -> Result<CommandResult, CommandResult> {
        Ok(CommandResult::Success {
            message: "".to_owned(),
        })
    }
}
