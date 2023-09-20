use crate::gcp;
use reqwest;
use reqwest::header::CONTENT_TYPE;
use serde_json::Value;
use base64::Engine as _;
use base64::engine::general_purpose;

fn base64_decode(value: &str) -> Result<String, gcp::Error> {
    match general_purpose::STANDARD.decode(value) {
        Ok(decoded) => Ok(String::from_utf8(decoded).unwrap()),
        Err(_) => Err(gcp::Error::GCPSecretManager(String::from(
            "failed to decode results",
        ))),
    }
}

pub async fn get(name: &str) -> Result<String, gcp::Error> {
    let parts: Vec<&str> = name.split('/').collect();
    let url = format!(
        "https://secretmanager.googleapis.com/v1/projects/{}/secrets/{}/versions/{}:access",
        parts[1], parts[3], parts[5]
    );
    let client = reqwest::Client::new();
    let token = gcp::auth::get_token().await;
    let auth_header = format!("Bearer {}", token.unwrap().as_str());

    let response = client
        .get(url)
        .header(CONTENT_TYPE, "application/json")
        .header("authorization", auth_header)
        .send()
        .await;

    let payload = match response {
        Ok(v) => v.text().await.unwrap().to_string().clone(),
        Err(e) => {
            return Err(gcp::Error::GCPSecretManager(format!(
                "failed to parse results from secret manager: {}",
                e
            )))
        }
    };

    let data: Value = serde_json::from_str(&payload).unwrap();

    match data["payload"]["data"].as_str() {
        Some(v) => match base64_decode(v) {
            Ok(v) => Ok(v),
            Err(e) => Err(e),
        },
        None => Err(gcp::Error::GCPSecretManager(String::from(
            "failed to parse results from secret manager",
        ))),
    }
}
