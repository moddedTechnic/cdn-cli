use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use serde::{Deserialize, Serialize};

/// # Example
///
/// ```toml
/// default = "cdn.example.com"
///
/// [[bucket]]
/// domain = "cdn.example.com"
/// endpoint = "..."
/// bucket = "..."
/// access_key = "..."
/// secret_key = "..."
/// index = false
///
/// [[bucket]]
/// domain = "cdn2.example.com"
/// endpoint = "..."
/// bucket = "..."
/// access_key = "..."
/// secret_key = "..."
/// index = true
/// ```
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub default: Option<String>,
    pub buckets: Vec<Bucket>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            default: None,
            buckets: Vec::new(),
        }
    }

    pub fn load(path: &Path) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(toml::from_str(&contents).expect("Failed to deserialize TOML"))
    }

    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        println!("Saving to {path:?}");
        let toml_string = toml::to_string(self).expect("Failed to serialize to TOML");
        let mut file = File::create(path)?;
        file.write_all(toml_string.as_bytes())?;
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Bucket {
    pub domain: String,
    pub endpoint: String,
    pub bucket: String,
    pub access_key: String,
    pub secret_key: String,
    pub index: bool,
}
