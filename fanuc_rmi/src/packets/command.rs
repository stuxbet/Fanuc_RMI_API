use serde::{Serialize, Deserialize};

use crate::{FrameData, Configuration, Position};
use super::Packet;
use crate::commands::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "Command")]
pub enum Command {
    #[serde(rename = "FRC_Initialize")]
    FrcInitialize(FrcInitialize),
    #[serde(rename = "FRC_Abort")]
    FrcAbort,
    #[serde(rename = "FRC_Pause")]
    FrcPause,
    Continue,
    Reset,
    ReadError { count: u8 },
    SetUFrameUTool { uframe_number: u8, utool_number: u8, group: Option<u8> },
    GetStatus,
    ReadUFrameData { frame_number: u8, group: Option<u8> },
    WriteUFrameData { frame_number: u8, frame: FrameData, group: Option<u8> },
    ReadUToolData { tool_number: u8, group: Option<u8> },
    WriteUToolData { tool_number: u8, frame: FrameData, group: Option<u8> },
    ReadDIN { port_number: u16 },
    WriteDOUT { port_number: u16, port_value: String },
    ReadCartesianPosition { group: Option<u8> },
    ReadJointAngles { group: Option<u8> },
    SetOverRide { value: u8 },
    GetUFrameUTool { group: Option<u8> },
    ReadPositionRegister { register_number: u16, group: Option<u8> },
    WritePositionRegister { register_number: u16, configuration: Configuration, position: Position, group: Option<u8> },
    ReadTCPSpeed,
}
#[serde(tag = "Command")]
#[derive(Serialize, Deserialize, Debug)]
pub enum CommandResponse {
    #[serde(rename = "FRC_Initialize")]
    FrcInitialize(FrcInitializeResponse),
    #[serde(rename = "FRC_Abort")]
    FrcAbort,
    #[serde(rename = "FRC_Pause")]
    FrcPause,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FrcInitializeResponse { 
    // #[serde(rename = "Command")]
    // pub command: Command,    
    #[serde(rename = "ErrorID")]
    pub error_id: u32,
    #[serde(rename = "GroupMask")]
    pub group_mask: u16,

}

impl Packet for Command{}
