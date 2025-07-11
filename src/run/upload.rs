use std::{fs::File, io::Read};

use crate::{
    cli::{Command, Upload},
    config::Config,
};
use aws_sdk_s3::{Client, config::Region, primitives::ByteStream};
use mime_guess::MimeGuess;
use uuid::Uuid;

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

        let mime = self.mime();

        let uuid = Uuid::new_v4().to_string();
        let key: &str = self.path.as_deref().unwrap_or(&uuid);

        client
            .put_object()
            .bucket(&bucket.bucket)
            .key(key)
            .body(ByteStream::from(buffer))
            .content_type(mime)
            .send()
            .await
            .expect("Upload failed");

        println!(
            "Uploaded {file:?} to {bucket:?} at path {path}",
            file = self.file,
            bucket = bucket.domain,
            path = key,
        );
    }

    fn mime(&self) -> String {
        if let Some(ref m) = self.mime {
            return m.to_string();
        }
        if let Some(guess) = MimeGuess::from_path(&self.file).first_raw() {
            return guess.to_string();
        }
        let mut file = File::open(&self.file).unwrap();
        let mut buffer = [0u8; 512];
        let n = file.read(&mut buffer).unwrap_or(0);
        let is_text = std::str::from_utf8(&buffer[..n]).is_ok();
        if is_text {
            "text/plain"
        } else {
            "application/octet-stream"
        }
        .to_string()
    }
}
