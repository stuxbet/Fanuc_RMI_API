use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FrcInitialize {
    group_mask: u8,
}


impl FrcInitialize{
    fn new(groupmask: Option<u8>) -> Self {
        let groupmask = match groupmask {
            Some(gm) => gm,
            None => 1
        };

        Self {
            group_mask: groupmask
        }

    }
}

impl Default for FrcInitialize {
    fn default() -> Self {
        FrcInitialize::new(Some(1))
    }
}