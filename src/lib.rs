pub mod args;
pub mod config;
pub mod event;
pub mod fetch_log;
pub mod gcp;
pub mod notify;
pub mod slack;
pub mod validate;

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub enum Notify {
    Slack(String),
    Off,
}

#[derive(Deserialize, Debug)]
pub enum Error {
    Deserialize(String),
    TemplateNotSet(String),
    SlackNotify(String),
    TemplateRender(String),
    EventParsing(String),
}

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub config: config::Config,
    pub notifiers: HashMap<String, Notify>,
    pub project: gcp::project::Project,
}
