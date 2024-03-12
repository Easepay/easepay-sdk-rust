/// the easepay client is a typed client that makes requests to the easepay server
pub trait EasepayClient{
pub fn new(api_key: &str, api_secret: &str) -> Self;

/// the health function checks if the server is up and running
pub async fn health(&self) -> Result<(), reqwest::Error>;

/// the get_balance function returns the balance of the account
pub async fn create_payment(&self) -> Result<(), reqwest::Error>;

/// the get_balance function returns the balance of the account
pub async fn payment_history(&self) -> Result<(), reqwest::Error>;


/// the get_balance function returns the balance of the account
pub async fn store_information(&self) -> Result<(), reqwest::Error>;
}