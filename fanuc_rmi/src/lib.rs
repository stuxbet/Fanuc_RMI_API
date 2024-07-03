use std::{error::Error, fmt};

use packets::Communication;
use packets::Command;
use packets::Instruction;

use serde::{Deserialize, Serialize};


pub mod packets;
pub mod drivers;
pub mod instructions;
pub mod commands;
pub mod communication;

#[derive(Serialize, Deserialize, Debug)]
pub struct FrameData {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    p: f32,
    r: f32,
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Configuration {
    pub u_tool_number: u8,
    pub u_frame_number: u8,
    pub front: u8,
    pub up: u8,
    pub left: u8,
    pub glip: u8,
    pub turn4: u8,
    pub turn5: u8,
    pub turn6: u8,
}

#[derive(Serialize, Deserialize,Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
    pub p: f32,
    pub r: f32,
    pub ext1: f32,
    pub ext2: f32,
    pub ext3: f32,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct JointAngles {
    j1: f32,
    j2: f32,
    j3: f32,
    j4: f32,
    j5: f32,
    j6: f32,
    j7: f32,
    j8: f32,
    j9: f32,
}


#[derive(Serialize, Deserialize, Debug)]
pub enum TermType {
    FINE,
    CNT, // CNT with a value from 1 to 100
    CR,  // CR with a value from 1 to 100
}


/// Represents different types of speed measurements.
///
/// This enum provides various units of speed that can be used
/// to specify movement or duration in different contexts.
/// 
/// # Variants
///
/// * `MMSec` - Represents speed in millimeters per second (mm/sec).
/// * `InchMin` - Represents speed in inches per second.
/// * `Time` - Represents time in 0.1 second increments.
/// * `MilliSeconds` - Represents time in milliseconds (0.001 seconds).
#[derive(Serialize, Deserialize, Debug)]
pub enum SpeedType {
    #[serde(rename = "mmSec")]
    MMSec, // Speed in millimeters per second (mm/sec).
    InchMin, // Speed in inches per second.
    Time, // Time in 0.1 second increments.
    #[serde(rename = "mSec")]
    MilliSeconds, // Time in milliseconds (0.001 seconds).
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FrcError{
    Serialization(String),
    UnrecognizedPacket,
    FanucErrorCode(u32),

}
impl Error for FrcError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
impl fmt::Display for FrcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            FrcError::Serialization(ref msg) => write!(f, "Serialization error: {}", msg),
            FrcError::UnrecognizedPacket => write!(f, "Fanuc threw a unrecognized "),
            FrcError::FanucErrorCode(ref errorid) => write!(f, "fanuc returned  error#: {}", errorid),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]

pub enum PacketEnum {
    Communication(Communication),
    Command(Command),
    Instruction(Instruction)
}