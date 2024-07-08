use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde_json::json;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;


// #[derive(Serialize, Deserialize, Debug)]
// struct ConnectResponse {
//     Communication: String,
//     PortNumber: Option<u16>,
//     MajorVersion: Option<u16>,
//     MinorVersion: Option<u16>,
// }

async fn handle_client(mut socket: TcpStream, new_port: Arc<Mutex<u16>>) -> Result<u16, Box<dyn Error + Send + Sync>> {
    let mut buffer = vec![0; 2048];
    let n = match socket.read(&mut buffer).await {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Failed to read from socket: {}", e);
            return Err(Box::new(e));
        }
    };
    
    if n == 0 {
        return Ok(0);
    }

    let request = String::from_utf8_lossy(&buffer[..n]);
    println!("Received on primary : {}", request);

    let request_json: serde_json::Value = serde_json::from_str(&request)?;

    let response_json = match request_json["Communication"].as_str() {
        Some("FRC_Connect") => {
            let port = {
                let mut port_lock = new_port.lock().await;
                *port_lock += 1;
                *port_lock
            };

            json!({
                "Communication": "FRC_Connect",
                "ErrorID": 1,
                "PortNumber": port,
                "MajorVersion": 1,
                "MinorVersion": 0,
            })
        }
        _ => json!({
            "Error": "Unknown command"
        }),
    };

    let response = serde_json::to_string(&response_json)? + "\r\n";
    socket.write_all(response.as_bytes()).await?;
    println!("Sent: {}", response);

    if let Some(port) = response_json["PortNumber"].as_u64() {
        println!("Port number for new connection: {}", port);
        return Ok(port as u16);
    }

    Err("Failed to parse port number".into())
}
async fn handle_secondary_client(mut socket: TcpStream) -> Result<(), Box<dyn Error + Send + Sync>> {
    // println!("Second client spawn");

    let mut buffer = vec![0; 1024];
    loop {
        let n = match socket.read(&mut buffer).await {
            Ok(n) => n,
            Err(e) => {
                eprintln!("Failed to read from socket: {}", e);
                return Err(Box::new(e));
            }
        };
        
        if n == 0 {
            println!("Client disconnected");
            break;
        }

        let request = String::from_utf8_lossy(&buffer[..n]);
        println!("Received on secondary port: {}", request);

        let request_json: serde_json::Value = serde_json::from_str(&request)?;

        let mut response_json = match request_json["Command"].as_str() {
            Some("FRC_Initialize") => json!({
                "Command": "FRC_Initialize",
                "ErrorID": 0,
                "GroupMask": 1
            }),
            Some("FRC_LinearMotion") => json!({
                "Status": "Motion started"
            }),
            Some("FRC_Abort") => json!({
                "Command": "FRC_Abort",
                "ErrorID": 0,
            }),
            _ => json!({}),
        };
        response_json = match request_json["Communication"].as_str() {
            Some("FRC_Disconnect") => json!({
                "Communication": "FRC_Disconnect",
                "ErrorID": 0,

            }),
            _ => response_json
        };
        response_json = match request_json["Instruction"].as_str() {
            Some("FRC_LinearMotion") => json!({
                "Instruction": "FRC_LinearMotion",
                "ErrorID": 0,
                "SequenceID": 1,

            }),
            _ => response_json
        };

        let response = serde_json::to_string(&response_json)? + "\r\n";
        socket.write_all(response.as_bytes()).await?;
        println!("Sent: {}", response);
    }

    Ok(())
}


async fn start_secondary_server(port: u16) -> Result<(), Box<dyn Error + Send + Sync>> {
    let addr = format!("0.0.0.0:{}", port);
    // let listener = TcpListener::bind(&addr).await;
    let listener = match TcpListener::bind(&addr).await {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to bind to address {}: {}", addr, e);
            return Err(Box::new(e));
        }
    };

    println!("Secondary server listening on port {}", port);

    loop {
        let (socket, _) = match listener.accept().await {
            Ok((socket, addr)) => (socket, addr),
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
                continue;
            }
        };

        tokio::spawn(async move {
            if let Err(e) = handle_secondary_client(socket).await {
                eprintln!("Error handling secondary client: {:?}", e);
            }
        });
    }
}

async fn start_server(port: u16) -> Result<(), Box<dyn Error + Send + Sync>> {
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;
    println!("Server listening on port {}", port);

    let new_port = Arc::new(Mutex::new(port + 1));

    loop {
        let (socket, _) = match listener.accept().await {
            Ok((socket, addr)) => (socket, addr),
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
                continue;
            }
        };

        let new_port = Arc::clone(&new_port);

        match handle_client(socket, new_port).await {
            Ok(port) if port != 0 => {
                println!("Starting secondary server on port {}", port);
                tokio::spawn(start_secondary_server(port));

            },
            Ok(_) => {},
            Err(e) => eprintln!("Failed to handle client: {:?}", e),
        };
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    start_server(16001).await?;
    Ok(())
}

