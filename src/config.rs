use std::{collections::HashMap, rc::Rc};

/// # Example
///
/// ```toml
/// ["cdn.example.com"]
/// endpoint = "..."
/// bucket = "..."
/// access_key = "..."
/// secret_key = "..."
///
/// ["cdn2.example.com"]
/// endpoint = "..."
/// bucket = "..."
/// access_key = "..."
/// secret_key = "..."
/// index = "00000000-0000-0000-0000-000000000000"
/// ```
#[derive(Clone, Debug)]
pub struct Config(HashMap<String, Bucket>);

#[derive(Clone, Debug)]
pub struct Bucket {
    endpoint: Rc<str>,
    bucket: Rc<str>,
    access_key: Rc<str>,
    secret_key: Rc<str>,
    index: Option<Rc<str>>,
}
