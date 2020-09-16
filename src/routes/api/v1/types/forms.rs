use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContactUsInput {
    pub mobile: String,
    pub email: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClientGeoLocation {
    pub country: String,
    pub region: String,
    pub city: String,
    pub pin_code: String,
}
