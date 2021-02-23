use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct CommandData{
    pub claw: u8,
    pub hand: u8,
    pub forearm: u8,
    pub strongarm: u8,
    pub shoulder: u8
}

impl CommandData {
    pub fn copy_from(&mut self, other : &CommandData) {
        self.claw = match other.claw { 0 => self.claw, _ => other.claw};
        self.hand = match other.hand { 0 => self.hand, _ => other.hand};
        self.forearm = match other.forearm { 0 => self.forearm, _ => other.forearm};
        self.strongarm = match other.strongarm { 0 => self.strongarm, _ => other.strongarm};
        self.shoulder = match other.shoulder { 0 => self.shoulder, _ => other.shoulder};
    }
}