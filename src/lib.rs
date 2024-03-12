use reqwest::Client;
use serde::{Deserialize, Serialize};

mod constants;
pub mod currency;
mod pkg;

use constants::API_URL;
use pkg::EasepayClient;

/// The Easepay struct is the main struct that holds the public and private keys
#[derive(Debug, Serialize, Deserialize)]

pub struct Easepay {
    pub api_key: String,
    pub api_secret: String,
}

impl Easepay {
    fn new(api_key: &str, api_secret: &str) -> Easepay {
        Easepay {
            api_key: api_key.to_string(),
            api_secret: api_secret.to_string(),
        }
    }

    /// the client makes the request to the server, essentiall it wraps around the reqwest client, adding the public and private keys to the request query
    pub async fn client(&self) -> Client {
        todo!("implement the client function")
    }
}

impl EasepayClient for Easepay {
    async fn health(&self) -> Result<(), reqwest::Error> {
        self.client()
            .await
            .get(&format!("{}/health", API_URL))
            .send()
            .await?;
        todo!()
    }

    async fn create_payment(&self) -> Result<(), reqwest::Error> {
        todo!()
    }

    async fn payment_history(&self) -> Result<(), reqwest::Error> {
        todo!()
    }

    async fn store_information(&self) -> Result<(), reqwest::Error> {
        todo!()
    }
}
