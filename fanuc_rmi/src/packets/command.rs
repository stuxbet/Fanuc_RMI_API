use serde::{Serialize, Deserialize};
use crate::{FrameData, Configuration, Position};
use super::Packet;
use crate::commands::*;


#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "Command")]
pub enum Command {
    #[serde(rename = "FRC_Initialize")]
    FrcInitialize(FrcInitialize),
    #[serde(rename = "FRC_Abort")]
    FrcAbort,
    #[serde(rename = "FRC_Pause")]
    FrcPause,
    #[serde(rename = "FRC_ReadError")]
    FrcReadError(FrcReadError),
    #[serde(rename = "FRC_Continue")]
    FrcContinue,
    #[serde(rename = "FRC_SetUFrameUTool")]
    FrcSetUframeUtool(FrcSetUframeUtool),



    //only requires that the remote device has made a connection to the robot controller.
    #[serde(rename = "FRC_ReadPositionRegister")]
    FrcReadPositionRegister,
    #[serde(rename = "FrcWritePositionRegister")]
    FrcWritePositionRegister,
    #[serde(rename = "FRC_SetOverride")]
    FrcSetOverride,
    #[serde(rename = "FRC_GetStatus")]
    FrcGetStatus,
    #[serde(rename = "FRC_GetUFrameUTool")]
    FrcGetUframeUtool,
    #[serde(rename = "FRC_ReadUToolData")]
    FrcReadUtoolData,
    #[serde(rename = "FRC_WriteUToolData")]
    FrcWriteUtoolData,
    #[serde(rename = "FRC_ReadUFrameData")]
    FrcReadUframeData,
    #[serde(rename = "FRC_WriteUFrameData")]
    FrcWriteUframeData,
    #[serde(rename = "FRC_ReadJointAngles")]
    FrcReadJointAngles,
    #[serde(rename = "FRC_ReadCartesianPosition")]
    FrcReadCartesianPosition,
    #[serde(rename = "FRC_Reset")]
    FrcReset,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "Command")]
pub enum CommandResponse {
    #[serde(rename = "FRC_Initialize")]
    FrcInitialize(FrcInitializeResponse),
    #[serde(rename = "FRC_Abort")]
    FrcAbort,
    #[serde(rename = "FRC_Pause")]
    FrcPause,
}


impl Packet for Command{}
