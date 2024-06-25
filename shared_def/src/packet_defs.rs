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


#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event_type", rename_all = "snake_case")]
enum Event {
    Login {
        success: bool,
    },
    Logout {
        reason: String,
    },
    Purchase {
        amount: f64,
    },
}
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


fn move_to() -> String{
    let packet = MotionPacket {
        Instruction: "FRC_LinearMotion".to_string(),
        SequenceID: 1,
        Configuration: Configuration {
            UToolNumber: 1,
            UFrameNumber: 1,
            Front: 0,
            Up: 1,
            Left: 0,
            Flip: 0,
            Turn4: 0,
            Turn5: 0,
            Turn6: 0,
        },
        Position: Position {
            X: 500.0,
            Y: 0.0,
            Z: 300.0,
            W: 0.0,
            P: 0.0,
            R: 0.0,
        },
        SpeedType: SpeedType::mmSec,
        Speed: 100,
        TermType: TermType::FINE,
    };

    // Serialize to JSON string
    // serde_json::to_string(&packet).unwrap()

    match serde_json::to_string(&packet) {
        Ok(json_packet) => {
            println!("{}", json_packet);
            json_packet
        }
        Err(e) => {
            eprintln!("Error serializing packet: {}", e);
            e.to_string()
        }
    }

}

#[derive(Serialize, Deserialize, Debug)]
enum TermType {
    FINE,
    CNT(u8), // CNT with a value from 1 to 100
    CR(u8),  // CR with a value from 1 to 100
}

#[derive(Serialize, Deserialize, Debug)]
enum SpeedType {
    mmSec,
    InchMin,
    Time,
    mSec,
}

#[derive(Serialize, Deserialize, Debug)]
struct Configuration {
    UToolNumber: u8,
    UFrameNumber: u8,
    Front: u8,
    Up: u8,
    Left: u8,
    Flip: u8,
    Turn4: u8,
    Turn5: u8,
    Turn6: u8,
}

#[derive(Serialize, Deserialize, Debug)]
struct Position {
    X: f64,
    Y: f64,
    Z: f64,
    W: f64,
    P: f64,
    R: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct MotionPacket {
    Instruction: String,
    SequenceID: u32,
    Configuration: Configuration,
    Position: Position,
    SpeedType: SpeedType,
    Speed: u16,
    TermType: TermType,
}
