use crate::{core::Job, utils, Command, CommandResult};

struct UPnPDos {}
impl Command for UPnPDos {
    fn name(&self) -> &str {
        "dos"
    }

    fn info(&self) -> &str {
        "DoS based on the SSDP Search protocol"
    }

    fn run(&self, input: &str) -> Result<CommandResult, CommandResult> {
        Ok(CommandResult::Success { message: "".to_owned() })
    }
}