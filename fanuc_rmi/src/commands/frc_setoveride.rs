use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Debug)]
pub struct FrcSetOverride {
    #[serde(rename = "Value")]
    value: u8,
}



impl FrcSetOverride{
    #[allow(unused)]
    fn new(val: u8) -> Self {
        Self {
            value: val
        }

    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct FrcSetOverrideResponse {   
    #[serde(rename = "ErrorID")]
    error_id: u16,
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct FrcSetOverrideResponse {   
//     #[serde(rename = "ErrorID")]
//     error_id: u16,
//     #[serde(rename = "UFrameNumber")]
//     UFrameNumber: u8,
//     #[serde(rename = "UToolNumber")]
//     UToolNumber: u8,
//     #[serde(rename = "Group")]
//     group: u8,
// }