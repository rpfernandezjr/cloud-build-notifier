use crate::config::Triggers;
use crate::fetch_log;
use crate::notify;
use crate::Error;
use crate::Settings;
use chrono::DateTime;
use chrono::Utc;
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;
use tera::Tera;

fn deserialize(json_str: &str) -> Result<Value, Error> {
    match serde_json::from_str(json_str) {
        Ok(v) => Ok(v),
        Err(e) => Err(Error::Deserialize(format!(
            "failed to deserialize event: {}",
            e
        ))),
    }
}

fn get_template(
    triggers: &Triggers,
    templates: &HashMap<String, String>,
    trigger_id: Option<&Value>,
    event_status: &str,
) -> Result<String, Error> {
    // Start with the default template key
    let mut template_key = triggers
        .default
        .iter()
        .find(|(_, values)| values.contains(&event_status.to_string()))
        .map(|(key, _)| key.to_string())
        .unwrap_or_else(String::new);

    // Check to see if there is an override template for this trigger_id
    if let Some(id) = trigger_id.and_then(|v| v.as_str()) {
        match &triggers.custom {
            Some(custom) => {
                if let Some(custom_trigger) = custom.iter().find(|x| x.trigger_id == id) {
                    if let Some(status) = custom_trigger
                        .status
                        .iter()
                        .find(|status| status.r#type.contains(&event_status.to_string()))
                    {
                        log::info!("Using custom trigger template");
                        template_key = status.template.clone();
                    }
                }
            }
            None => {}
        };
    }

    templates
        .get(&template_key)
        .map(|template| template.to_string())
        .ok_or_else(|| {
            Error::TemplateNotSet(format!("template not found for status={}", event_status))
        })
}

fn time_diff(end: &str, start: &str) -> String {
    let start_time: DateTime<Utc> = DateTime::parse_from_rfc3339(start).unwrap().into();
    let end_time: DateTime<Utc> = DateTime::parse_from_rfc3339(end).unwrap().into();
    let duration = end_time.signed_duration_since(start_time);
    let hours = duration.num_hours();
    let minutes = duration.num_minutes() % 60;
    let seconds = duration.num_seconds() % 60;

    format!("{}h {}m {}s", hours, minutes, seconds)
}

pub fn get_build_time(event: &Value) -> String {
    if event.get("finishTime").is_some() && event.get("startTime").is_some() {
        let start = event.get("startTime").unwrap().as_str().unwrap();
        let end = event.get("finishTime").unwrap().as_str().unwrap();
        return time_diff(end, start);
    }
    String::from("Build time not available")
}

pub async fn process(
    message_id: &str,
    event_str: String,
    settings: &Settings,
) -> Result<(), Error> {
    let event = deserialize(&event_str)?;
    let re = Regex::new(r"(?:\{\{[^{}]*\blog\b[^{}]*\}\}|\{%[^{}]*\blog\b[^{}]*%\})").unwrap();
    let status = event["status"].as_str().ok_or(Error::EventParsing(
        "failed to get event status".to_string(),
    ))?;

    let build_id = event["id"]
        .as_str()
        .ok_or(Error::EventParsing("failed to get trigger_id".to_string()))?;

    let trigger_id = event.get("buildTriggerId");
    log::info!(
        "processing message_id={} event_id={} status={}",
        message_id,
        build_id,
        status
    );

    let template = get_template(
        &settings.config.triggers,
        &settings.config.templates,
        trigger_id,
        status,
    )?;
    log::debug!("message_id={} template: {}", message_id, template);

    let build_time = get_build_time(&event);
    let mut context = tera::Context::new();
    context.insert("event", &event);
    context.insert("buildTime", &build_time);

    if re.is_match(&template) {
        match fetch_log::get_logs(&event, build_id, &settings.project).await {
            None => log::warn!("failed to download logs for build_id={}", build_id),
            Some(logs) => {
                log::debug!("log={}", logs);
                context.insert("log", &logs);
            }
        };
    }

    log::debug!("message_id={} context: {:?}", message_id, context);

    let rendered = Tera::one_off(&template, &context, false).map_err(|_| {
        Error::TemplateRender(format!(
            "failed to render. logs={}",
            event["logUrl"].as_str().unwrap()
        ))
    })?;
    log::debug!("message_id={} rendered: {}", message_id, rendered);

    let mut notify = settings.notifiers.get("default").unwrap();
    if let Some(Value::String(id)) = trigger_id {
        if settings.notifiers.contains_key(id) {
            notify = settings.notifiers.get(id).unwrap();
        }
    }

    let _ = notify::notify(notify, &rendered).await;

    Ok(())
}
