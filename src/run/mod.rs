pub mod delete;
pub mod register;
pub mod upload;

use std::{
    path::{Path, PathBuf},
    rc::Rc,
};

use crate::cli::{Command, SubCommand};

impl Command {
    pub fn run(&self) {
        self.subcommand.run(self)
    }

    pub fn config_path(&self) -> Rc<Path> {
        self.config
            .clone()
            .unwrap_or(PathBuf::from("~/.cdncli").into())
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
