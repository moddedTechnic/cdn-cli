use crate::{
    cli::{Command, Register, RegisterR2},
    config::{Bucket, Config},
};

impl Register {
    pub fn run(&self, common: &Command) {
        match self {
            Self::R2(reg) => reg.run(common),
        }
    }
}

impl RegisterR2 {
    pub fn run(&self, common: &Command) {
        println!();

        let config_path = common.config_path();
        let mut config = if config_path.exists() {
            println!("Appending to config");
            Config::load(&config_path).expect("Failed to load config")
        } else {
            println!("Creating config");
            Config::new()
        };

        println!();
        println!("Registering an R2 bucket: {self:#?}");

        println!();
        println!("{config:#?}");

        // TODO: check endpoint format
        let endpoint = format!(
            "https://{account}.r2.cloudflarestorage.com/",
            account = self.account_id
        );

        // TODO: use a proper get secret input thing
        let access_key = "";
        let secret_key = "";

        let bucket = Bucket {
            domain: self.domain.clone().to_string(),
            endpoint: endpoint.into(),
            bucket: self.bucket_name.clone().to_string(),
            access_key: access_key.to_string(),
            secret_key: secret_key.to_string(),
            index: self.index.clone(),
        };

        if self.default || config.default.is_none() {
            config.default = Some(bucket.domain.clone());
        }
        config.buckets.push(bucket);

        println!("Updated config:");
        println!("{config:#?}");

        config.save(&config_path).unwrap();
    }
}
