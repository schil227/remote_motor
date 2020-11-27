use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
#[derive(PartialEq)]
pub enum MotorCommand{
    Forward(u8),
    Backward(u8),
    Stop()
}


#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
#[derive(PartialEq)]
pub struct MotorData{
    pub gpio_pin: u8,
    pub max: f64,
    pub min: f64
}


#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
#[derive(PartialEq)]
pub struct MotorMessage{
    pub data: MotorData,
    pub command: MotorCommand
}