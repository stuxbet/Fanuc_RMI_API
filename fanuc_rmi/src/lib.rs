use std::collections::HashMap;
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


// static RMIERROR_ID: HashMap<u32, &str> = HashMap::from([
//     (2556929, "Internal System Error."), 
//     (2556930,  "Invalid UTool Number."), 
//     (2556931, "Invalid UFrame Number."),
//     (2556932, "Invalid Position Register."),
//     (2556933, "Invalid Speed Override."),
//     (2556934,  "Cannot Execute TP program."),
//     (2556935, "Controller Servo is Off."),
//     (2556936, "Cannot Execute TP program."),
//     (2556937, "RMI is Not Running."),
//     (2556938, "TP Program is Not Paused."),
//     (2556939, "Cannot Resume TP Program."),
//     (2556940, "Cannot Reset Controller."),
//     (2556941, "Invalid RMI Command."),
//     (2556942, "RMI Command Fail."),
//     (2556943, "Invalid Controller State."),
//     (2556944, "Please Cycle Power."),
//     (2556945, "Invalid Payload Schedule."),
//     (2556946, "Invalid Motion Option."),
//     (2556947, "Invalid Vision Register."),
//     (2556948, "Invalid RMI Instruction."),
//     (2556949, "Invalid Value."),
//     (2556950, "Invalid Text String"),
//     (2556951, "Invalid Position Data"),
//     (2556952, "RMI is In HOLD State"),
//     (2556953, "Remote Device Disconnected."),
//     (2556954, "Robot is Already Connected."),
//     (2556955, "Wait for Command Done."),
//     (2556956, "Wait for Instruction Done."),
//     (2556957, "Invalid sequence ID number."),
//     (2556958, "Invalid Speed Type."),
//     (2556959, "Invalid Speed Value."),
//     (2556960, "Invalid Term Type."),
//     (2556961, "Invalid Term Value."),
//     (2556962, "Invalid LCB Port Type."),
//     (2556963, "Invalid ACC Value."),
//     (2556964, "Invalid Destination Position"),
//     (2556965, "Invalid VIA Position."),
//     (2556966, "Invalid Port Number."),
//     (2556967, "Invalid Group Number"),
//     (2556968, "Invalid Group Mask"),
//     (2556969, "Joint motion with COORD"),
//     (2556970, "Incremental motn with COORD"),
//     (2556971, "Robot in Single Step Mode"),
//     (2556972, "Invalid Position Data Type"),
//     (2556973, "Ready for ASCII Packet"),
//     (2556974, "ASCII Conversion Failed"),
//     (2556975, "Invalid ASCII Instruction"),
//     (2556976, "Invalid Number of Groups"),
//     (2556977, "Invalid Instruction packet"),
//     (2556978, "Invalid ASCII String packet"),
//     (2556979, "Invalid ASCII string size"),
//     (2556980, "Invalid Application Tool"),
//     (2556981, "Invalid Call Program Name"),
// ]);

const RMIERROR_ENTRIES: &[(u32, &str)] = &[
    (2556929, "Internal System Error."),
    (2556930,  "Invalid UTool Number."),
    (2556931, "Invalid UFrame Number."),
    (2556932, "Invalid Position Register."),
    (2556933, "Invalid Speed Override."),
    (2556934,  "Cannot Execute TP program."),
    (2556935, "Controller Servo is Off."),
    (2556936, "Cannot Execute TP program."),
    (2556937, "RMI is Not Running."),
    (2556938, "TP Program is Not Paused."),
    (2556939, "Cannot Resume TP Program."),
    (2556940, "Cannot Reset Controller."),
    (2556941, "Invalid RMI Command."),
    (2556942, "RMI Command Fail."),
    (2556943, "Invalid Controller State."),
    (2556944, "Please Cycle Power."),
    (2556945, "Invalid Payload Schedule."),
    (2556946, "Invalid Motion Option."),
    (2556947, "Invalid Vision Register."),
    (2556948, "Invalid RMI Instruction."),
    (2556949, "Invalid Value."),
    (2556950, "Invalid Text String"),
    (2556951, "Invalid Position Data"),
    (2556952, "RMI is In HOLD State"),
    (2556953, "Remote Device Disconnected."),
    (2556954, "Robot is Already Connected."),
    (2556955, "Wait for Command Done."),
    (2556956, "Wait for Instruction Done."),
    (2556957, "Invalid sequence ID number."),
    (2556958, "Invalid Speed Type."),
    (2556959, "Invalid Speed Value."),
    (2556960, "Invalid Term Type."),
    (2556961, "Invalid Term Value."),
    (2556962, "Invalid LCB Port Type."),
    (2556963, "Invalid ACC Value."),
    (2556964, "Invalid Destination Position"),
    (2556965, "Invalid VIA Position."),
    (2556966, "Invalid Port Number."),
    (2556967, "Invalid Group Number"),
    (2556968, "Invalid Group Mask"),
    (2556969, "Joint motion with COORD"),
    (2556970, "Incremental motn with COORD"),
    (2556971, "Robot in Single Step Mode"),
    (2556972, "Invalid Position Data Type"),
    (2556973, "Ready for ASCII Packet"),
    (2556974, "ASCII Conversion Failed"),
    (2556975, "Invalid ASCII Instruction"),
    (2556976, "Invalid Number of Groups"),
    (2556977, "Invalid Instruction packet"),
    (2556978, "Invalid ASCII String packet"),
    (2556979, "Invalid ASCII string size"),
    (2556980, "Invalid Application Tool"),
    (2556981, "Invalid Call Program Name"),
];

fn get_rmierror_id() -> HashMap<u32, &'static str> {
    RMIERROR_ENTRIES.iter().copied().collect()
}