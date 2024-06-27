use fanuc_rmi::drivers::FanucDriver;
use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use serde_json;
pub use serde::{Deserialize,Serialize};
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;
use std::io;

async fn send_packet(stream: &mut TcpStream, packet:String) -> Result<serde_json::Value,Box<dyn Error>> {

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

    match serde_json::from_str::<serde_json::Value>(&response) {
        Ok(response_json) => {
            // Successfully parsed JSON, you can use `response_json` here
            println!("Parsed JSON: {:?}", response_json);
            Ok(response_json)

        }
        Err(e) => {
            // Failed to parse JSON
            println!("could not parse json: {}", e);
            Err(Box::new(io::Error::new(io::ErrorKind::Other, "could not parse json")))

        }
    }

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

    let mut driver = FanucDriver::default();
    match driver.connect().await {
        Ok(_) => println!("Connected successfully"),
        Err(e) => println!("Failed to connect: {}", e),
    }
    Ok(())
}
