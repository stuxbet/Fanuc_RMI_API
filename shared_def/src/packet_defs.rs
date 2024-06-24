use std::error::Error;

pub use serde::{Deserialize,Serialize};
use serde_json;



// #[derive(Serialize, Deserialize)]
// pub enum Packet {
//     Communication,
//     Command,
//     Instruction,
//     Initialize,
//     Abort,
//     Disconnect
// }

// #[derive(Serialize, Deserialize)]
// enum Communication{
//     FRC_Connect,
//     FRC_Disconnect,
// }
// #[derive(Serialize, Deserialize)]
// enum Command{

//     //intialize can specify a groupmask but defaults to 1
//     FRC_Initialize,
//     FRC_ReadPositionRegister,
//     FRC_WritePositionRegister,
//     FRC_SetOverride,
//     FRC_GetStatus,
//     FRC_GetUFrameUTool,
//     FRC_ReadUToolData,
//     FRC_WriteUToolData,
//     FRC_ReadUFrameData,
//     FRC_WriteUFrameDat,
//     FRC_ReadJointAngles,
//     FRC_ReadCartesianPosition,
//     FRC_Reset,
// }

// enum FRC_Packet{
//     CommandPacket: Packet(Command),
//     CommunicationPacket: Packet(Communication)
// }

// impl Packet {
//     // Serialize the packet into the required string format
//     pub fn to_string(&self) -> Result<String, Error> {
//         match serde_json::to_string(&self) {
//             Ok(str) => Ok(str),
//             Err(err) => Error::new("Could not serialize")
//         }
//         // let json_packet = match self {
//         //     Packet::Communication => json!({"Communication": "FRC_Connect"}),
//         //     Packet::Command => json!({"Command": "FRC_Disconnect"}),
//         //     Packet::Initialize => json!({"Command": "FRC_Initialize"}),
//         //     Packet::Instruction => json!({"Instruction": "FRC_Disconnect"}),
//         //     Packet::Abort => json!({"Command": "FRC_Abort"}),
//         //     Packet::Disconnect => json!({"Communication": "FRC_Disconnect"}),
//         // };

//         // format!("{}\r\n", json_packet.to_string())
//     }
// }

#[derive(Serialize, Deserialize)]
pub enum CommunicationPacket {
    FRC_Connect,
    FRC_Disconnect,
}

#[derive(Serialize, Deserialize)]
pub enum CommandPacket {
    FRC_Initialize { group_mask: u8 },
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
pub struct Configuration {
    utool_number: u8,
    uframe_number: u8,
    front: u8,
    up: u8,
    left: u8,
    flip: u8,
    turn4: u8,
    turn5: u8,
    turn6: u8,
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

// fn main() {
//     // Example of creating a packet
//     let packet = Packet::Command(CommandPacket::Initialize { group_mask: 1 });

//     // Serialize packet to JSON string
//     let serialized_packet = to_string(&packet).unwrap();
//     println!("{}", serialized_packet);

//     // Send `serialized_packet` over a TCP/IP socket as needed
// }
