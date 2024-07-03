use serde::Deserialize;
use std::{error::Error, io, sync::Arc, time::Duration};
use tokio::{io::AsyncWriteExt, io::AsyncReadExt, net::TcpStream, sync::Mutex, time::sleep};

use crate::packets::*;
use crate::instructions::*;
use crate::commands::*;
use crate::{Configuration, Position, SpeedType, TermType, FrcError};

pub struct FanucDriver {
    addr: String,
    initialize_port: u32,
    connection_port: Option<String>,
    tcp_stream: Option<Arc<Mutex<TcpStream>>>,
}

impl FanucDriver {
    pub fn new(addr: String, initialize_port: u32) -> FanucDriver {
        Self {
            addr,
            initialize_port,
            connection_port: None,
            tcp_stream: None,
        }
    }

    pub async fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        let init_addr = format!("{}:{}", self.addr, self.initialize_port);
        let stream = connect_with_retries(&init_addr, 3).await?;
        println!("Connected to the server at {}", init_addr);
        self.tcp_stream = Some(Arc::new(Mutex::new(stream)));

        // Create a connection packet
        let packet = Communication::FrcConnect {};
        
        let packet = serde_json::to_string(&packet).unwrap() + "\r\n";

        // Send a connection request packet to start the handshake
        let response = self.send::<CommunicationResponse>(packet).await?;

        self.connection_port = match response {
            CommunicationResponse::FrcConnect(res) => Some(res.port_number.to_string()),
            _ => None,
        };


        // Close the initial connection
        self.close_connection();

        match &self.connection_port {
            Some(port) => {
                // Connect to the new port
                let new_addr = format!("{}:{}",&self.addr, port);
                let stream = connect_with_retries(&new_addr, 3).await?;
                println!("Connected to the secondary server at {}", new_addr);
                self.tcp_stream = Some(Arc::new(Mutex::new(stream)));
            },
            None => {return Err(Box::new(io::Error::new(io::ErrorKind::InvalidData, "Port number is missing in the response")));}
        }

        Ok(())
    }

    pub fn close_connection(&mut self) {
        self.tcp_stream = None;
    }



    pub async fn initialize(&self) -> Result<(), Box<dyn Error>> {
        // Create a connection packet
        let packet = Command::FrcInitialize(FrcInitialize::default());
        
        // let packet = serde_json::to_string(&packet).expect("Initalize packet didnt serialize") + "\r\n";

        let packet = match serde_json::to_string(&packet) {
            Ok(serialized_packet) => serialized_packet + "\r\n",
            Err(_) => return Err(Box::new(FrcError::Serialization("Initalize packet didnt serialize correctly".to_string()))),
        };

        // Send a connection request packet to start the handshake
        let response = self.send::<CommandResponse>(packet).await?;
        if let CommandResponse::FrcInitialize(ref res) = response {
            if res.error_id != 0 {
                println!("Error ID: {}", res.error_id);
                return Err(Box::new(io::Error::new(io::ErrorKind::Interrupted, format!("Fanuc threw a Error #{} on a initialization packet", res.error_id))));
                // return Err(FrcError::FanucErrorCode(res.error_id));
            }
        }
        Ok(())

    }
    
    
    
    pub async fn abort(&self) -> Result<(), Box<dyn Error>> {

        let packet = Command::FrcAbort {};
        
        let packet = serde_json::to_string(&packet).expect("Abort packet didnt serialize") + "\r\n";

        let response = self.send::<CommandResponse>(packet).await?;
        if let CommandResponse::FrcAbort(ref res) = response {
            if res.error_id != 0 {
                println!("Error ID: {}", res.error_id);
                return Err(Box::new(io::Error::new(io::ErrorKind::Interrupted, format!("Fanuc threw a Error #{} on a abort packet", res.error_id))));
            }
        }
        Ok(())
    }
    pub async fn get_status(&self) -> Result<(), Box<dyn Error>> {

        let packet = Command::FrcGetStatus {};
        
        let packet = serde_json::to_string(&packet).expect("FrcGetStatus packet didnt serialize") + "\r\n";

        let response = self.send::<CommandResponse>(packet).await?;
        if let CommandResponse::FrcGetStatus(ref res) = response {
            if res.error_id != 0 {
                println!("Error ID: {}", res.error_id);
                return Err(Box::new(io::Error::new(io::ErrorKind::Interrupted, format!("Fanuc threw a Error #{} on a FrcGetStatus return packet", res.error_id))));
            }
        }
        Ok(())
    }

    pub async fn disconnect(&mut self) -> Result<(), Box<dyn Error>> {

        let packet = Communication::FrcDisconnect {};
        
        let packet = serde_json::to_string(&packet).expect("Disconnect packet didnt serialize") + "\r\n";

        let response = self.send::<CommunicationResponse>(packet).await?;
        if let CommunicationResponse::FrcDisconnect(ref res) = response {
            if res.error_id != 0 {
                println!("Error ID: {}", res.error_id);
                return Err(Box::new(io::Error::new(io::ErrorKind::Interrupted, format!("Fanuc threw a Error #{} on a Disconect packet", res.error_id))));
            }
        }
        self.close_connection();

        Ok(())

    }

    //this need to be updated and need clearification on location and config input
    pub async fn linear_motion(
        &self,
        sequenceid: u32,    
        config: Configuration,
        pos: Position,
        speed_t: SpeedType,
        speed: u16,
        term_t: TermType,
        term_va: u8,

    ) -> Result<(), Box<dyn Error>> {
        let packet = Instruction::FrcLinearMotion(FrcLinearMotion::new(
            sequenceid,    
            config,
            pos,
            speed_t,
            speed,
            term_t,
            term_va,

        ));
        
        let packet = serde_json::to_string(&packet).expect("Disconnect packet didnt serialize") + "\r\n";

        let response = self.send::<CommunicationResponse>(packet).await?;
        if let CommunicationResponse::FrcDisconnect(ref res) = response {
            if res.error_id != 0 {
                println!("Error ID: {}", res.error_id);
                return Err(Box::new(io::Error::new(io::ErrorKind::Interrupted, format!("Fanuc threw a Error #{} on a linear motion on return packet", res.error_id))));
            }
        }
        Ok(())

    }

    async fn send<T>(&self, packet: String) -> Result<T, Box<dyn Error>>
    where
        T: for<'a> Deserialize<'a> + std::fmt::Debug,
    {
        match &self.tcp_stream {
            Some(stream) => {
                let mut stream = stream.lock().await;
                // let packet = serde_json::to_string(&packet).unwrap() + "\r\n";
                stream.write_all(packet.as_bytes()).await?;
                println!("Sent: {}", packet);

                // Read response
                let mut buffer = vec![0; 2048];
                let n = stream.read(&mut buffer).await?;
                if n == 0 {
                    return Err(Box::new(io::Error::new(io::ErrorKind::Other, "Connection closed by peer")));
                }

                let response = String::from_utf8_lossy(&buffer[..n]);
                
                println!("Received: {}", response);

                // Parse JSON response
                match serde_json::from_str::<T>(&response) {
                    Ok(response_packet) => {
                        // Successfully parsed JSON into the generic type T
                        Ok(response_packet)
                    }
                    Err(e) => {
                        // Failed to parse JSON
                        println!("Could not parse response: {}", e);
                        Err(Box::new(io::Error::new(io::ErrorKind::Other, "could not parse response")))
                    }
                }
            }
            None => Err(Box::new(io::Error::new(io::ErrorKind::NotConnected, "Cannot send without initializing an open TCP stream"))),
        }
    }
}

impl Default for FanucDriver {
    fn default() -> Self {
        let addr = "127.0.0.1".to_string(); // Change if the server is running on a different machine
        let initialize_port = 16001;
        Self::new(addr, initialize_port)
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