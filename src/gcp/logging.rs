use crate::gcp;
use crate::gcp::Error;
use reqwest;
use reqwest::header::CONTENT_TYPE;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Serialize, Debug)]
pub struct LoggingFilter {
    #[serde(rename = "resourceNames")]
    pub resource_names: Vec<String>,
    pub filter: String,
    #[serde(rename = "orderBy")]
    pub order_by: String,
    #[serde(rename = "pageSize")]
    pub page_size: u16,
}

async fn get(url: &str, bearer: &str, payload: &str) -> Result<Value, Error> {
    let client = reqwest::Client::new();
    log::debug!("url={} payload={}", url, payload);

    let response = client
        .post(url)
        .header("authorization", bearer)
        .header(CONTENT_TYPE, "application/json")
        .body(payload.to_owned())
        .send()
        .await;

    let text = match response {
        Ok(v) => v.text().await.unwrap().to_string(),
        Err(e) => {
            return Err(Error::GCPRequest(format!(
                "failed to fetch logs from GCP: {}",
                e
            )))
        }
    };

    match serde_json::from_str(&text) {
        Ok(v) => Ok(v),
        Err(e) => Err(Error::GCPRequest(format!(
            "failed to deserialize logs from GCP: {}",
            e
        ))),
    }
}

pub async fn list(request: LoggingFilter) -> Result<Vec<Value>, Error> {
    let token = gcp::auth::get_token().await;
    let bearer = format!("Bearer {}", token.unwrap().as_str());
    let base: &'static str = "https://logging.googleapis.com/v2/entries:list";
    let mut results: Vec<Value> = Vec::new();
    let mut url = base.to_string();
    let payload = match serde_json::to_string(&request) {
        Ok(v) => v,
        Err(e) => {
            return Err(Error::Serialize(format!(
                "failed to serialize the request: {}",
                e
            )))
        }
    };

    loop {
        let data = get(&url, &bearer, &payload).await?;
        log::debug!("response={}", data);

        if let Some(entries) = data["entries"].as_array() {
            results.extend(entries.iter().cloned());
        }

        if let Some(next_page_token) = data.get("nextPageToken") {
            url = format!("{}?pageToken={}", base, next_page_token.as_str().unwrap());
        } else {
            break;
        }
    }

    Ok(results)
}
