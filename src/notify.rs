use crate::config::CustomTrigger;
use crate::config::Output;
use crate::gcp;
use crate::slack;
use crate::Error;
use crate::Notify;
use std::collections::HashMap;
use std::process::exit;

async fn build_output(output: &Output) -> Notify {
    match &output.r#type[..] {
        "slack" => slack(&output.params).await,
        _ =>  Notify::Off,
    }
}

async fn slack(params: &HashMap<String, String>) -> Notify {
    log::info!("loading output type for slack");
    let mut webhook = String::new();

    if params.contains_key("secret_manager") {
        let name = params.get("secret_manager").unwrap();

        match gcp::secret_manager::get(name).await {
            Ok(value) => {
                log::info!("retreived slack webhook from secret manager: {}", name);
                webhook = value;
            }
            Err(gcp::Error::GCPSecretManager(e)) => {
                log::error!("{}", e);
                exit(1);
            }
            _ => {
                log::error!("unknown error");
                exit(1);
            }
        };
    }

    if params.contains_key("webhook") {
        webhook = params.get("webhook").unwrap().to_string();
    }

    Notify::Slack(webhook)
}

pub async fn load_outputs(
    default: &Output,
    custom: &Option<Vec<CustomTrigger>>,
) -> HashMap<String, Notify> {
    let mut outputs: HashMap<String, Notify> = HashMap::new();

    let default_notify = build_output(default).await;

    outputs.insert(String::from("default"), default_notify);

    if let Some(custom_triggers) = custom {
        for trigger in custom_triggers {
            if let Some(out) = &trigger.output {
                let key = trigger.trigger_id.clone();
                let notify = build_output(out).await;
                outputs.insert(key, notify);
            }
        }
    }

    outputs
}

pub async fn notify(n: &Notify, data: &str) -> Result<(), Error> {
    match n {
        Notify::Slack(webhook) => Ok(slack::notify(webhook, data).await?),
        Notify::Off => Ok(()),
    }
}
