use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct CommandData{
    pub claw: u8,
    pub hand: u8,
    pub forearm: u8,
    pub strongarm: u8,
    pub shoulder: u8
}