use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FrcWaitTime {
    #[serde(rename = "SequenceID")]
    sequence_id: i32,
    #[serde(rename = "Time")]
    time: f32,

}

 
impl FrcWaitTime{
    #[allow(unused)]
    fn new(seq:i32, time:f32) -> Self {
        Self {
            sequence_id: seq,
            time: time,
        }

    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FrcWaitTimeResponse { 
    #[serde(rename = "ErrorID")]
    pub error_id: u32,
    #[serde(rename = "SequenceID")]
    sequence_id: i32,
}