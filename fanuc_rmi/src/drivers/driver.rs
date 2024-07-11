use serde::Deserialize;
// use serde::Serialize;
use tokio::sync::mpsc;
use std::{error::Error, io, sync::Arc, time::Duration};
use tokio::{ net::TcpStream, sync::Mutex, time::sleep};
use tokio::io::{ AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf, split};
use std::collections::VecDeque;

use crate::packets::*;
use crate::instructions::*;
use crate::commands::*;
use crate::PacketEnum;
use crate::{Configuration, Position, SpeedType, TermType, FrcError };

#[derive(Debug,Clone)]
pub struct  FanucDriverConfig {
    addr: String,
    port: u32,
    max_messages: usize,
}


impl Default for FanucDriverConfig {
    fn default() -> Self {
        let addr = "127.0.0.1".to_string(); // Change if the server is running on a different machine
        let port = 16001;
        let max_messages = 30;
        Self {
            addr,
            port,
            max_messages
        }
    }
}

#[derive( Debug, Clone)]
pub struct FanucDriver {
    pub config: FanucDriverConfig,
    pub messages: Arc<Mutex<VecDeque<String>>>,
    write_half: Arc<Mutex<WriteHalf<TcpStream>>>,
    read_half: Arc<Mutex<ReadHalf<TcpStream>>>,
}

impl FanucDriver {
    pub async fn connect(config: FanucDriverConfig) -> Result<FanucDriver, Box<dyn Error>> {
        let init_addr = format!("{}:{}",&config.addr, &config.port);
        let mut stream = connect_with_retries(&init_addr, 3).await?;

        // Create a connection packet
        let packet = Communication::FrcConnect {};
        
        let packet = match serde_json::to_string(&packet) {
            Ok(serialized_packet) => serialized_packet + "\r\n",
            Err(_) => return Err(Box::new(FrcError::Serialization("Communication: Connect packet didnt serialize correctly".to_string()))),
        };

        stream.write(packet.as_bytes()).await?;
        
        let mut buffer = vec![0; 2048];

        let n = stream.read(&mut buffer).await?;
        if n == 0 {
            return Err(Box::new(io::Error::new(io::ErrorKind::Other, "Connection closed by peer")));
        }

        let response = String::from_utf8_lossy(&buffer[..n]);
        
        println!("Sent: {}\nReceived: {}", &packet, &response);

        // Parse JSON response
        let res = match serde_json::from_str::<CommunicationResponse>(&response) {
            Ok(response_packet) => {
                // Successfully parsed JSON into the generic type T
                response_packet
            }
            Err(e) => {
                // Failed to parse JSON
                println!("Could not parse response: {}", e);
                return Err(Box::new(io::Error::new(io::ErrorKind::Other, "could not parse response")))
            }
        };

        let mut new_port = 0;
        match res {
            CommunicationResponse::FrcConnect(res) => new_port = res.port_number,
            _ => ()
        };

        drop(stream);
        let init_addr = format!("{}:{}",config.addr, new_port);
        let stream = connect_with_retries(&init_addr, 3).await?;

        // let (tx, mut rx) = mpsc::channel(100);
        

        let (read_half, write_half) = split(stream);
        let read_half = Arc::new(Mutex::new(read_half));
        let write_half = Arc::new(Mutex::new(write_half));
        let messages: Arc<Mutex<VecDeque<String>>> = Arc::new(Mutex::new(VecDeque::new()));
        let mut msg = messages.lock().await;
        msg.push_back("Connected".to_string());
        drop(msg);
        // messages.push_back("Connected".to_string());

        Ok(Self {
            config,
            messages,
            write_half,
            read_half,
        })
    }

    async fn log_message<T: Into<String>>(&self, message:T){
        let message = message.into();
        let messages = self.messages.clone();
        let mut messages = messages.lock().await;

        #[cfg(feature="logging")]
        println!(&message);

        loop{
            if messages.len() >= self.config.max_messages {
                messages.pop_front();
            }
            else{break}
        }
        messages.push_back(message);
    }


    pub async fn initialize(&self) -> Result<(), Box<dyn Error>> {

        let packet = Command::FrcInitialize(FrcInitialize::default());
        
        let packet = match serde_json::to_string(&packet) {
            Ok(serialized_packet) => serialized_packet + "\r\n",
            Err(_) => return Err(Box::new(FrcError::Serialization("Initalize packet didnt serialize correctly".to_string()))),
        };

        self.send_packet(packet.clone()).await?;
        let response = self.recieve::<CommandResponse>().await?;

        if let CommandResponse::FrcInitialize(ref res) = response {
            if res.error_id != 0 {
                self.log_message(format!("Error ID: {}", res.error_id)).await;
                return Err(Box::new(FrcError::FanucErrorCode(res.error_id)));
            }
        }
        Ok(())

    }
    
        
    pub async fn abort(&self) -> Result<(), Box<dyn Error>> {

        let packet = Command::FrcAbort {};
        
        let packet = match serde_json::to_string(&packet) {
            Ok(serialized_packet) => serialized_packet + "\r\n",
            Err(_) => return Err(Box::new(FrcError::Serialization("Abort packet didnt serialize correctly".to_string()))),
        };

        self.send_packet(packet.clone()).await?;
        let response = self.recieve::<CommandResponse>().await?;

        if let CommandResponse::FrcAbort(ref res) = response {
            if res.error_id != 0 {
                self.log_message(format!("Error ID: {}", res.error_id)).await;
                return Err(Box::new(io::Error::new(io::ErrorKind::Interrupted, format!("Fanuc threw a Error #{} on a abort packet", res.error_id))));
            }
        }
        Ok(())
    }

    pub async fn get_status(&self) -> Result<(), Box<dyn Error>> {

        let packet = Command::FrcGetStatus {};
        
        let packet = match serde_json::to_string(&packet) {
            Ok(serialized_packet) => serialized_packet + "\r\n",
            Err(_) => return Err(Box::new(FrcError::Serialization("get_status packet didnt serialize correctly".to_string()))),
        };

        self.send_packet(packet.clone()).await?;
        let response = self.recieve::<CommandResponse>().await?;        
        if let CommandResponse::FrcGetStatus(ref res) = response {
            if res.error_id != 0 {
                self.log_message(format!("Error ID: {}", res.error_id)).await;
                return Err(Box::new(io::Error::new(io::ErrorKind::Interrupted, format!("Fanuc threw a Error #{} on a FrcGetStatus return packet", res.error_id))));
            }
        }
        Ok(())
    }

    pub async fn disconnect(&self) -> Result<(), Box<dyn Error>> {

        let packet = Communication::FrcDisconnect {};
        
        let packet = match serde_json::to_string(&packet) {
            Ok(serialized_packet) => serialized_packet + "\r\n",
            Err(_) => return Err(Box::new(FrcError::Serialization("Disconnect packet didnt serialize correctly".to_string()))),
        };

        self.send_packet(packet.clone()).await?;
        let response = self.recieve::<CommunicationResponse>().await?;        
        if let CommunicationResponse::FrcDisconnect(ref res) = response {
            if res.error_id != 0 {
                self.log_message(format!("Error ID: {}", res.error_id)).await;
                return Err(Box::new(io::Error::new(io::ErrorKind::Interrupted, format!("Fanuc threw a Error #{} on a Disconect packet", res.error_id))));
            }
        }

        Ok(())

    }

    async fn send_packet(&self, packet: String) -> Result<(), Box<dyn Error>> {      
            let mut stream = self.write_half.lock().await;
            stream.write_all(packet.as_bytes()).await?;
            self.log_message(format!("Sent: {}", packet)).await;
            Ok(())
    }

    async fn recieve<T>(&self) -> Result<T, Box<dyn Error>>
        where
            T: for<'a> Deserialize<'a> + std::fmt::Debug,
        {
            
            let mut buffer = vec![0; 2048];
            let mut stream = self.read_half.lock().await;

            let n = stream.read(&mut buffer).await?;
            if n == 0 {
                return Err(Box::new(io::Error::new(io::ErrorKind::Other, "Connection closed by peer")));
            }

            let response = String::from_utf8_lossy(&buffer[..n]);

            self.log_message(format!("Received: {}", &response)).await;

            // Parse JSON response
            match serde_json::from_str::<T>(&response) {
                Ok(response_packet) => Ok(response_packet),
                Err(e) => {
                    self.log_message(format!("Could not parse response: {}", e)).await;
                    Err(Box::new(io::Error::new(io::ErrorKind::Other, "could not parse response")))
                }
            }
        }

    pub async fn linear_relative(
        &self,
        sequenceid: u32,    
        config: Configuration,
        pos: Position,
        speed_t: SpeedType,
        speed: u16,
        term_t: TermType,
        term_va: u8,

    ) -> Result<(), Box<dyn Error>> {
        let packet = Instruction::FrcLinearRelative(FrcLinearRelative::new(
            sequenceid,    
            config,
            pos,
            speed_t,
            speed,
            term_t,
            term_va,

        ));
        
        let packet = match serde_json::to_string(&packet) {
            Ok(serialized_packet) => serialized_packet + "\r\n",
            Err(_) => return Err(Box::new(FrcError::Serialization("linear motion packet didnt serialize correctly".to_string()))),
        };

        self.send_packet(packet.clone()).await?;
        let response = self.recieve::<InstructionResponse>().await?;
        if let InstructionResponse::FrcLinearRelative(ref res) = response {
            if res.error_id != 0 {
                self.log_message(format!("Error ID: {}", res.error_id)).await;
                return Err(Box::new(io::Error::new(io::ErrorKind::Interrupted, format!("Fanuc threw a Error #{} on a linear motion on return packet", res.error_id))));
            }
        }
        Ok(())

    }

    pub fn load_gcode(&self) -> Result<VecDeque<PacketEnum>, Box<dyn Error>> {
        let mut queue: VecDeque<PacketEnum> = VecDeque::new();
        queue.push_back(PacketEnum::Instruction(Instruction::FrcLinearRelative(FrcLinearRelative::new(
            1,    
                Configuration {
                    u_tool_number: 1, u_frame_number: 1, front: 1, up: 1, left: 1, glip: 1, turn4: 1, turn5: 1, turn6: 1,
                },
                Position { x: 0.0, y: 0.0, z: 100.0, w: 0.0, p: 0.0, r: 0.0, ext1: 0.0, ext2: 0.0, ext3: 0.0,
                },
                SpeedType::MMSec,
                30,
                TermType::FINE,
                1,
        ))));
        queue.push_back(PacketEnum::Instruction(Instruction::FrcLinearRelative(FrcLinearRelative::new(
            2,    
            Configuration {
                u_tool_number: 1, u_frame_number: 1, front: 1, up: 1, left: 1, glip: 1, turn4: 1, turn5: 1, turn6: 1,
            },
            Position { x: 30.0, y: 100.0, z: 0.0, w: 0.0, p: 0.0, r: 0.0, ext1: 0.0, ext2: 0.0, ext3: 0.0,
            },
            SpeedType::MMSec,
            30,
            TermType::FINE,
            1,
        ))));
        queue.push_back(PacketEnum::Instruction(Instruction::FrcLinearRelative(FrcLinearRelative::new(
                3,    
                Configuration { u_tool_number: 1, u_frame_number: 1, front: 1, up: 1, left: 1, glip: 1, turn4: 1, turn5: 1, turn6: 1,
                },
                Position { x: 0.0, y: 0.0, z: -100.0, w: 0.0, p: 0.0, r: 0.0, ext1: 0.0, ext2: 0.0, ext3: 0.0,
                },
                SpeedType::MMSec,
                30,
                TermType::FINE,
                1,
        ))));
        queue.push_back(PacketEnum::Instruction(Instruction::FrcLinearRelative(FrcLinearRelative::new(
                4,    
                Configuration { u_tool_number: 1, u_frame_number: 1, front: 1, up: 1, left: 1, glip: 1, turn4: 1, turn5: 1, turn6: 1,
                },
                Position { x: -30.0, y: -100.0, z: 0.0, w: 0.0, p: 0.0, r: 0.0, ext1: 0.0, ext2: 0.0, ext3: 0.0,
                },
                SpeedType::MMSec,
                30,
                TermType::FINE,
                1,
        ))));
        Ok(queue)
    }

    pub async fn start_program(&self) -> Result<(), Box<dyn Error>> {

        let mut queue = self.load_gcode()?; // Handle synchronous load_gcode
        let (tx, rx) = mpsc::channel(100); // Create a channel with a buffer size of 100

        //spins up 2 async concurent functions
        let (res1, res2) = tokio::join!(
            self.send_queue(&mut queue, tx),
            self.read_queue_responses(rx)
        );
        
        match res1 {
            Ok(_) => self.log_message("send_queue completed successfully").await,
            Err(e) => self.log_message(format!("send_queue failed: {}", e)).await,
        }

        match res2 {
            Ok(_) => self.log_message("read_queue_responses completed successfully").await,
            Err(e) => self.log_message(format!("read_queue_responses failed: {}", e)).await,
        }

        Ok(())
    }

    async fn send_queue(&self, queue: &mut VecDeque<PacketEnum>, tx: mpsc::Sender<u32>)-> Result<(), Box<dyn Error>>{
        while !queue.is_empty() {
            let packet = queue.pop_front();
            
            //all this match statement does is extract the sequence id from the packet about to be sent, it may not be neccesary later
            let sequence_id = match &packet.as_ref().unwrap() {
                PacketEnum::Instruction(instruction) => {
                    match instruction {
                        Instruction::FrcLinearRelative(packet) => packet.sequence_id,
                        // Handle other instruction types similarly if needed
                        _ => 0, // Use a default value if sequence_id is not applicable
                    }
                },
                _ => 0, // Use a default value for non-instruction packets
            };

            tx.send(sequence_id).await.expect("Failed to send message");

            let packet = match serde_json::to_string(&packet) {
                Ok(serialized_packet) => serialized_packet + "\r\n",
                Err(e) => {
                    self.log_message(format!("Failed to serialize a packet: {}", e)).await;
                    break;
                }
            };
            self.send_packet(packet).await?;
        
            // sleep(Duration::from_millis(1)).await;
            
        }
        self.log_message("Sent all packets").await;

        //when 0 is sent it shuts  off the recciever system so we wait one sec so that the response can be sent back and processed
        sleep(Duration::from_secs(1)).await;

        tx.send(0).await.expect("Failed to send end message");

        Ok(())
    }
    

    async fn read_queue_responses(&self, mut rx: mpsc::Receiver<u32>) -> Result<(), Box<dyn Error>> {
        
        let mut reader = self.read_half.lock().await;

        let mut numbers_to_look_for: VecDeque<u32> = VecDeque::new();
        let mut buffer = vec![0; 2048];
        let mut temp_buffer = Vec::new();

        loop {
            tokio::select! {
                result = reader.read(&mut buffer) => {
                    match result {
                        Ok(0) => break, // Connection closed
                        Ok(n) => {
                            // Append new data to temp_buffer
                            temp_buffer.extend_from_slice(&buffer[..n]);

                            while let Some(pos) = temp_buffer.iter().position(|&x| x == b'\n') {
                                // Split the buffer into the current message and the rest
                                let request: Vec<u8> = temp_buffer.drain(..=pos).collect();
                                // Remove the newline character
                                let request = &request[..request.len() - 1];

                                let response_str = String::from_utf8_lossy(request);
                                self.log_message(response_str.clone()).await;

                                let response_packet: Option<InstructionResponse> = match serde_json::from_str::<InstructionResponse>(&response_str) {
                                    Ok(response_packet) => Some(response_packet),
                                    Err(e) => {
                                        self.log_message(format!("Could not parse response: {}", e)).await;
                                        None
                                    }
                                };

                                if let Some(response_packet) = response_packet {
                                    let sequence_id = response_packet.get_sequence_id();
                                    self.log_message(format!("Found matching id: {}", sequence_id)).await;
                                    numbers_to_look_for.retain(|&x| x != sequence_id);
                                }
                            }
                        }
                        Err(e) => {
                            self.log_message(format!("Failed to read from stream: {}", e)).await;
                        }
                    }
                },
                Some(message) = rx.recv() => {
                    if message == 0 {
                        break;
                    } else {
                        numbers_to_look_for.push_back(message);
                    }
                }
            }
        }
        
        Ok(())
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