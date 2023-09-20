use crate::Error;
use reqwest;
use reqwest::header::CONTENT_TYPE;

pub async fn notify(webhook: &str, payload: &str) -> Result<(), Error> {
    let client = reqwest::Client::new();
    log::debug!("webhook={} payload={}", webhook, payload);

    match client
        .post(webhook)
        .body(payload.to_owned())
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await
    {
        Ok(response) => {
            if response.status() == 200 {
                log::info!("Notification sent to slack. response={}", response.status());
                Ok(())
            } else {
                Err(Error::SlackNotify(format!(
                    "Failed to notify with Slack: {}",
                    response.text().await.unwrap()
                )))
            }
        }
        Err(e) => Err(Error::SlackNotify(format!(
            "Failed to notify with Slack: {}",
            e
        ))),
    }
}
