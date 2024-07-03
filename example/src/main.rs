use fanuc_rmi::drivers::FanucDriver;
// use fanuc_rmi::{Configuration, Position};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let mut driver = FanucDriver::default();
    // let mut driver = FanucDriver::new("192.168.1.100".to_string(), 16001);

    match driver.connect().await {
        Ok(_) => println!("Connected successfully"),
        Err(e) => println!("Failed to connect: {}", e),
    }
    // driver.get_status().await?;
    driver.initialize().await?;
    driver.start_proccess().await?;
    // driver.linear_motion(
    //     1,    
    //     Configuration {
    //         u_tool_number: 1,
    //         u_frame_number: 1,
    //         front: 1,
    //         up: 1,
    //         left: 1,
    //         glip: 1,
    //         turn4: 1,
    //         turn5: 1,
    //         turn6: 1,
    //     },
    //     Position {
    //         x: 0.0,
    //         y: 0.0,
    //         z: 0.0,
    //         w: 0.0,
    //         p: 0.0,
    //         r: 0.0,
    //         ext1: 0.0,
    //         ext2: 0.0,
    //         ext3: 0.0,
    //     },
    //     fanuc_rmi::SpeedType::MMSec,
    //     20,
    //     fanuc_rmi::TermType::CNT,
    //     1,
    // ).await?;

    // driver.linear_motion(
    //     2,    
    //     Configuration {
    //         u_tool_number: 1,
    //         u_frame_number: 1,
    //         front: 1,
    //         up: 1,
    //         left: 1,
    //         glip: 1,
    //         turn4: 1,
    //         turn5: 1,
    //         turn6: 1,
    //     },
    //     Position {
    //         x: 0.0,
    //         y: 100.0,
    //         z: 0.0,
    //         w: 0.0,
    //         p: 0.0,
    //         r: 0.0,
    //         ext1: 0.0,
    //         ext2: 0.0,
    //         ext3: 0.0,
    //     },
    //     fanuc_rmi::SpeedType::MMSec,
    //     20,
    //     fanuc_rmi::TermType::CNT,
    //     1,
        
    // ).await?;
    driver.abort().await?;
    driver.disconnect().await?;

    Ok(())
}
