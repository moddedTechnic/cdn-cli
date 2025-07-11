use crate::{
    cli::{Command, Register, RegisterR2, Unregister},
    config::{Bucket, Config},
};

impl Register {
    pub async fn run(&self, common: &Command) {
        match self {
            Self::R2(reg) => reg.run(common).await,
        }
    }
}

impl RegisterR2 {
    pub async fn run(&self, common: &Command) {
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

        let endpoint = format!(
            "https://{account}.r2.cloudflarestorage.com",
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

impl Unregister {
    pub async fn run(&self, common: &Command) {
        let config_path = common.config_path();
        let mut config = if config_path.exists() {
            Config::load(&config_path).expect("Failed to load config")
        } else {
            println!("No config found at {config_path:?}");
            return;
        };

        if let Some(ref default) = config.default {
            if String::from(default) == self.domain.to_string() {
                config.default = None;
            }
        }

        config.buckets = config
            .buckets
            .into_iter()
            .filter(|bucket| bucket.domain != self.domain.to_string())
            .collect();

        config.save(&config_path).unwrap();
    }
}
