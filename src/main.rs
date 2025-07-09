mod cli;
mod config;
mod run;
mod state;

use crate::cli::*;

fn main() {
    let cmd = Command {
        config: None,
        subcommand: SubCommand::Register(Register::R2(RegisterR2 {
            domain: "cdn.example.com".into(),
            account_id: "example".into(),
            bucket_name: "cdn".into(),
            index: false,
            default: false,
        })),
    };
    cmd.run();

    // let cmd = Command {
    //     config: None,
    //     subcommand: SubCommand::Upload(Upload {
    //         target: None,
    //         mime: None,
    //         path: Some("Cargo.toml".into()),
    //         password: false,
    //         file: PathBuf::from("./Cargo.toml").into(),
    //     }),
    // };
    // cmd.run();

    // let cmd = Command {
    //     config: None,
    //     subcommand: SubCommand::Delete(Delete {
    //         target: Some("cdn.example.com".into()),
    //         path: "Cargo.toml".into(),
    //     }),
    // };
    // cmd.run();
}
