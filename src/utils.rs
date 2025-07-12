use crate::CommandResult;

#[derive(Clone)]
pub struct MacAddr {
    addr: [u8; 6],
}

impl MacAddr {
    pub fn as_string(&self) -> String {
        format!(
            "{:02X?}{:02X?}{:02X?}{:02X?}{:02X?}{:02X?}",
            self.addr[0], self.addr[1], self.addr[2], self.addr[3], self.addr[4], self.addr[5]
        )
    }

    pub fn new_zeroed() -> Self {
        Self { addr: [0; 6] }
    }

    pub fn increment(&mut self) {
        for i in 0..6 {
            if self.addr[i] == 255 {
                continue;
            }

            self.addr[i] += 1;
            break;
        }
    }
}

use std::fs::File;
use std::io::ErrorKind;
pub fn create_log_file(name: &str) -> Result<File, CommandResult> {
    let mut log_iter = 1;
    loop {
        match File::create_new(format!("{}-{}.txt", name, log_iter)) {
            Ok(f) => {
                return Ok(f);
            }
            Err(e) => {
                if e.kind() == ErrorKind::AlreadyExists {
                    log_iter += 1;
                    continue;
                } else {
                    return Err(CommandResult::Failure {
                        message: format!("failed to create log file: {}", e),
                    });
                }
            }
        }
    }
}
