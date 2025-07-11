use std::{path::PathBuf, sync::Arc};

use clap::{Args, Parser, Subcommand};

/// # Usage
///
/// ```sh
/// cdn [--config CONFIG] register r2 <DOMAIN> <ACCOUNT_ID> <BUCKET_NAME> [--index] [--default]
/// cdn [--config CONFIG] unregister <DOMAIN>
/// cdn [--config CONFIG] upload [--target DOMAIN] [--mime MIME] [--path PATH] [--password] <FILE>
/// cdn [--config CONFIG] delete [--target DOMAIN] <PATH>
/// cdn [--config CONFIG] get [--target DOMAIN] <PATH>
/// cdn [--config CONFIG] list buckets
/// cdn [--config CONFIG] list files [--target DOMAIN]
/// cdn [--config CONFIG] configure default <DOMAIN>
/// ```
#[derive(Clone, Debug, Parser)]
pub struct Command {
    #[clap(long)]
    pub config: Option<PathBuf>,
    #[command(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(Clone, Debug, Subcommand)]
pub enum SubCommand {
    Register {
        #[command(subcommand)]
        mode: Register,
    },
    Unregister(Unregister),
    Upload(Upload),
    Delete(Delete),
    Get(Get),
    List {
        #[command(subcommand)]
        mode: List,
    },
}

#[derive(Clone, Debug, Subcommand)]
pub enum Register {
    R2(RegisterR2),
}

#[derive(Args, Clone, Debug)]
pub struct Unregister {
    pub domain: Arc<str>,
}

#[derive(Args, Clone, Debug)]
pub struct RegisterR2 {
    pub domain: Arc<str>,
    pub account_id: Arc<str>,
    pub bucket_name: Arc<str>,
    #[clap(long)]
    pub index: bool,
    #[clap(long)]
    pub default: bool,
}

#[derive(Args, Clone, Debug)]
pub struct Upload {
    pub target: Option<Arc<str>>,
    pub mime: Option<Arc<str>>,
    pub path: Option<Arc<str>>,
    pub password: bool,
    pub file: PathBuf,
}

#[derive(Args, Clone, Debug)]
pub struct Delete {
    pub target: Option<Arc<str>>,
    pub path: Arc<str>,
}

#[derive(Args, Clone, Debug)]
pub struct Get {
    pub target: Option<Arc<str>>,
    pub path: Arc<str>,
}

#[derive(Clone, Debug, Subcommand)]
pub enum List {
    Buckets,
    Files(ListFiles),
}

#[derive(Args, Clone, Debug)]
pub struct ListFiles {
    pub target: Option<Arc<str>>,
}
