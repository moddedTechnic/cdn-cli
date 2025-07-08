use crate::cli::{Command, Delete};

impl Delete {
    pub fn run(&self, common: &Command) {
        println!("Deleting {self:#?}");
    }
}
