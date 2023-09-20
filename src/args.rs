use clap::Arg;
use clap::Command;

pub fn parse_options() -> Command<'static> {
    Command::new("Cloud Build Notifier")
        .version(env!("CARGO_PKG_VERSION"))
        .about("\n    Alerts when the state of a Cloud Build job changes.")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("config.yaml")
                .help("Load custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .help("Increases logging verbosity")
                .takes_value(false),
        )
        .arg(
            Arg::new("nack")
                .long("nack")
                .help("Does not ack the message after processing")
                .takes_value(false),
        )
        .arg(
            Arg::new("validate")
                .long("validate")
                .help("Renders and validates a template, printing the output")
                .takes_value(true)
                .requires("event-file"),
        )
        .arg(
            Arg::new("event-file")
                .long("event-file")
                .help("Event to use to render a tempalte. Used with --validate")
                .takes_value(true)
                .requires("validate"),
        )
        .arg(
            Arg::new("log-file")
                .long("log-file")
                .help("Log file to use when rendering a tempalte. Used with --validate")
                .takes_value(true)
                .requires("validate"),
        )
}
