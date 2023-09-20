use crate::gcp::Error;
use gcp_auth::AuthenticationManager;
use gcp_auth::Token;

pub async fn get_token() -> Result<Token, Error> {
    let scopes = &["https://www.googleapis.com/auth/cloud-platform"];
    let authentication_manager = match AuthenticationManager::new().await {
        Ok(v) => v,
        Err(e) => {
            return Err(Error::GCPAuthToken(format!(
                "failed to fetch auth token from GCP: {}",
                e
            )))
        }
    };
    match authentication_manager.get_token(scopes).await {
        Ok(token) => Ok(token),
        Err(e) => Err(Error::GCPAuthToken(format!(
            "failed to fetch auth token from GCP: {}",
            e
        ))),
    }
}
