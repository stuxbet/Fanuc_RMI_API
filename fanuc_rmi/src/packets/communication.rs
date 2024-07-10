use serde::{Serialize, Deserialize};

use super::Packet;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "Communication")]
pub enum Communication {
    #[serde(rename = "FRC_Connect")]
    FrcConnect,
    #[serde(rename = "FRC_Disconnect")]
    FrcDisconnect,
    #[serde(rename = "FRC_Terminate")]
    FrcTerminate,
    #[serde(rename = "FRC_SystemFault")]
    FrcSystemFault,
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "Communication")]
pub enum CommunicationResponse {
    #[serde(rename = "FRC_Connect")]
    FrcConnect(FrcConnectResponse),
    #[serde(rename = "FRC_Disconnect")]
    FrcDisconnect(FrcDisconnectResponse),
    #[serde(rename = "FRC_Terminate")]
    FrcTerminate,
    #[serde(rename = "FRC_SystemFault")]
    FrcSystemFault,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FrcConnectResponse {
    #[serde(rename = "ErrorID")]
    pub error_id: u32,
    #[serde(rename = "PortNumber")]
    pub port_number: u32,
    #[serde(rename = "MajorVersion")]
    pub major_version: u16,
    #[serde(rename = "MinorVersion")]
    pub minor_version: u16
}
#[derive(Serialize, Deserialize, Debug)]

pub struct FrcDisconnectResponse {
    #[serde(rename = "ErrorID")]
    pub error_id: u32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct FrcSystemFault {
    #[serde(rename = "SequenceID")]
    pub sequence_id: u32,
}

impl Packet for Communication{}