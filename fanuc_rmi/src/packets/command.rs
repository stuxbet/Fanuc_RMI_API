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

impl Packet for Command{}
