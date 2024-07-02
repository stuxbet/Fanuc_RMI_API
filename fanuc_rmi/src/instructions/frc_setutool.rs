use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FrcSetUTool {
    #[serde(rename = "SequenceID")]
    sequence_id: i32,
    #[serde(rename = "ToolNumber")]
    tool_number: u8,

}

 
impl FrcSetUTool{
    #[allow(unused)]
    fn new(seq:i32, tool_num:u8) -> Self {
        Self {
            sequence_id: seq,
            tool_number: tool_num,
        }

    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FrcSetUToolResponse { 
    #[serde(rename = "ErrorID")]
    pub error_id: u32,
    #[serde(rename = "SequenceID")]
    sequence_id: i32,
}