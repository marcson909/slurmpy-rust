use std::default;
use serde::{Serialize, Deserialize};
use confy::load_path;

#[derive(Debug,Serialize, Deserialize)]
pub struct DefaultConfig {
    version: u8,
    api_key: String,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct MyConfig {
    defaults: DefaultConfig,
}

/// `DefaultConfig` implements `Default`
impl Default for DefaultConfig {
    fn default() -> Self { Self { version: 0, api_key: "".into() } }
}
/// `MyConfig` implements `Default`
impl Default for MyConfig {
    fn default() -> Self { Self { defaults: DefaultConfig::default() } }
}