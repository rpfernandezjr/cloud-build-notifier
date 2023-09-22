use crate::gcp;
use serde_json::Value;

async fn get_legacy_logs(project_number: &str, build_id: &str) -> Option<String> {
    log::info!("downloading logs from GCS {}/{}", project_number, build_id);

    let object = format!("log-{}.txt", build_id);
    let bucket = format!("{}.cloudbuild-logs.googleusercontent.com", project_number);

    match gcp::storage::download_object(bucket.clone(), object.clone()).await {
        Ok(log) => Some(log),
        Err(e) => {
            log::warn!(
                "failed to fetch legacy logs from {}/{}: {:?}",
                bucket,
                object,
                e
            );
            None
        }
    }
}

async fn get_cloud_logging_logs(project_name: &str, build_id: &str) -> Option<String> {
    log::info!("dowloading logs from cloud logging for {}/{}",project_name, build_id);

    let filter = format!(
        "resource.type=build AND logName=projects/{}/logs/cloudbuild AND resource.labels.build_id={}",
        project_name,
        build_id,
    );

    let query = gcp::logging::LoggingFilter {
        resource_names: vec![format!("projects/{}", project_name)],
        filter,
        order_by: String::from("timestamp asc"),
        page_size: 100,
    };
    log::debug!("cloud logging query: {:?}", query);

    match gcp::logging::list(query).await {
        Ok(logs) => {
            let mut log_file = String::new();

            for entry in &logs {
                if let Some(text) = entry["textPayload"].as_str() {
                    log_file.push_str(text);
                    log_file.push('\n');
                }
            }
            Some(log_file)
        }
        Err(err) => match err {
            gcp::Error::GCPAuthToken(e)
            | gcp::Error::GCPSecretManager(e)
            | gcp::Error::GCPRequest(e)
            | gcp::Error::Serialize(e) => {
                log::warn!("{}", e);
                None
            }
        },
    }
}

pub async fn get_logs(
    event: &Value,
    build_id: &str,
    project: &gcp::project::Project,
) -> Option<String> {
    let mut use_legacy = true;

    // Lets see if the logs are located int he legacy storage
    // our cloud logging. `options.logging` tells us this.
    if let Some(options) = event.get("options") {
        if let Some(logging) = options.get("logging") {
            if logging.as_str().unwrap() == "CLOUD_LOGGING_ONLY" {
                use_legacy = false;
            }
        }
    }

    match use_legacy {
        true => get_legacy_logs(&project.id, build_id).await,
        false => get_cloud_logging_logs(&project.name, build_id).await,
    }
}
