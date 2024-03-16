use easepay_sdk_rust::{currency::Currency, Easepay};

#[tokio::main]
async fn main() {
    //instantiate the SDK
    let public_key: String = String::from("pk_test_8WZvMq0PRc1b9VMAdRNVmhYk5PrLLqkE8yDUSrvTP5Q");
    let secret_key: String = String::from("sk_test_VDq9cNa8zDaPXrTN8ZmFSeJpfZ9O0FIqfJDGxcYxb1I=");

    let easepay = Easepay::new(&public_key, &secret_key);

    // convert from Naira to Bitcoin
    let resp = easepay.convert_to_bitcoin(30056.9, Currency::NGN).await;
    println!("{:?}", resp);

    // convert from Bitcoin to naira
    let resp = easepay.convert_from_bitcoin(0.000000234, Currency::NGN).await;
    println!("{:?}", resp);
}
