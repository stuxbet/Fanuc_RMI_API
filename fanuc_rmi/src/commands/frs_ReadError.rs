use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FrcReadError {
    Count: u8,
}



impl FrcReadError{
    fn new(count: Option<u8>) -> Self {
        let count = match count {
            Some(gm) => gm,
            None => 1
        };
        Self {
            Count: count
        }

    }
}

impl Default for FrcReadError {
    fn default() -> Self {
        FrcReadError::new(Some(1))
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct FrcReadErrorResponse {   
    #[serde(rename = "ErrorID")]
    error_id: u16,
    #[serde(rename = "Count")]
    count: u8,
    #[serde(rename = "ErrorData")]
    error_data: String
}