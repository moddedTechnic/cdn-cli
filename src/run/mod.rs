pub mod delete;
pub mod list;
pub mod register;
pub mod upload;

use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use resolve_path::PathResolveExt;

use crate::cli::{Command, SubCommand};

impl Command {
    pub fn run(&self) {
        self.subcommand.run(self)
    }

    pub fn config_path(&self) -> Arc<Path> {
        self.config
            .clone()
            .unwrap_or(PathBuf::from("~/.cdncli"))
            .resolve()
            .into()
    }
}

impl SubCommand {
    pub fn run(&self, common: &Command) {
        match self {
            Self::Register { mode } => mode.run(common),
            Self::Unregister(unreg) => todo!(),
            Self::Upload(up) => up.run(common),
            Self::Delete(del) => del.run(common),
            Self::Get(get) => todo!(),
            Self::List { mode } => mode.run(common),
        }
    }
}
