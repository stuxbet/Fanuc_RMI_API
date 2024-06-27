// use serde_json;
// use serde_derive::{Deserialize, Serialize};
// use crate::packets::Command;

// #[derive(Serialize, Deserialize)]
// pub enum CommunicationPacket {
//     FRC_Connect,
//     FRC_Disconnect,
// }

// // #[derive(Serialize, Deserialize)]
// // pub enum CommandPacket {
// //     FRC_Initialize,
// //     FRC_Abort,
// //     FRC_Pause,
// //     Continue,
// //     Reset,
// //     ReadError { count: u8 },
// //     SetUFrameUTool { uframe_number: u8, utool_number: u8, group: Option<u8> },
// //     GetStatus,
// //     ReadUFrameData { frame_number: u8, group: Option<u8> },
// //     WriteUFrameData { frame_number: u8, frame: FrameData, group: Option<u8> },
// //     ReadUToolData { tool_number: u8, group: Option<u8> },
// //     WriteUToolData { tool_number: u8, frame: FrameData, group: Option<u8> },
// //     ReadDIN { port_number: u16 },
// //     WriteDOUT { port_number: u16, port_value: String },
// //     ReadCartesianPosition { group: Option<u8> },
// //     ReadJointAngles { group: Option<u8> },
// //     SetOverRide { value: u8 },
// //     GetUFrameUTool { group: Option<u8> },
// //     ReadPositionRegister { register_number: u16, group: Option<u8> },
// //     WritePositionRegister { register_number: u16, configuration: Configuration, position: Position, group: Option<u8> },
// //     ReadTCPSpeed,
// // }

// // #[derive(Serialize, Deserialize)]
// // pub enum InstructionPacket {
// //     FRC_WaitDIN,            // Wait for DIN Instruction
// //     FRC_SetUFrame,          // Set User Frame Instruction
// //     FRC_SetUTool,           // Set User Tool Instruction
// //     FRC_WaitTime,           // Add Wait Time Instruction
// //     FRC_SetPayLoad,         // Set Payload Instruction
// //     FRC_Call,               // Call a Program
// //     FRC_LinearMotion,       // Add Linear Motion Instruction
// //     FRC_LinearRelative,     // Add Linear Incremental Motion Instruction
// //     FRC_JointMotion,        // Add Joint Motion Instruction
// //     FRC_JointRelative,      // Add Joint Incremental Motion Instruction
// //     FRC_CircularMotion,     // Add Circular Motion Instruction
// //     FRC_CircularRelative,   // Add Circular Incremental Motion Instruction
// //     FRC_JointMotionJRep,    // Add Joint Motion with Joint Representation
// //     FRC_JointRelativeJRep,  // Add Joint Incremental Motion with Joint Representation
// //     FRC_LinearMotionJRep,   // Add Linear Motion with Joint Representation
// // }




// // #[derive(Serialize, Deserialize)]
// // pub enum Packet {
// //     Communication(CommunicationPacket),
// //     Command(CommandPacket),
// //     Instruction(InstructionPacket),
// // }
// #[derive(Serialize, Deserialize)]

// pub struct Attributes {
//     #[serde(flatten)]
//     extra: serde_json::Value,
// }

// //////////////////////////////////////////////////////////////////////////////////////
// /// Utilities 
// /// Data types here may be exported elsewhere
// ////////////////////////////////////////////////////////////////////////////////////
// // #[derive(Serialize, Deserialize)]
// // pub struct FrameData {
// //     x: f32,
// //     y: f32,
// //     z: f32,
// //     w: f32,
// //     p: f32,
// //     r: f32,
// // }




// /////////////////////////////////////////////////////////////////////////////////////////////
// //new definition system


// // #[serde(flatten)]
// #[derive(Serialize, Deserialize)]
// pub struct CommandPacketStruct {
//     pub CommandPacket: CommandPacket,
//     pub group: Option<u8>
// }


// //this looks dumb but serializes correctly?
// #[derive(Serialize, Deserialize)]
// pub struct CommunicationPacketStruct {
//     pub Communication: CommunicationPacket,
// }





// #[derive(Serialize, Deserialize)]
// pub struct MotionPacket {
//     pub Instruction: InstructionPacket,
//     pub SequenceID: u32,
//     pub Configuration: Configuration,
//     pub Position: Position,
//     pub SpeedType: SpeedType,
//     pub Speed: u16,
//     pub TermType: TermType,
// }

// #[derive(Serialize, Deserialize)]
// pub struct InitializePacket {
//     pub Command: CommandPacket,
//     pub Groupmask: u8,
// }

// impl InitializePacket{
//     pub fn new(groupmask: Option<u8>) -> Self {
//         let groupmask = match groupmask {
//             Some(gm) => gm,
//             None => 1
//         };

//         Self {
//             Command: CommandPacket::FRC_Initialize,
//             Groupmask: groupmask
//         }
//     }
// }


// impl Default for InitializePacket {
//     fn default() -> Self {
//         InitializePacket::new(Some(1))
//     }
// }


