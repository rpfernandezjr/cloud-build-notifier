# cloud-build-notifier
Cloud Build Notifier is a tool that sends alerts when the state of a Google Cloud Build job changes.

# Installation
To use this app, make sure you have [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed.

## Configuration

Before using Cloud Build Notifier, you need to configure it by following these steps:

1. Create a Cloud Pub/Sub topic on Google Cloud Platform:
```shell
gcloud pubsub topics create cloud-builds
```

2. Create a subscription for the topic
```shell
gcloud pubsub subscriptions create subscriber-id \
   --topic=cloud-builds
```

## Notification Types
Currently just Slack is supported.


### Slack Integration

To receive notifications on Slack, you'll need to configure the Slack integration in your app's YAML configuration file. Here's an example:

```yaml
output:
  type: slack
  params:
    webhook: https://hooks.slack.com/services/YYYYYYYYY/XXXXXXXXXXXXXXXXXXXXXXXXXX
    secret_manager: projects/1234567890/secrets/MY_SECRET/versions/latest
```
Slack requires a webhook URL. You can provide it directly with the webhook key or use the secret_manager key to fetch the webhook from Google Cloud Platform. Only one of these two parameters is required.

## Templates

This is using the Rust [Tera](https://keats.github.io/tera/docs/) template language

There are 3 keys within the template context that can be used for rendering within the template.

### Event
The `event` key within the template is the full event that is published to Pub/Sub.
```json
{
    "id": "1234-5678-909876-5443-2100",
    "status": "SUCCESS",
    ...
}
```
To access the status field within the template, It can be done like this:

```
{{ event.status }}
```

### buildTime
`buildTime` - This key is derived from the event start and finish times. It is the total time the build took to run. The format is `{}h {}m {}s`. You can access this from the template with:

```
{{ buildTime }}
```


### log
This process downloads the logs from the cloud build run and loads it into the `log` key within the template context.

```
{{ log }}
```

Note: There are some instances where the logs fail to download from GCS, I still haven't tracked down why, but just be aware if your template is dependent on it


## Local Development

Follow these steps to build and run Cloud Build Notifier locally:

Build
```shell
cargo build
```

Run
```shell
cargo run -- -c /path/to/config.yaml
```

## Docker

You can also build and run Cloud Build Notifier using Docker. Here's how:

Build
```shell
docker build -t cloud-build-notifier:latest . --target app
```
