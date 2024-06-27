use fanuc_rmi::drivers::FanucDriver;
pub use serde::{Deserialize,Serialize};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let mut driver = FanucDriver::default();
    match driver.connect().await {
        Ok(_) => println!("Connected successfully"),
        Err(e) => println!("Failed to connect: {}", e),
    }
    Ok(())
}
