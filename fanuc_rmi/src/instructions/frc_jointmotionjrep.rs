use serde::{Deserialize, Serialize};
use crate::{JointAngles, SpeedType, TermType};

#[derive(Serialize, Deserialize, Debug)]
pub struct FrcJointMotionJRep {
    #[serde(rename = "SequenceID")]
    sequence_id: u32,    
    #[serde(rename = "JointAngles")]
    joint_angles: JointAngles,
    #[serde(rename = "SpeedType")]
    speed_type: SpeedType,
    #[serde(rename = "Speed")]
    speed: u16,
    #[serde(rename = "TermType")]
    term_type: TermType,
    #[serde(rename = "TermValue")]
    term_value: u8,
}


impl FrcJointMotionJRep{
    pub fn new(    
        sequenceid: u32,    
        joints: JointAngles,
        speed_t: SpeedType,
        speed: u16,
        term_t: TermType,
        term_va: u8,
    
    ) -> Self {
        Self {
            sequence_id: sequenceid,    
            joint_angles: joints,
            speed_type: speed_t,
            speed: speed,
            term_type: term_t,
            term_value: term_va,
        }

    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FrcJointMotionJRepResponse { 
    #[serde(rename = "ErrorID")]
    pub error_id: u32,
    #[serde(rename = "SequenceID")]
    pub sequence_id: u32,
}