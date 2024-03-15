/// the easepay client is a typed client that makes requests to the easepay server

pub trait EasepayClient {
    /// the health function checks if the server is up and running
    async fn health(&self) -> Result<String, reqwest::Error>;

    /// the get_balance function returns the balance of the account
    async fn create_payment(&self) -> Result<(), reqwest::Error>;

    /// the get_balance function returns the balance of the account
    async fn payment_history(&self) -> Result<(), reqwest::Error>;

    /// the get_balance function returns the balance of the account
    async fn store_information(&self) -> Result<(), reqwest::Error>;
}
