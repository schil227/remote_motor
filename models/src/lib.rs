use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum MotorCommand{
    Forward(u8),
    Backward(u8),
    Stop()
}