mod cli;
mod config;
mod run;
mod state;

use clap::Parser;

use crate::cli::Command;

fn main() {
    let cmd = Command::parse();
    cmd.run();
}
