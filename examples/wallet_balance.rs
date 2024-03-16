use easepay_sdk_rust::Easepay;

#[tokio::main]
async fn main() {
    let public_key: String = String::from("");
    let secret_key: String = String::from("");

    let easepay = Easepay::new(&public_key, &secret_key);
    let resp = easepay.get_wallet_balance().await;

    println!("{:?}", resp);
}
