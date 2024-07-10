use serde::{Serialize,Deserialize};
use super::Packet;
use crate::instructions::*;


#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "Instruction")]
pub enum Instruction {
    #[serde(rename = "FRC_WaitDIN")]
    FrcWaitDIN(FrcWaitDIN),            // Wait for DIN Instruction

    #[serde(rename = "FRC_SetUFrame")]
    FrcSetUFrame(FrcSetUFrame),          // Set User Frame Instruction

    #[serde(rename = "FRC_SetUTool")]
    FrcSetUTool(FrcSetUTool),           // Set User Tool Instruction
    
    #[serde(rename = "FRC_WaitTime")]
    FrcWaitTime(FrcWaitTime),           // Add Wait Time Instruction

    #[serde(rename = "FRC_SetPayLoad")]
    FrcSetPayLoad(FrcSetPayLoad),         // Set Payload Instruction

    #[serde(rename = "FRC_Call")]
    FrcCall(FrcCall),               // Call a Program

    #[serde(rename = "FRC_LinearMotion")]
    FrcLinearMotion(FrcLinearMotion),       // Add Linear Motion Instruction

    #[serde(rename = "FRC_LinearRelative")]
    FrcLinearRelative(FrcLinearRelative),     // Add Linear Incremental Motion Instruction

    #[serde(rename = "FRC_LinearRelativeJRep")]
    FrcLinearRelativeJRep(FrcLinearRelativeJRep),     // Add Linear Relative Motion with Joint Representation

    #[serde(rename = "FRC_JointMotion")]
    FrcJointMotion(FrcJointMotion),        // Add Joint Motion Instruction

    #[serde(rename = "FRC_JointRelative")]
    FrcJointRelative(FrcJointRelative),      // Add Joint Incremental Motion Instruction

    #[serde(rename = "FRC_CircularMotion")]
    FrcCircularMotion(FrcCircularMotion),     // Add Circular Motion Instruction

    #[serde(rename = "FRC_CircularRelative")]
    FrcCircularRelative(FrcCircularRelative),   // Add Circular Incremental Motion Instruction

    #[serde(rename = "FRC_JointMotionJRep")]
    FrcJointMotionJRep(FrcJointMotionJRep),    // Add Joint Motion with Joint Representation

    #[serde(rename = "FRC_JointRelativeJRep")]
    FrcJointRelativeJRep(FrcJointRelativeJRep),  // Add Joint Incremental Motion with Joint Representation

    #[serde(rename = "FRC_LinearMotionJRep")]
    FrcLinearMotionJRep(FrcLinearMotionJRep),   // Add Linear Motion with Joint Representation
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "Instruction")]
pub enum InstructionResponse {
    #[serde(rename = "FRC_WaitDIN")]
    FrcWaitDIN(FrcWaitDINResponse),
    #[serde(rename = "FRC_SetUFrame")]
    FrcSetUFrame(FrcSetUFrameResponse),
    #[serde(rename = "FRC_SetUTool")]
    FrcSetUTool(FrcSetUToolResponse),
    #[serde(rename = "FRC_WaitTime")]
    FrcWaitTime(FrcWaitTimeResponse),
    #[serde(rename = "FRC_SetPayLoad")]
    FrcSetPayLoad(FrcSetPayLoadResponse),
    #[serde(rename = "FRC_Call")]
    FrcCall(FrcCallResponse),
    #[serde(rename = "FRC_LinearMotion")]
    FrcLinearMotion(FrcLinearMotionResponse), 
    #[serde(rename = "FRC_LinearRelative")]
    FrcLinearRelative(FrcLinearRelativeResponse),
    #[serde(rename = "FRC_LinearRelativeJRep")]
    FrcLinearRelativeJRep(FrcLinearRelativeJRepResponse),  
    #[serde(rename = "FRC_JointMotion")]
    FrcJointMotion(FrcJointMotionResponse),  
    #[serde(rename = "FRC_JointRelative")]
    FrcJointRelative(FrcJointRelativeResponse),  
    #[serde(rename = "FRC_CircularMotion")]
    FrcCircularMotion(FrcCircularMotionResponse), 
    #[serde(rename = "FRC_CircularRelative")]
    FrcCircularRelative(FrcCircularRelativeResponse), 
    #[serde(rename = "FRC_JointMotionJRep")]
    FrcJointMotionJRep(FrcJointMotionJRepResponse),   
    #[serde(rename = "FRC_JointRelativeJRep")]
    FrcJointRelativeJRep(FrcJointRelativeJRepResponse),
    #[serde(rename = "FRC_LinearMotionJRep")]
    FrcLinearMotionJRep(FrcLinearMotionJRepResponse), 


}


impl InstructionResponse {
    pub fn get_sequence_id(&self) -> u32 {
        match self {
            InstructionResponse::FrcWaitDIN(resp) => resp.sequence_id,
            InstructionResponse::FrcSetUFrame(resp) => resp.sequence_id,
            InstructionResponse::FrcSetUTool(resp) => resp.sequence_id,
            InstructionResponse::FrcWaitTime(resp) => resp.sequence_id,
            InstructionResponse::FrcSetPayLoad(resp) => resp.sequence_id,
            InstructionResponse::FrcCall(resp) => resp.sequence_id,
            InstructionResponse::FrcLinearMotion(resp) => resp.sequence_id,
            InstructionResponse::FrcLinearRelative(resp) => resp.sequence_id,
            InstructionResponse::FrcLinearRelativeJRep(resp) => resp.sequence_id,
            InstructionResponse::FrcJointMotion(resp) => resp.sequence_id,
            InstructionResponse::FrcJointRelative(resp) => resp.sequence_id,
            InstructionResponse::FrcCircularMotion(resp) => resp.sequence_id,
            InstructionResponse::FrcCircularRelative(resp) => resp.sequence_id,
            InstructionResponse::FrcJointMotionJRep(resp) => resp.sequence_id,
            InstructionResponse::FrcJointRelativeJRep(resp) => resp.sequence_id,
            InstructionResponse::FrcLinearMotionJRep(resp) => resp.sequence_id,
        }
    }
}




#[derive(Serialize, Deserialize, Debug)]
pub enum OnOff{
    ON,
    OFF
}

impl Packet for Instruction{}