use reqwest::Client;
use crate::domain::SubscriberEmail;

#[derive(Clone)]
pub struct EmailClient {
    http_client: Client,
    base_url: String,
    sender: SubscriberEmail,
}

impl EmailClient {
    pub fn new(
        base_url: String,
        sender: SubscriberEmail,
    ) -> Self {
        let http_client = Client::builder().build().unwrap();
        Self {
            http_client,
            base_url,
            sender,
        }
    }
}