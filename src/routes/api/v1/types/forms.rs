use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContactUsInput {
    pub mobile: String,
    pub email: String,
    pub message: String,
}
