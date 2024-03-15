use std::collections::HashMap;

use currency::Currency;
use reqwest;
use serde::{Deserialize, Serialize};

mod constants;
pub mod currency;
mod pkg;

use constants::BASE_API_URL;
// use pkg::EasepayClient;

/// The Easepay struct is the main struct that holds the public and private keys
#[derive(Debug, Serialize, Deserialize)]

pub struct Easepay {
    /// Unique identifier of the Store
    pub public_key: String,
    /// The store's authorization passkey
    pub secret_key: String,
}

#[allow(dead_code)]
impl Easepay {
    ///Constructs a new easpay client
    ///
    ///Accepts the Store's Public and Secret Key provided on the  Merchant's dashboard
    ///
    pub fn new(public_key: &str, secret_key: &str) -> Self {
        Self {
            public_key: public_key.to_string(),
            secret_key: secret_key.to_string(),
        }
    }

    /// the client makes the request to the server, essentiall it wraps around the reqwest client, adding the public and private keys to the request query
    fn build_url(&self, request_path: &str) -> String {
        format!(
            "{base_url}/{end_point}?public_key={public_key}&={secret_key}",
            base_url = &BASE_API_URL,
            end_point = request_path.trim(),
            public_key = self.public_key,
            secret_key = self.secret_key
        )
    }

    /// build a url with query params
    fn build_url_with_query(&self, request_path: &str, query: HashMap<&str, &str>) -> String {
        let mut query_builder = String::new();

        for (key, value) in query {
            let entry = format!("{key}={value}&");
            query_builder.push_str(&entry);
        }

        format!(
            "{base_url}/{end_point}?{queries}public_key={public_key}&={secret_key}",
            base_url = &BASE_API_URL,
            end_point = request_path.trim(),
            public_key = self.public_key,
            secret_key = self.secret_key,
            queries = query_builder
        )
    }

    /// See if the service is active
    /// returns  Ok("API Gateway is up and running")
    pub async fn health(&self) -> Result<String, reqwest::Error> {
        let resp = reqwest::get(self.build_url("health")).await?.text().await?;

        Ok(resp)
    }

    /// convert value in one currency to bitcoin
    pub async fn convert_to_bitcoin(&self, amount: f64, currency: Currency) {
        let path = self.build_url_with_query(
            "convert",
            HashMap::from([
                ("currency", currency.to_string().as_str()),
                ("amount", &amount.to_string()),
            ]),
        );

        println!("{path}");
        // let resp = reqwest::post(path)
    }

    /// convert from bitcoin to fiat
    pub async fn convert_from_bitcoin(&self, amount: f64, currency: Currency) {
        let path = self.build_url_with_query(
            "convert",
            HashMap::from([
                ("currency", currency.to_string().as_str()),
                ("amount", &amount.to_string()),
            ]),
        );

        println!("{path}");
        // let resp = reqwest::post(path)
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

    async fn get_wallet_balance() {}

    async fn get_store_information() {}

    async fn get_transaction_history() {}
}
