use crate::{
    cli::{Command, List},
    config::Config,
};

impl List {
    pub async fn run(&self, common: &Command) {
        match self {
            Self::Buckets => self.list_buckets(common),
            Self::Files(files) => todo!(),
        }
    }

    fn list_buckets(&self, common: &Command) {
        let config_path = common.config_path();
        let config = if config_path.exists() {
            Config::load(&config_path).expect("Failed to load config")
        } else {
            println!("No config file at {config_path:?}");
            return;
        };

        if config.buckets.len() == 0 {
            println!("No buckets found");
        }
        for bucket in config.buckets {
            println!(
                "{default}{name} at {url}",
                name = bucket.domain,
                url = bucket.endpoint,
                default = if Some(&bucket.domain) == config.default.as_ref() {
                    "*"
                } else {
                    ""
                },
            );
        }
    }
}
