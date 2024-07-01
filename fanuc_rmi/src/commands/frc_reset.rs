use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct FrcResetResponse { 
    #[serde(rename = "ErrorID")]
    pub error_id: u32,
}