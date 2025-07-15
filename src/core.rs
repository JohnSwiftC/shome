use std::sync::{mpsc, Arc, Mutex};
pub mod airplay;
pub mod upnp;
use std::collections::HashMap;

/// Used to pass important context and data to commands, especially
/// commands that start new threads
pub struct EngineContext {
    job_manager: Arc<Mutex<JobManager>>,
    upnp_device_manager: Arc<Mutex<upnp::DeviceManager>>,
}

impl EngineContext {
    pub fn new() -> Self {
        Self {
            job_manager: Arc::new(Mutex::new(JobManager::new())),
            upnp_device_manager: Arc::new(Mutex::new(upnp::DeviceManager::new())),
        }
    }
}

pub enum CommandResult {
    Success { message: String },
    SuccessWithJob { message: String, job: Job },
    Failure { message: String },
}
pub trait Command {
    fn process(&self, input: &str) -> Result<CommandResult, CommandResult> {
        match input {
            "help" => {
                return Ok(CommandResult::Success {
                    message: self.info().to_owned(),
                })
            }
            &_ => (),
        }

        self.run(input)
    }
    fn run(&self, input: &str) -> Result<CommandResult, CommandResult>;
    fn info(&self) -> &str;
    fn name(&self) -> &str;
}

pub struct CommandRouter {
    name: &'static str,
    info: &'static str,
    commands: Vec<Box<dyn Command>>,
    routers: Vec<CommandRouter>,
}

impl CommandRouter {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            info: "No info for this router",
            commands: Vec::new(),
            routers: Vec::new(),
        }
    }

    pub fn register<T: Command + 'static>(&mut self, command: T) {
        self.commands.push(Box::new(command));
    }

    pub fn register_router(&mut self, router: CommandRouter) {
        self.routers.push(router);
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_info(&mut self, info: &'static str) {
        self.info = info;
    }

    pub fn parse(&self, input: &str) -> Result<CommandResult, CommandResult> {
        let input = input.trim();

        if input == "" {
            return Err(CommandResult::Failure {
                message: format!(
                    "{} is a command router/module, \
                     not a command. Use '{} help' \
                     to see commands and sub-modules",
                    self.name, self.name
                ),
            });
        }

        if input == "help" {
            return Ok(CommandResult::Success {
                message: format!("{}\n{}", self.info, self.generate_help()),
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
            ret.push_str("- There are no commands in this module.\n")
        }

        for c in &self.commands {
            ret.push_str(&format!("- {}\n", c.name()));
        }

        ret.push_str("Modules/Routers\n");

        if self.routers.len() == 0 {
            ret.push_str("- There are no submodules in this module.\n");
        }

        for r in &self.routers {
            ret.push_str(&format!("- {}\n", r.name()));
        }

        ret
    }
}

/// The JobManager keeps tracks of mpsc channels used to control threads started
/// as a result of commands, particularly services that do not normally end.
/// It is the command's responsibilty to propogate a sender
pub struct JobManager {
    jobs: Vec<Job>,
}

impl JobManager {
    pub fn new() -> Self {
        Self { jobs: Vec::new() }
    }

    pub fn insert(&mut self, job: Job) {
        self.jobs.push(job);
    }

    pub fn list_current_jobs(&self) -> String {
        let mut ret = String::new();

        for (i, j) in self.jobs.iter().enumerate() {
            ret.push_str(&format!("({}) > {}", i, j.name))
        }

        ret
    }

    pub fn kill(&mut self, index: usize) -> Result<CommandResult, CommandResult> {
        if index >= self.jobs.len() {
            return Err(CommandResult::Failure {
                message: format!("there is no job at index {}", index),
            });
        }

        self.jobs[index]
            .sender
            .send(())
            .map_err(|e| CommandResult::Failure {
                message: format!("error trying to kill job: {}", e),
            })?;

        let job_name = self.jobs[index].name.clone();
        self.jobs.remove(index);

        Ok(CommandResult::Success {
            message: format!("job '{}' at index {} killed", job_name, index),
        })
    }
}

pub struct Job {
    name: String,
    sender: mpsc::Sender<()>,
}
