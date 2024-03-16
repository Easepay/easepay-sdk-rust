use easepay_sdk_rust::{currency::Currency, Easepay};

#[tokio::main]
async fn main() {
    //instantiate the SDK
    let public_key: String = String::from("");
    let secret_key: String = String::from("");

    let easepay = Easepay::new(&public_key, &secret_key);

    // convert from Naira to Bitcoin
    let resp = easepay.convert_to_bitcoin(30056.9, Currency::NGN).await;
    println!("{:?}", resp);

    // convert from Bitcoin to naira
    let resp = easepay
        .convert_from_bitcoin(0.000000234, Currency::NGN)
        .await;
    println!("{:?}", resp);
}
