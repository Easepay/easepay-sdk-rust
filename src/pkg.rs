use serde::{Deserialize, Serialize};


/// the API response  
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

impl<T: std::fmt::Debug+ std::fmt::Display> ApiResponse<T> {
    pub fn from(success: bool, message: String, data: Option<T>) -> ApiResponse<T> {
        ApiResponse {
            success,
            message,
            data,
        }
    }
}

/// format the API response
impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for ApiResponse<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let data = match &self.data {
            Some(data) => data,
            None => todo!("impl this"),
        };

        write!(
            f,
            "success: {},\nmessage: {},\ndata: {:?}\n",
            self.success, self.message, data
        )
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Store {
    /// the store unique identiiers
     id: String,
    /// store name
    name: String,
    store_id: String,
    currency: Option<String>,
    support_url: Option<String>,
    redirect_url: Option<String>,
    is_test_mode: bool,
    kyc_information: Option<KycInformation>,
    business_information: Option<BusinessInformation>,
    owner_information: Option<OwnerInformation>,
    payout_information: Option<PayoutInformation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KycInformation {
    id: String,
    document: String,
    document_id: String,
    document_url: String,
}



#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessInformation{}


#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerInformation{}


#[derive(Debug, Serialize, Deserialize)]
pub struct PayoutInformation{}
