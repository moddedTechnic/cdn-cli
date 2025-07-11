pub mod delete;
pub mod list;
pub mod registration;
pub mod upload;

use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use resolve_path::PathResolveExt;

use crate::cli::{Command, SubCommand};

impl Command {
    pub async fn run(&self) {
        self.subcommand.run(self).await;
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
    pub async fn run(&self, common: &Command) {
        match self {
            Self::Register { mode } => mode.run(common).await,
            Self::Unregister(unreg) => unreg.run(common).await,
            Self::Upload(up) => up.run(common).await,
            Self::Delete(del) => del.run(common).await,
            Self::Get(get) => todo!(),
            Self::List { mode } => mode.run(common).await,
        };
    }
}
