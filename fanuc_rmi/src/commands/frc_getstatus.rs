use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FrcGetStatusResponse { 
    // #[serde(rename = "Command")]
    // pub command: Command,    
    #[serde(rename = "ErrorID")]
    pub error_id: u32,
    #[serde(rename = "ServoReady")]
    pub servo_ready: i8,
    #[serde(rename = "TPMode")]
    pub tp_mode: i8,
    #[serde(rename = "RMIMotionStatus")]
    pub rmi_motion_status: i8,
    #[serde(rename = "ProgramStatus")]
    pub program_status: i8,
    #[serde(rename = "SingleStepMode")]
    pub single_step_mode: i8,
    #[serde(rename = "NumberUTool")]
    pub number_utool: i8,
    #[serde(rename = "NumberUFrame")]
    pub number_uframe: i8,
}