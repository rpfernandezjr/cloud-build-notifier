pub mod args;
pub mod config;
pub mod event;
pub mod gcp;
pub mod notify;
pub mod slack;
pub mod validate;

use serde::Deserialize;

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
