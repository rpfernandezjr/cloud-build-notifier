use crate::gcp;
use reqwest;
use reqwest::header::CONTENT_TYPE;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ProjectResponse {
    pub name: String,
    pub parent: String,
    pub state: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "createTime")]
    pub create_time: String,
    #[serde(rename = "updateTime")]
    pub update_time: String,
    pub etag: String,
}

#[derive(Deserialize, Debug)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub parent: String,
    pub state: String,
}

pub async fn get(name: &str) -> Project {
    let token = gcp::auth::get_token().await;
    let bearer = format!("Bearer {}", token.unwrap().as_str());
    let url = format!(
        "https://cloudresourcemanager.googleapis.com/v3/projects/{}",
        name
    );
    let client = reqwest::Client::new();

    let response = client
        .get(url)
        .header("authorization", bearer)
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await;

    let text = match response {
        Ok(v) => v.text().await.unwrap().to_string(),
        Err(_) => todo!(),
    };

    let data: ProjectResponse = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(_) => todo!(),
    };

    let parts: Vec<&str> = data.name.split('/').collect();

    Project {
        id: parts[1].to_string(),
        name: data.display_name,
        parent: data.parent,
        state: data.state,
    }
}
