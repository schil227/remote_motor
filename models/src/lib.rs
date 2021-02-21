use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
pub enum MotorCommand {
    Go(u16),
    Stop(),
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
pub enum MotorName {
    ALL,
    Shoulder,
    StrongArm,
    ForeArm,
    Hand,
    Claw,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
pub struct MotorData {
    pub motor_name: MotorName,
    pub max: u16,
    pub min: u16,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
pub struct MotorMessage {
    pub data: MotorData,
    pub command: MotorCommand,
}

pub static CLAW_DATA: MotorData = MotorData {
    motor_name: MotorName::Claw,
    max: 310,
    min: 143,
};

pub static HAND_DATA: MotorData = MotorData {
    motor_name: MotorName::Hand,
    max: 286,
    min: 102,
};

pub static FOREARM_DATA: MotorData = MotorData {
    motor_name: MotorName::ForeArm,
    max: 266,
    min: 168,
};

pub static STRONGARM_DATA: MotorData = MotorData {
    motor_name: MotorName::StrongArm,
    max: 300,
    min: 127,
};

pub static SHOULDER_DATA: MotorData = MotorData {
    motor_name: MotorName::Shoulder,
    max: 492,
    min: 82,
};

pub static STOP_EVERYTHING: MotorData = MotorData {
    motor_name: MotorName::ALL,
    max: 0,
    min: 0,
};
