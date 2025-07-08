use crate::cli::{Command, Register, RegisterR2};

impl Register {
    pub fn run(&self, common: &Command) {
        match self {
            Self::R2(reg) => reg.run(common),
        }
    }
}

impl RegisterR2 {
    pub fn run(&self, common: &Command) {
        println!("Registering an R2 bucket: {self:#?}")
    }
}
