use std::{fs::File, io::Read};

use aws_sdk_s3::{Client, config::Region, primitives::ByteStream};
use uuid::Uuid;
use crate::{
    cli::{Command, Upload},
    config::Config,
};

impl Upload {
    pub async fn run(&self, common: &Command) {
        let config_path = common.config_path();
        if !config_path.exists() {
            println!("No config at {config_path:?}");
            return;
        }
        let config = Config::load(&config_path).unwrap();
        let bucket = config.get_bucket(self.target.as_deref());
        if bucket.is_none() {
            println!("Please specify a bucket to upload to");
            return;
        }
        let bucket = bucket.unwrap();

        let region = Region::new("auto");
        let credentials = bucket.get_creds();
        let config = aws_sdk_s3::config::Builder::new()
            .region(region)
            .endpoint_url(&bucket.endpoint)
            .credentials_provider(credentials)
            .build();
        let client = Client::from_conf(config);

        let mut file = File::open(&self.file).expect("Failed to open file");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        let key = Uuid::new_v4().to_string();

        client
            .put_object()
            .bucket(&bucket.bucket)
            .key(key)
            .body(ByteStream::from(buffer))
            .send()
            .await
            .expect("Upload failed");
    }
}
