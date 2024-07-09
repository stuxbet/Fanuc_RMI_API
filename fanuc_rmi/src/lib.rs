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

#[derive(Debug)]
enum ErrorCode {
    InternalSystemError = 2556929,
    InvalidUToolNumber = 2556930,
    InvalidUFrameNumber = 2556931,
    InvalidPositionRegister = 2556932,
    InvalidSpeedOverride = 2556933,
    CannotExecuteTPProgram = 2556934,
    ControllerServoOff = 2556935,
    CannotExecuteTPProgramDuplicate = 2556936,
    RMINotRunning = 2556937,
    TPProgramNotPaused = 2556938,
    CannotResumeTPProgram = 2556939,
    CannotResetController = 2556940,
    InvalidRMICommand = 2556941,
    RMICommandFail = 2556942,
    InvalidControllerState = 2556943,
    PleaseCyclePower = 2556944,
    InvalidPayloadSchedule = 2556945,
    InvalidMotionOption = 2556946,
    InvalidVisionRegister = 2556947,
    InvalidRMIInstruction = 2556948,
    InvalidValue = 2556949,
    InvalidTextString = 2556950,
    InvalidPositionData = 2556951,
    RMIInHoldState = 2556952,
    RemoteDeviceDisconnected = 2556953,
    RobotAlreadyConnected = 2556954,
    WaitForCommandDone = 2556955,
    WaitForInstructionDone = 2556956,
    InvalidSequenceIDNumber = 2556957,
    InvalidSpeedType = 2556958,
    InvalidSpeedValue = 2556959,
    InvalidTermType = 2556960,
    InvalidTermValue = 2556961,
    InvalidLCBPortType = 2556962,
    InvalidACCValue = 2556963,
    InvalidDestinationPosition = 2556964,
    InvalidVIAPosition = 2556965,
    InvalidPortNumber = 2556966,
    InvalidGroupNumber = 2556967,
    InvalidGroupMask = 2556968,
    JointMotionWithCOORD = 2556969,
    IncrementalMotionWithCOORD = 2556970,
    RobotInSingleStepMode = 2556971,
    InvalidPositionDataType = 2556972,
    ReadyForASCIIPacket = 2556973,
    ASCIIConversionFailed = 2556974,
    InvalidASCIIInstruction = 2556975,
    InvalidNumberOfGroups = 2556976,
    InvalidInstructionPacket = 2556977,
    InvalidASCIIStringPacket = 2556978,
    InvalidASCIIStringSize = 2556979,
    InvalidApplicationTool = 2556980,
    InvalidCallProgramName = 2556981,
}

impl ErrorCode {
    fn message(&self) -> &str {
        match self {
            ErrorCode::InternalSystemError => "Internal System Error.",
            ErrorCode::InvalidUToolNumber => "Invalid UTool Number.",
            ErrorCode::InvalidUFrameNumber => "Invalid UFrame Number.",
            ErrorCode::InvalidPositionRegister => "Invalid Position Register.",
            ErrorCode::InvalidSpeedOverride => "Invalid Speed Override.",
            ErrorCode::CannotExecuteTPProgram => "Cannot Execute TP program.",
            ErrorCode::ControllerServoOff => "Controller Servo is Off.",
            ErrorCode::CannotExecuteTPProgramDuplicate => "Cannot Execute TP program.",
            ErrorCode::RMINotRunning => "RMI is Not Running.",
            ErrorCode::TPProgramNotPaused => "TP Program is Not Paused.",
            ErrorCode::CannotResumeTPProgram => "Cannot Resume TP Program.",
            ErrorCode::CannotResetController => "Cannot Reset Controller.",
            ErrorCode::InvalidRMICommand => "Invalid RMI Command.",
            ErrorCode::RMICommandFail => "RMI Command Fail.",
            ErrorCode::InvalidControllerState => "Invalid Controller State.",
            ErrorCode::PleaseCyclePower => "Please Cycle Power.",
            ErrorCode::InvalidPayloadSchedule => "Invalid Payload Schedule.",
            ErrorCode::InvalidMotionOption => "Invalid Motion Option.",
            ErrorCode::InvalidVisionRegister => "Invalid Vision Register.",
            ErrorCode::InvalidRMIInstruction => "Invalid RMI Instruction.",
            ErrorCode::InvalidValue => "Invalid Value.",
            ErrorCode::InvalidTextString => "Invalid Text String.",
            ErrorCode::InvalidPositionData => "Invalid Position Data.",
            ErrorCode::RMIInHoldState => "RMI is In HOLD State.",
            ErrorCode::RemoteDeviceDisconnected => "Remote Device Disconnected.",
            ErrorCode::RobotAlreadyConnected => "Robot is Already Connected.",
            ErrorCode::WaitForCommandDone => "Wait for Command Done.",
            ErrorCode::WaitForInstructionDone => "Wait for Instruction Done.",
            ErrorCode::InvalidSequenceIDNumber => "Invalid sequence ID number.",
            ErrorCode::InvalidSpeedType => "Invalid Speed Type.",
            ErrorCode::InvalidSpeedValue => "Invalid Speed Value.",
            ErrorCode::InvalidTermType => "Invalid Term Type.",
            ErrorCode::InvalidTermValue => "Invalid Term Value.",
            ErrorCode::InvalidLCBPortType => "Invalid LCB Port Type.",
            ErrorCode::InvalidACCValue => "Invalid ACC Value.",
            ErrorCode::InvalidDestinationPosition => "Invalid Destination Position.",
            ErrorCode::InvalidVIAPosition => "Invalid VIA Position.",
            ErrorCode::InvalidPortNumber => "Invalid Port Number.",
            ErrorCode::InvalidGroupNumber => "Invalid Group Number.",
            ErrorCode::InvalidGroupMask => "Invalid Group Mask.",
            ErrorCode::JointMotionWithCOORD => "Joint motion with COORD.",
            ErrorCode::IncrementalMotionWithCOORD => "Incremental motn with COORD.",
            ErrorCode::RobotInSingleStepMode => "Robot in Single Step Mode.",
            ErrorCode::InvalidPositionDataType => "Invalid Position Data Type.",
            ErrorCode::ReadyForASCIIPacket => "Ready for ASCII Packet.",
            ErrorCode::ASCIIConversionFailed => "ASCII Conversion Failed.",
            ErrorCode::InvalidASCIIInstruction => "Invalid ASCII Instruction.",
            ErrorCode::InvalidNumberOfGroups => "Invalid Number of Groups.",
            ErrorCode::InvalidInstructionPacket => "Invalid Instruction packet.",
            ErrorCode::InvalidASCIIStringPacket => "Invalid ASCII String packet.",
            ErrorCode::InvalidASCIIStringSize => "Invalid ASCII string size.",
            ErrorCode::InvalidApplicationTool => "Invalid Application Tool.",
            ErrorCode::InvalidCallProgramName => "Invalid Call Program Name.",
        }
    }
}


