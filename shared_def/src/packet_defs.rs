use std::error::Error;

pub use serde::{Deserialize,Serialize};
use serde_json;



#[derive(Serialize, Deserialize)]
pub enum Packet {
    Communication,
    Command,
    Instruction,
    Initialize,
    Abort,
    Disconnect
}

#[derive(Serialize, Deserialize)]
enum Communication{
    FRC_Connect,
    FRC_Disconnect,
}
#[derive(Serialize, Deserialize)]
enum Command{

    //intialize can specify a groupmask but defaults to 1
    FRC_Initialize,
    FRC_ReadPositionRegister,
    FRC_WritePositionRegister,
    FRC_SetOverride,
    FRC_GetStatus,
    FRC_GetUFrameUTool,
    FRC_ReadUToolData,
    FRC_WriteUToolData,
    FRC_ReadUFrameData,
    FRC_WriteUFrameDat,
    FRC_ReadJointAngles,
    FRC_ReadCartesianPosition,
    FRC_Reset,
}

enum FRC_Packet{
    CommandPacket: <Packet<Command>>,
    CommunicationPacket: <Packet<Communication>>
}

impl Packet {
    // Serialize the packet into the required string format
    pub fn to_string(&self) -> Result<String, Error> {
        match serde_json::to_string(&self) {
            Ok(str) => Ok(str),
            Err(err) => Error::new("Could not serialize")
        }
        // let json_packet = match self {
        //     Packet::Communication => json!({"Communication": "FRC_Connect"}),
        //     Packet::Command => json!({"Command": "FRC_Disconnect"}),
        //     Packet::Initialize => json!({"Command": "FRC_Initialize"}),
        //     Packet::Instruction => json!({"Instruction": "FRC_Disconnect"}),
        //     Packet::Abort => json!({"Command": "FRC_Abort"}),
        //     Packet::Disconnect => json!({"Communication": "FRC_Disconnect"}),
        // };

        // format!("{}\r\n", json_packet.to_string())
    }
}


