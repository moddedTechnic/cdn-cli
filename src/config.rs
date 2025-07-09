use std::{path::Path, rc::Rc};

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
#[derive(Clone, Debug)]
pub struct Config {
    pub default: Option<Rc<str>>,
    pub buckets: Vec<Bucket>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            default: None,
            buckets: Vec::new(),
        }
    }

    pub fn load(_path: &Path) -> Option<Self> {
        todo!("Implement via serde")
    }

    pub fn save(&self, _path: &Path) -> Result<(), ()> {
        // FIXME: swap for serde

        let mut lines = Vec::new();
        if let Some(ref default) = self.default {
            lines.push(format!(r#"default = "{default}""#));
            lines.push("".into());
        }

        for bucket in &self.buckets {
            lines.push("[[bucket]]".into());
            lines.push(format!(r#"domain = "{}""#, bucket.domain));
            lines.push(format!(r#"endpoint = "{}""#, bucket.endpoint));
            lines.push(format!(r#"bucket = "{}""#, bucket.bucket));
            lines.push(format!(r#"access_key = "{}""#, bucket.access_key));
            lines.push(format!(r#"secret_key = "{}""#, bucket.secret_key));
            lines.push(format!(r#"index = {}"#, if bucket.index {"true"} else {"false"}));
            lines.push("".into());
        }

        let config = lines.join("\n".into());
        println!("{config}");

        // TODO: save to file
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Bucket {
    pub domain: Rc<str>,
    pub endpoint: Rc<str>,
    pub bucket: Rc<str>,
    pub access_key: Rc<str>,
    pub secret_key: Rc<str>,
    pub index: bool,
}
