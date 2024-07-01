use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct FrcPauseResponse { 
    #[serde(rename = "ErrorID")]
    pub error_id: u32,

}