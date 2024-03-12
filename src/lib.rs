use serde::{Deserialize, Serialize};
use reqwest::Client;

mod pkg;
mod constants;

use constants::API_URL;
use pkg::EasepayClient;

/// The Easepay struct is the main struct that holds the public and private keys
#[derive(Debug, Serialize, Deserialize)]

pub struct Easepay {
    pub api_key: String,
    pub api_secret: String,
}

impl Easepay {
    pub fn new(api_key: &str, api_secret: &str) -> Easepay {
        Easepay {
            api_key: api_key.to_string(),
            api_secret: api_secret.to_string(),
        }
    }


    /// the client makes the request to the server, essentiall it wraps around the reqwest client, adding the public and private keys to the request query
    pub async fn client(&self) -> Client{
       todo!("implement the client function")
    }


    /// the health function checks if the server is up and running
    pub async fn health(&self) -> Result<(), reqwest::Error> {
        let client = self.client().await;
        let url = format!("{}{}", API_URL, "health");
        let response = client.get(&url).send().await?;
        response.error_for_status_ref()?;
        Ok(())
    }


    /// the get_balance function returns the balance of the account
    pub async fn get_balance(&self) -> Result<(), reqwest::Error> {
        let client = self.client().await;
        let url = format!("{}{}", API_URL, "balance");
        let response = client.get(&url).send().await?;
        response.error_for_status_ref()?;
        Ok(())
    }
}



