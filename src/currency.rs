use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Currency {
    NGN,
    USD,
}

impl Currency {
    pub fn to_string(&self) -> String {
        match self {
            Currency::NGN => "NGN".to_lowercase(),
            Currency::USD => "USD".to_lowercase(),
        }
    }
}
