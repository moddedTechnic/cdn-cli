use std::{path::Path, rc::Rc};

/// # Usage
///
/// ```sh
/// cdn [--config CONFIG] register r2 <DOMAIN> <ACCOUNT_ID> <BUCKET_NAME> [--index]
/// cdn [--config CONFIG] upload [--target DOMAIN] [--mime MIME] [--path PATH] <FILE>
/// cdn [--config CONFIG] delete [--target DOMAIN] <PATH>
/// ```
#[derive(Clone, Debug)]
pub struct Command {
    pub config: Option<Rc<Path>>,
    pub subcommand: SubCommand,
}

#[derive(Clone, Debug)]
pub enum SubCommand {
    Register(Register),
    Upload(Upload),
    Delete(Delete),
}

#[derive(Clone, Debug)]
pub enum Register {
    R2(RegisterR2),
}

#[derive(Clone, Debug)]
pub struct RegisterR2 {
    pub domain: Rc<str>,
    pub account_id: Rc<str>,
    pub bucket_name: Rc<str>,
    pub index: bool,
}

#[derive(Clone, Debug)]
pub struct Upload {
    pub target: Option<Rc<str>>,
    pub mime: Option<Rc<str>>,
    pub path: Option<Rc<str>>,
    pub file: Rc<Path>,
}

#[derive(Clone, Debug)]
pub struct Delete {
    pub target: Option<Rc<str>>,
    pub path: Rc<str>,
}
