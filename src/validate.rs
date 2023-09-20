use crate::config::Config;
use crate::event::get_build_time;
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::process::exit;
use tera::Tera;

fn load_json_file(filename: &str) -> Result<Value, Box<dyn Error>> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let loaded: Value = serde_json::from_str(contents.as_str())?;
    log::debug!("{:?}", loaded);
    Ok(loaded)
}

fn load_text_file(filename: &str) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(filename)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;
    Ok(file_contents)
}

fn is_json_valid(json_str: &str) -> bool {
    if let Ok(parsed_json) = serde_json::from_str::<Value>(json_str) {
        if parsed_json.is_object() {
            return true;
        }
    }
    false
}

pub fn template(config: &Config, template_key: &str, event_file: &str, log_file: Option<String>) {
    let event = match load_json_file(event_file) {
        Ok(v) => v,
        Err(e) => {
            log::error!("failed to load event from file={}: {}", event_file, e);
            exit(1);
        }
    };

    let template = match config.templates.contains_key(template_key) {
        true => config.templates.get(template_key).unwrap(),
        false => {
            log::error!("template key={} not found in config", template_key);
            exit(1);
        }
    };

    let logs = match log_file {
        Some(f) => match load_text_file(&f) {
            Ok(l) => Some(l),
            Err(e) => {
                log::error!("failed to load log file={}: {}", f, e);
                exit(1);
            }
        },
        None => None,
    };

    let build_time = get_build_time(&event);
    let mut context = tera::Context::new();
    context.insert("event", &event);
    context.insert("buildTime", &build_time);

    if let Some(v) = logs {
        context.insert("log", &v)
    }

    let rendered = match Tera::one_off(template, &context, false) {
        Ok(v) => v,
        Err(e) => {
            log::error!("failed to render template: {}", e);
            exit(1);
        }
    };
    log::info!("\n{}\n", rendered);

    match is_json_valid(&rendered) {
        true => log::info!("json is valid"),
        false => log::warn!("json is not valid"),
    }
}
