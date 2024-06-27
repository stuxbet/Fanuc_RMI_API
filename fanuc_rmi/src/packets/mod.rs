mod command;
mod communication;
mod instruction;

pub use command::*;
pub use communication::*;
pub use instruction::*;

use serde::{Serialize, Deserialize};

pub trait Packet: Serialize + for<'de> Deserialize<'de> {}

