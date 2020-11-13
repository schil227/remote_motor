use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
#[derive(PartialEq)]
pub enum MotorCommand{
    Forward(u8),
    Backward(u8),
    Stop()
}