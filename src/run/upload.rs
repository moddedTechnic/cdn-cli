use crate::cli::{Command, Upload};

impl Upload {
    pub fn run(&self, common: &Command) {
        println!("Uploading {self:#?}");
    }
}
