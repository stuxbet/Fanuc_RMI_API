use serde::Deserialize;
use std::{error::Error, io, sync::Arc, time::Duration};
use tokio::{io::AsyncWriteExt, io::AsyncReadExt, net::TcpStream, sync::Mutex, time::sleep};

use crate::{commands::FrcInitialize, packets::*};


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
                let stream = connect_with_retries(&init_addr, 3).await?;
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
        match &self.tcp_stream {
            Some(stream) => {
                let mut stream = stream.lock().await;
                // Serialize to JSON string
                let packet = Command::FrcInitialize(FrcInitialize::default());
                let packet = serde_json::to_string(&packet).unwrap() + "\r\n";
                stream.write_all(packet.as_bytes()).await?;
                Ok(())
            }
            None => Err(Box::new(io::Error::new(io::ErrorKind::NotConnected, "Cannot initialize robot without an open TCP stream"))),
        }
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