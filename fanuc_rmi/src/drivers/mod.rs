use serde::Deserialize;
use serde::Serialize;
use tokio::sync::mpsc;
use tokio::task;
use std::{error::Error, io, sync::Arc, time::Duration};
use tokio::{ net::TcpStream, sync::Mutex, time::sleep};
use tokio::io::{ AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf, split, BufReader};
use std::collections::VecDeque;


use crate::packets::*;
use crate::instructions::*;
use crate::commands::*;
use crate::PacketEnum;
use crate::{Configuration, Position, SpeedType, TermType, FrcError };

pub struct FanucDriver {
    addr: String,
    initialize_port: u32,
    connection_port: Option<u32>,
    write_half: Option<Arc<Mutex<WriteHalf<TcpStream>>>>,
    read_half: Option<Arc<Mutex<ReadHalf<TcpStream>>>>,

}

impl FanucDriver {
    pub fn new(addr: String, initialize_port: u32) -> FanucDriver {
        Self {
            addr,
            initialize_port,
            connection_port: None,
            write_half: None,
            read_half:None,
        }
    }

    pub async fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        let init_addr = format!("{}:{}", self.addr, self.initialize_port);
        let stream = connect_with_retries(&init_addr, 3).await?;
        println!("Connected to the server at {}", init_addr);
        
        let (read_half, write_half) = split(stream);

        // Wrap the read and write halves in Arc<Mutex<>>
        self.read_half = Some(Arc::new(Mutex::new(read_half)));
        self.write_half = Some(Arc::new(Mutex::new(write_half)));

        // Create a connection packet
        let packet = Communication::FrcConnect {};
        
        let packet = match serde_json::to_string(&packet) {
            Ok(serialized_packet) => serialized_packet + "\r\n",
            Err(_) => return Err(Box::new(FrcError::Serialization("Communication: Connect packet didnt serialize correctly".to_string()))),
        };

        // Send a connection request packet to start the handshake
        self.send_packet(packet.clone()).await?;
        let response = self.recieve::<CommunicationResponse>().await?;

        //FIXME: this should prob have a defined behavior to handle not getting a port number back
        self.connection_port = match response {
            CommunicationResponse::FrcConnect(res) => Some(res.port_number),
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


                let (read_half, write_half) = split(stream);

                // Wrap the read and write halves in Arc<Mutex<>>
                self.read_half = Some(Arc::new(Mutex::new(read_half)));
                self.write_half = Some(Arc::new(Mutex::new(write_half)));



            },
            None => {return Err(Box::new(io::Error::new(io::ErrorKind::InvalidData, "Port number is missing in the response")));}
        }


        Ok(())
    }

    pub fn close_connection(&mut self) {
        self.read_half = None;
        self.write_half = None;
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
                println!("Error ID: {}", res.error_id);
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
                println!("Error ID: {}", res.error_id);
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
                println!("Error ID: {}", res.error_id);
                return Err(Box::new(io::Error::new(io::ErrorKind::Interrupted, format!("Fanuc threw a Error #{} on a FrcGetStatus return packet", res.error_id))));
            }
        }
        Ok(())
    }

    pub async fn disconnect(&mut self) -> Result<(), Box<dyn Error>> {

        let packet = Communication::FrcDisconnect {};
        
        let packet = match serde_json::to_string(&packet) {
            Ok(serialized_packet) => serialized_packet + "\r\n",
            Err(_) => return Err(Box::new(FrcError::Serialization("Disconnect packet didnt serialize correctly".to_string()))),
        };

        self.send_packet(packet.clone()).await?;
        let response = self.recieve::<CommunicationResponse>().await?;        
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
                println!("Error ID: {}", res.error_id);
                return Err(Box::new(io::Error::new(io::ErrorKind::Interrupted, format!("Fanuc threw a Error #{} on a linear motion on return packet", res.error_id))));
            }
        }
        Ok(())

    }

//new send and recieve abstractions to use on queue stystem to simplify
    async fn send_packet(&self, packet: String) -> Result<(), Box<dyn Error>> {
        if let Some(stream) = &self.write_half {
            let mut stream = stream.lock().await;
            stream.write_all(packet.as_bytes()).await?;
            println!("Sent: {}", packet);
            Ok(())
        } else {
            Err(Box::new(io::Error::new(
                io::ErrorKind::NotConnected,
                "Cannot send without a write stream",
            )))
        }
        // recieve()
    }


    async fn recieve<T>(&self) -> Result<T, Box<dyn Error>>
        where
            T: for<'a> Deserialize<'a> + std::fmt::Debug,
        {
            if let Some(stream) = &self.read_half {
                let mut buffer = vec![0; 2048];
                let mut stream = stream.lock().await;

                let n = stream.read(&mut buffer).await?;
                if n == 0 {
                    return Err(Box::new(io::Error::new(io::ErrorKind::Other, "Connection closed by peer")));
                }

                let response = String::from_utf8_lossy(&buffer[..n]);

                println!("Received: {}", &response);

                // Parse JSON response
                match serde_json::from_str::<T>(&response) {
                    Ok(response_packet) => Ok(response_packet),
                    Err(e) => {
                        println!("Could not parse response: {}", e);
                        Err(Box::new(io::Error::new(io::ErrorKind::Other, "could not parse response")))
                    }
                }
            } else {
                Err(Box::new(io::Error::new(
                    io::ErrorKind::NotConnected,
                    "Cannot receive without a read stream",
                )))
            }
        }



    fn load_gcode(&self) -> Result<VecDeque<PacketEnum>, Box<dyn Error>> {
        //here is where we will convert the gcode to the packets we need and return a queue

        //apply math magic to generate g code here


        let mut queue: VecDeque<PacketEnum> = VecDeque::new();
        queue.push_back(PacketEnum::Instruction(Instruction::FrcLinearRelative(FrcLinearRelative::new(
            1,    
                Configuration {
                    u_tool_number: 1,
                    u_frame_number: 1,
                    front: 1,
                    up: 1,
                    left: 1,
                    glip: 1,
                    turn4: 1,
                    turn5: 1,
                    turn6: 1,
                },
                Position {
                    x: 0.0,
                    y: 0.0,
                    z: 100.0,
                    w: 0.0,
                    p: 0.0,
                    r: 0.0,
                    ext1: 0.0,
                    ext2: 0.0,
                    ext3: 0.0,
                },
                SpeedType::MMSec,
                30,
                TermType::FINE,
                1,
        ))));
        queue.push_back(PacketEnum::Instruction(Instruction::FrcLinearRelative(FrcLinearRelative::new(
            2,    
            Configuration {
                u_tool_number: 1,
                u_frame_number: 1,
                front: 1,
                up: 1,
                left: 1,
                glip: 1,
                turn4: 1,
                turn5: 1,
                turn6: 1,
            },
            Position {
                x: 30.0,
                y: 100.0,
                z: 0.0,
                w: 0.0,
                p: 0.0,
                r: 0.0,
                ext1: 0.0,
                ext2: 0.0,
                ext3: 0.0,
            },
            SpeedType::MMSec,
            30,
            TermType::FINE,
            1,
        ))));
        queue.push_back(PacketEnum::Instruction(Instruction::FrcLinearRelative(FrcLinearRelative::new(
                3,    
                Configuration {
                    u_tool_number: 1,
                    u_frame_number: 1,
                    front: 1,
                    up: 1,
                    left: 1,
                    glip: 1,
                    turn4: 1,
                    turn5: 1,
                    turn6: 1,
                },
                Position {
                    x: 0.0,
                    y: 0.0,
                    z: -100.0,
                    w: 0.0,
                    p: 0.0,
                    r: 0.0,
                    ext1: 0.0,
                    ext2: 0.0,
                    ext3: 0.0,
                },
                SpeedType::MMSec,
                30,
                TermType::FINE,
                1,
        ))));
        queue.push_back(PacketEnum::Instruction(Instruction::FrcLinearRelative(FrcLinearRelative::new(
                4,    
                Configuration {
                    u_tool_number: 1,
                    u_frame_number: 1,
                    front: 1,
                    up: 1,
                    left: 1,
                    glip: 1,
                    turn4: 1,
                    turn5: 1,
                    turn6: 1,
                },
                Position {
                    x: -30.0,
                    y: -100.0,
                    z: 0.0,
                    w: 0.0,
                    p: 0.0,
                    r: 0.0,
                    ext1: 0.0,
                    ext2: 0.0,
                    ext3: 0.0,
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
            self.parse_path_responses(rx)
        );
        
        match res1 {
            Ok(_) => println!("send_queue completed successfully"),
            Err(e) => eprintln!("send_queue failed: {}", e),
        }
    
        match res2 {
            Ok(_) => println!("parse_path_responses completed successfully"),
            Err(e) => eprintln!("parse_path_responses failed: {}", e),
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
                    eprintln!("Failed to serialize a packet: {}", e);
                    break;
                }
            };
            self.send_packet(packet).await?;
        
            // sleep(Duration::from_millis(1)).await;
            
        }
        println!("Sent all packets");

        //when 0 is sent it shuts  off the recciever system so we wait one sec so that the response can be sent back and processed
        sleep(Duration::from_secs(1)).await;

        tx.send(0).await.expect("Failed to send end message");

        Ok(())
    
    
    }
    
    // async fn parse_path_responses(&self, mut rx: mpsc::Receiver<u32>)-> Result<(), Box<dyn Error>>{
    //     match &self.read_half {
    //         Some(read_stream) => {
    //             let mut reader = read_stream.lock().await;

    //             let mut numbers_to_look_for: VecDeque<u32> = VecDeque::new();
    //             let mut buffer = vec![0; 2048];

    //             loop {
    //                 tokio::select! {
    //                     result = reader.read(&mut buffer) => {
    //                         match result {
    //                             Ok(0) => break, // Connection closed
    //                             Ok(n) => {
    //                                 let response = String::from_utf8_lossy(&buffer[..n]);
    //                                 println!("Received {}", response);

    //                                 // let request_json: serde_json::Value = serde_json::from_str(&response)?;

    //                                 let response_packet: Option<InstructionResponse> = match serde_json::from_str::<InstructionResponse>(&response) {
    //                                     Ok(response_packet) => Some(response_packet),
    //                                     Err(e) => {
    //                                         println!("Could not parse response: {}", e);
    //                                         None
    //                                     }
    //                                 };
    //                                 let response_packet = response_packet.expect("no parsey parsey gringo");


    //                                 let sequence_id = response_packet.get_sequence_id();

    //                                 println!("Found matching id: {}", sequence_id);
    //                                 numbers_to_look_for.retain(|&x| x != sequence_id);
 

    //                             }
    //                             Err(e) => {
    //                                 eprintln!("Failed to read from stream: {}", e);
    //                             }
    //                         }
    //                     },
    //                     Some(message) = rx.recv() => {
    //                         if message == 0 {break;}
    //                         else{numbers_to_look_for.push_back(message);}
    //                     }
 
    //                 }
    //             }
    //         }
    //         None => {
    //             println!("No TcpStream available.");
    //             return Err(Box::new(io::Error::new(
    //                 io::ErrorKind::NotConnected,
    //                 "Cannot start program without initializing an open TCP stream",
    //             )));
    //         }            
    //     }
    //     Ok(())

    // }
    async fn parse_path_responses(&self, mut rx: mpsc::Receiver<u32>) -> Result<(), Box<dyn Error>> {
        match &self.read_half {
            Some(read_stream) => {
                let mut reader = read_stream.lock().await;

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
                                        println!("Received: {}", response_str);

                                        let response_packet: Option<InstructionResponse> = match serde_json::from_str::<InstructionResponse>(&response_str) {
                                            Ok(response_packet) => Some(response_packet),
                                            Err(e) => {
                                                println!("Could not parse response: {}", e);
                                                None
                                            }
                                        };

                                        if let Some(response_packet) = response_packet {
                                            let sequence_id = response_packet.get_sequence_id();
                                            println!("Found matching id: {}", sequence_id);
                                            numbers_to_look_for.retain(|&x| x != sequence_id);
                                        }
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Failed to read from stream: {}", e);
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
            }
            None => {
                println!("No TcpStream available.");
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotConnected,
                    "Cannot start program without initializing an open TCP stream",
                )));
            }
        }
        Ok(())
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