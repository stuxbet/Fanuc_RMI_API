use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct FrcContinueResponse { 
    #[serde(rename = "ErrorID")]
    pub error_id: u32,

}