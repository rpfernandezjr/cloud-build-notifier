use cloud_build_notifier::args;
use cloud_build_notifier::config;
use cloud_build_notifier::event;
use cloud_build_notifier::gcp;
use cloud_build_notifier::notify;
use cloud_build_notifier::validate;
use cloud_build_notifier::Error::Deserialize;
use cloud_build_notifier::Error::EventParsing;
use cloud_build_notifier::Error::SlackNotify;
use cloud_build_notifier::Error::TemplateNotSet;
use cloud_build_notifier::Error::TemplateRender;
use cloud_build_notifier::Settings;
use env_logger::Builder;
use env_logger::Env;
use futures_util::StreamExt;
use google_cloud_pubsub::subscription::MessageStream;
use std::process::exit;

async fn run(
    mut consumer: MessageStream,
    settings: &Settings,
    nack: bool,
) {
    log::info!("listening for messages on {}", settings.config.input.subscription_id);

    while let Some(message) = consumer.next().await {
        let id = &message.ack_id().to_string()[0..10];
        let bytes = &message.message.data;
        let data = String::from_utf8_lossy(bytes).to_string();
        log::debug!("message_id={} data={}", id, data);

        if let Err(error) = event::process(id, data, settings).await {
            match error {
                Deserialize(e) | TemplateRender(e) | SlackNotify(e) | EventParsing(e) => {
                    log::error!("message_id={} error={}", id, e);
                }
                TemplateNotSet(e) => {
                    log::info!("message_id={} msg={}", id, e);
                }
            }
        } else {
            log::info!("message_id={} has been processed", id);
        }

        if !nack {
            let _ = message.ack().await;
        }
    }
}

#[tokio::main]
async fn main() {
    let options = args::parse_options().get_matches();

    let config_file = match options.occurrences_of("config") {
        0 => String::from("config.yaml"),
        _ => options.value_of("config").unwrap().to_string(),
    };

    let log_level = match options.is_present("debug") {
        true => log::LevelFilter::Debug,
        false => log::LevelFilter::Info,
    };

    Builder::from_env(Env::default().filter("cloud_build_notifier"))
        .filter(Some("cloud_build_notifier"), log_level)
        .init();

    let config: config::Config = match config::Config::load(&config_file) {
        Ok(config) => config,
        Err(e) => {
            log::error!("Failed to load config file={}: {}", config_file, e);
            exit(1);
        }
    };

    let project = gcp::project::get(&config.input.project).await;

    let notifiers = notify::load_outputs(&config.output, &config.triggers.custom).await;

    let settings = Settings {
        config,
        notifiers,
        project,
    };
    log::debug!("{:?}", settings);

    if options.is_present("validate") {
        let template_key = options
            .value_of("validate")
            .ok_or_else(|| {
                log::error!("template name is needed");
                exit(1);
            })
            .unwrap();

        let event_file = options
            .value_of("event-file")
            .ok_or_else(|| {
                log::error!("file containing event is needed");
                exit(1);
            })
            .unwrap();

        let log_file = options.value_of("log-file").map(|v| v.to_string());

        validate::template(&settings, template_key, event_file, log_file);
    } else {
        let nack = options.is_present("nack");

        let consumer = gcp::pubsub::get_consumer(&settings.config.input.subscription_id)
            .await
            .unwrap();

        run(consumer, &settings, nack).await;
    }

}
