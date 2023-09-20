use google_cloud_gax::grpc::Status;
use google_cloud_pubsub::client::{Client, ClientConfig};
use google_cloud_pubsub::subscription::MessageStream;

pub async fn get_consumer(subscription_id: &str) -> Result<MessageStream, Status> {
    let config = ClientConfig::default().with_auth().await.unwrap();
    let client = Client::new(config).await.unwrap();
    let subscription = client.subscription(subscription_id);
    let stream = subscription.subscribe(None).await?;
    Ok(stream)
}
