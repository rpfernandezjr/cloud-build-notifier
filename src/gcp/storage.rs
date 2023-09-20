use crate::gcp;
use crate::gcp::Error;
use google_cloud_storage::http::objects::download::Range;
use google_cloud_storage::http::objects::get::GetObjectRequest;
use reqwest;

pub async fn download_object(bucket: String, object: String) -> Result<String, Error> {
    let config = google_cloud_storage::client::ClientConfig::default()
        .with_auth()
        .await
        .unwrap();
    let client = google_cloud_storage::client::Client::new(config);

    match client
        .download_object(
            &GetObjectRequest {
                bucket,
                object,
                ..Default::default()
            },
            &Range::default(),
        )
        .await
    {
        Ok(bytes) => Ok(String::from_utf8_lossy(&bytes).to_string()),
        Err(e) => Err(Error::GCPAuthToken(e.to_string())),
    }
}

pub async fn download_object_2(bucket: String, object: String) -> Result<String, Error> {
    let token = gcp::auth::get_token().await;
    let auth_header = format!("Bearer {}", token.unwrap().as_str());
    let url = format!(
        "https://storage.googleapis.com/storage/v1/b/{}/o/{}?alt=media",
        bucket, object
    );

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("authorization", auth_header)
        .send()
        .await;

    match response {
        Ok(v) => Ok(v.text().await.unwrap().to_string()),
        Err(e) => Err(Error::GCPSecretManager(format!(
            "failed to parse results from secret manager: {}",
            e
        ))),
    }
}
