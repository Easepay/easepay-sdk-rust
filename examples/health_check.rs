use easepay_sdk_rust::Easepay;

#[tokio::main]
async fn main() {
    let public_key: String = String::from("pk_test_8WZvMq0PRc1b9VMAdRNVmhYk5PrLLqkE8yDUSrvTP5Q");
    let secret_key: String = String::from("sk_test_VDq9cNa8zDaPXrTN8ZmFSeJpfZ9O0FIqfJDGxcYxb1I=");

    let easepay = Easepay::new(&public_key, &secret_key);
    let resp = easepay.health().await;

    println!("{:?}", resp);
}
