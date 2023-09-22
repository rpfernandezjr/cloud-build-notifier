use serde_derive::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Deserialize)]
pub struct Input {
    pub project: String,
    pub subscription_id: String,
}

#[derive(Debug, Deserialize)]
pub struct Output {
    pub r#type: String,
    pub params: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct CustomTriggerStatus {
    pub r#type: Vec<String>,
    pub template: String,
}

#[derive(Debug, Deserialize)]
pub struct CustomTrigger {
    pub trigger_id: String,
    pub status: Vec<CustomTriggerStatus>,
    pub output: Option<Output>,
}

#[derive(Debug, Deserialize)]
pub struct Triggers {
    pub default: HashMap<String, Vec<String>>,
    pub custom: Option<Vec<CustomTrigger>>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub input: Input,
    pub output: Output,
    pub triggers: Triggers,
    pub templates: HashMap<String, String>,
}

impl Config {
    pub fn load(config_file: &str) -> Result<Config, Box<dyn Error>> {
        let mut file = File::open(config_file)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let loaded: Config = serde_yaml::from_str(contents.as_str())?;

        log::debug!("{:?}", loaded);
        Ok(loaded)
    }
}
