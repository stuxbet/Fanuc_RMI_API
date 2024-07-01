use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FrcSetUFrameUTool {
    #[serde(rename = "UFrameNumber")]
    u_frame_number: i8,    
    #[serde(rename = "UToolNumber")]
    u_tool_number: i8,
    #[serde(rename = "Group")]
    group: u8,
}


impl FrcSetUFrameUTool{
    fn new(groupentered: Option<u8>, tool:i8, frame:i8) -> Self {
        Self {
            group: match groupentered {
                Some(gm) => gm,
                None => 1
            },
            u_frame_number: frame,
            u_tool_number: tool,
        }

    }
}

// impl Default for FrcSetUFrameUTool {
//     fn default() -> Self {
//         FrcSetUFrameUTool::new(Some(1))
//     }
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct FrcSetUFrameUToolResponse { 
    // #[serde(rename = "Command")]
    // pub command: Command,    
    #[serde(rename = "ErrorID")]
    pub error_id: u32,
    #[serde(rename = "Group")]
    pub group: u16,

}