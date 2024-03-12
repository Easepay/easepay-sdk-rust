 # Easepay Rust SDK

- [Description](#description)
- [Getting Started](#getting-started)
- [Documentation](#documentation)
- [Version History](#version-history)
- [Usage](#usage)
- [License](#license)


## Description
Easepay NodeJs SDK is a library that allows you to easily interact with the Easepay API. It provides a simple and easy-to-use interface for the Easepay API. The SDK is built on top of the Axios library and provides a simple interface for making requests to the Easepay API.

## Getting Started
1.  Create an account on [Easepay](https://easepay.io)
2.  Create an API key on the [API settings page](https://easepay.io/settings/api)
3.  Install the SDK using npm
  ```bash
  cargo add @easepay/sdk
  ```
4. Use the SDK to interact with the Easepay API
```rust
use easepay::{Easepay, Currency};
use std::env;

#[tokio::main]
async fn main() {
    let easepay =
        Easepay::new(env::var("EASEPAY_API_KEY").unwrap(),
        env::var("EASEPAY_API_SECRET").unwrap());

    let response = easepay.create_payment(
        31550,
        Currency::NGN,
        "https://example.com/redirect",
        "https://example.com/callback",
        Some("50193eb8-40ba-4280-b6e4-7077f7f5626e"),
        Some("6614c062-cbf5-47a0-911e-8e3d30e1c900"),
    ).await;

    match response {
        Ok(payment) => {
            println!("Payment created: {:?}", payment);
        }
        Err(error) => {
            println!("Error creating payment: {:?}", error);
        }
    }
}
```

For test and development purposes, you can use the test API key and secret provided in the [API settings page](https://easepay.io/settings/api)


## Usage 
The SDK provides a simple interface for making requests to the Easepay API. The following methods are available:
- `createPayment`: Create a new payment
- `getPayment`: Get a payment by ID
- `getPayments`: Get a list of payments
- `createPayout`: Create a new payout
- `getPayout`: Get a payout by ID
- `getPayouts`: Get a list of payouts


## Building locally 
- Clone the repository
```bash
git clone https://github.com/easepay/easepay-sdk-rust.git
```
- Install dependencies
```bash
cargo build
```
- Run the example
```bash
cargo run --example create_payment

```
- Run tests
```bash
cargo test
```

## License

This project is licensed under the MIT License - see the [Licesnse](/LICENSE) file for details

