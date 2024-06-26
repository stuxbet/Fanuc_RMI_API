
use crate::packet_defs::*;

pub fn connect_packet() -> String{
    // Serialize to JSON string
    serde_json::to_string(&Packet::Communication(CommunicationPacket::FRC_Connect )).unwrap() + "\r\n"
}
pub fn Disconnect_packet() -> String{
    // Serialize to JSON string
    serde_json::to_string(&Packet::Communication(CommunicationPacket::FRC_Disconnect )).unwrap() + "\r\n"
}
pub fn Initialize_packet(GroupMask: Option<u8>) -> String{
    // Serialize to JSON string
    let GroupMask = GroupMask.unwrap_or(0); // Default to 0 if None

    serde_json::to_string(&Packet::Command(CommandPacket::FRC_Initialize ) ).unwrap() + "\r\n"

}
pub fn Abort_packet() -> String{
    // Serialize to JSON string
    serde_json::to_string(&Packet::Command(CommandPacket::FRC_Abort)).unwrap() + "\r\n"

}
pub fn linear() -> String{
    let packet = MotionPacket {
        Instruction: "FRC_LinearMotion".to_string(),
        SequenceID: 1,
        Configuration: Configuration {
            UToolNumber: 1,
            UFrameNumber: 1,
            Front: 0,
            Up: 1,
            Left: 0,
            Flip: 0,
            Turn4: 0,
            Turn5: 0,
            Turn6: 0,
        },
        Position: Position {
            X: 500.0,
            Y: 0.0,
            Z: 300.0,
            W: 0.0,
            P: 0.0,
            R: 0.0,
        },
        SpeedType: SpeedType::mmSec,
        Speed: 100,
        TermType: TermType::FINE,
    };

    // Serialize to JSON string
    serde_json::to_string(&packet).unwrap()

    // match serde_json::to_string(&packet) {
    //     Ok(json_packet) => {
    //         println!("{}", json_packet);
    //         json_packet
    //     }
    //     Err(e) => {
    //         eprintln!("Error serializing packet: {}", e);
    //         e.to_string()
    //     }
    // }

}