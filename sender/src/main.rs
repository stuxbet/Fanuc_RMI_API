use serde_json::to_string;
use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use serde_json;
pub use serde::{Deserialize,Serialize};
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;
// use shared_def::packet_defs::*;
use shared_def::packet_functions::*;
// use shared_def::packet_defs::Packet;
// use shared_def::packet_defs::linea;



async fn send_packet(stream: &mut TcpStream, packet:String) -> Result<serde_json::Value,Box<dyn Error>> {
    // let packet = packet + "\r\n";

    //let serialized_packet: String = serde_json::to_string(&packet).unwrap() + "\r\n";

    stream.write_all(packet.as_bytes()).await?;
    println!("Sent: {}", packet);

    // Read response
    let mut buffer = vec![0; 2048];
    let n = stream.read(&mut buffer).await?;
    if n == 0 {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Connection closed by peer")));
    }

    let response = String::from_utf8_lossy(&buffer[..n]);
    println!("Received: {}", response);

    // Parse JSON response
    let response_json = serde_json::from_str(&response)?;

    println!("Parsed response: {:?}", response_json);

    Ok(response_json)
}

async fn connect_with_retries(addr: &str, retries: u32) -> Result<TcpStream, Box<dyn Error>> {
    for attempt in 0..retries {
        match TcpStream::connect(addr).await {
            Ok(stream) => return Ok(stream),
            Err(e) => {
                eprintln!("Failed to connect (attempt {}): {}", attempt + 1, e);
                if attempt + 1 == retries {
                    return Err(Box::new(e));
                }
                sleep(Duration::from_secs(2)).await;
            }
        }
    }
    Err("Exceeded maximum connection attempts".into())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {


    let addr = "127.0.0.1:16001"; // Change if the server is running on a different machine
    let mut stream = connect_with_retries(addr, 3).await?;
    println!("Connected to the server at {}", addr);



    let response = send_packet(&mut stream, connect_packet()).await?;

    // if(response["major"] < major){println!("Not compatible");}

    // Extract the new port number from the response
    let new_port = response["PortNumber"].as_u64().ok_or("No port number in response")? as u16;

    // Close the initial connection
    drop(stream);

    // Connect to the new port
    let new_addr = format!("127.0.0.1:{}", new_port);
    let mut new_stream = connect_with_retries(&new_addr, 3).await?;
    println!("Connected to the secondary server at {}", new_addr);

    // Initialize the robot

    let send_status = send_packet(&mut new_stream, Initialize_packet(None)).await;
    match send_status {
        Ok(_) => println!("Initialized connection with the robot"),
        Err(err) => {
            println!("{}", format!("An error occured opening the connection to the robot: {}", err));
            return Ok(());
        }
    };

    println!("Serialized LinearMotionPacket: {}", linear()); // Debugging line
    println!("do this work?: {}", connect_packet()); // Debugging line

    //FIXME:
    // send_packet(&mut new_stream, &linear_motion_packet_json).await?;


    // // Abort the motion
    send_packet(&mut new_stream, Abort_packet()).await?;

    // // Disconnect from the server
    let echo = send_packet(&mut new_stream, Disconnect_packet()).await?;

    Ok(())
}
