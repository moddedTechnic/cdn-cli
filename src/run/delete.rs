use crate::cli::{Command, Delete};

impl Delete {
    pub async fn run(&self, common: &Command) {
        println!("Deleting {self:#?}");
    }
}
