pub mod delete;
pub mod register;
pub mod upload;

use crate::cli::{Command, SubCommand};

impl Command {
    pub fn run(&self) {
        self.subcommand.run(self)
    }
}

impl SubCommand {
    pub fn run(&self, common: &Command) {
        match self {
            Self::Register(reg) => reg.run(common),
            Self::Upload(up) => up.run(common),
            Self::Delete(del) => del.run(common),
        }
    }
}
