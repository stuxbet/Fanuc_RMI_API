use serde::{Serialize,Deserialize};
use super::Packet;
use crate::instructions::*;


#[derive(Serialize, Deserialize, Debug)]
pub enum Instruction {
    #[serde(rename = "FRC_WaitDIN")]
    FrcWaitDIN(FrcWaitDIN),            // Wait for DIN Instruction
    #[serde(rename = "FRC_SetUFrame")]
    FrcSetUFrame,          // Set User Frame Instruction
    #[serde(rename = "FRC_SetUTool")]
    FrcSetUTool,           // Set User Tool Instruction
    #[serde(rename = "FRC_WaitTime")]
    FrcWaitTime,           // Add Wait Time Instruction
    #[serde(rename = "FRC_SetPayLoad")]
    FrcSetPayLoad,         // Set Payload Instruction
    #[serde(rename = "FRC_Call")]
    FrcCall,               // Call a Program
    #[serde(rename = "FRC_LinearMotion")]
    FrcLinearMotion,       // Add Linear Motion Instruction
    #[serde(rename = "FRC_LinearRelative")]
    FrcLinearRelative,     // Add Linear Incremental Motion Instruction
    #[serde(rename = "FRC_JointMotion")]
    FrcJointMotion,        // Add Joint Motion Instruction
    #[serde(rename = "FRC_JointRelative")]
    FrcJointRelative,      // Add Joint Incremental Motion Instruction
    #[serde(rename = "FRC_CircularMotion")]
    FrcCircularMotion,     // Add Circular Motion Instruction
    #[serde(rename = "FRC_CircularRelative")]
    FrcCircularRelative,   // Add Circular Incremental Motion Instruction
    #[serde(rename = "FRC_JointMotionJRep")]
    FrcJointMotionJRep,    // Add Joint Motion with Joint Representation
    #[serde(rename = "FRC_JointRelativeJRep")]
    FrcJointRelativeJRep,  // Add Joint Incremental Motion with Joint Representation
    #[serde(rename = "FRC_LinearMotionJRep")]
    FrcLinearMotionJRep,   // Add Linear Motion with Joint Representation
}

#[derive(Serialize, Deserialize, Debug)]
pub enum OnOff{
    ON,
    OFF
}

impl Packet for Instruction{}