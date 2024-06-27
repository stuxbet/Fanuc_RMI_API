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
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FrcConnectResponse {
    pub error_id: u32,
    pub port_number: u16,
    pub major_version: u16,
    pub minor_version: u16
}

impl Packet for Communication{}