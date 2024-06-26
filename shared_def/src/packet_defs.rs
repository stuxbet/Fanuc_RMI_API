use serde_json;

// use serde::{Deserialize, Serialize};
use serde_derive::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub enum CommunicationPacket {
    FRC_Connect,
    FRC_Disconnect,
}

#[derive(Serialize, Deserialize)]
pub enum CommandPacket {
    FRC_Initialize,

    FRC_Abort,
    FRC_Pause,
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
    WritePositionRegister { register_number: u16, configuration: Configuration, position: PositionData, group: Option<u8> },
    ReadTCPSpeed,
}

#[derive(Serialize, Deserialize)]
pub struct FrameData {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    p: f32,
    r: f32,
}

#[derive(Serialize, Deserialize)]
pub struct PositionData {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    p: f32,
    r: f32,
    ext1: Option<f32>,
    ext2: Option<f32>,
    ext3: Option<f32>,
}

#[derive(Serialize, Deserialize)]
pub enum InstructionPacket {
    WaitDIN { sequence_id: u32, port_number: u16, port_value: String },
    SetUFrame { sequence_id: u32, frame_number: u8 },
    SetUTool { sequence_id: u32, tool_number: u8 },
    WaitTime { sequence_id: u32, time: f32 },
    SetPayLoad { sequence_id: u32, schedule_number: u8 },
    Call { sequence_id: u32, program_name: String },
    LinearMotion { sequence_id: u32, configuration: Configuration, position: PositionData, speed_type: String, speed: u16, term_type: String, term_value: u8 },
    // Add other instruction types as needed
}

#[derive(Serialize, Deserialize)]
pub enum Packet {
    Communication(CommunicationPacket),
    Command(CommandPacket),
    Instruction(InstructionPacket),
}
#[derive(Serialize, Deserialize)]

pub struct Attributes {
    #[serde(flatten)]
    extra: serde_json::Value,
}
/////////////////////////////////////////////////////////////////////////////////////////////
//new definition system


// #[serde(flatten)]
#[derive(Serialize, Deserialize)]
pub struct CommandPacketStruct {
    pub CommandPacket: CommandPacket,
    pub group: Option<u8>
}

#[derive(Serialize, Deserialize)]
pub struct CommunicationPacketStruct {
    pub Communication: CommunicationPacket,
}


pub struct optionalInfo{
    pub group: Option<u8>

}


#[derive(Serialize, Deserialize, Debug)]
pub enum TermType {
    FINE,
    CNT(u8), // CNT with a value from 1 to 100
    CR(u8),  // CR with a value from 1 to 100
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SpeedType {
    mmSec,
    InchMin,
    Time,
    mSec,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub UToolNumber: u8,
    pub UFrameNumber: u8,
    pub Front: u8,
    pub Up: u8,
    pub Left: u8,
    pub Flip: u8,
    pub Turn4: u8,
    pub Turn5: u8,
    pub Turn6: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Position {
    pub X: f64,
    pub Y: f64,
    pub Z: f64,
    pub W: f64,
    pub P: f64,
    pub R: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MotionPacket {
    pub Instruction: String,
    pub SequenceID: u32,
    pub Configuration: Configuration,
    pub Position: Position,
    pub SpeedType: SpeedType,
    pub Speed: u16,
    pub TermType: TermType,
}

pub struct SimplePacket {
    command: String,
    groupmask: u8,
}