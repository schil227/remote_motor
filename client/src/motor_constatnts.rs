use models::MotorMessage;
use models::MotorData;
use models::MotorCommand;
use models::MotorName;

static CLAW_DATA: MotorData = MotorData{
    motor_name: MotorName::Claw,
    max: 286,
    min: 143
};

static HAND_DATA: MotorData = MotorData{
    motor_name: MotorName::Hand,
    max: 286,
    min: 102
};

static FOREARM_DATA: MotorData = MotorData{
    motor_name: MotorName::ForeArm,
    max: 266,
    min: 163
};

static STRONGARM_DATA: MotorData = MotorData{
    motor_name: MotorName::StrongArm,
    max: 246,
    min: 127
};

static SHOULDER_DATA: MotorData = MotorData{
    motor_name: MotorName::Shoulder,
    max: 492,
    min: 82
};

static STOP_EVERYTHING: MotorData = MotorData{
    motor_name: MotorName::ALL,
    max: 0,
    min: 0
};

pub struct MotorConstants{
}

impl MotorConstants{
    pub fn open_claw() -> MotorMessage{
        MotorMessage{
            data: CLAW_DATA,
            command: MotorCommand::Forward(5)
        }
    }

    pub fn close_claw() -> MotorMessage{
        MotorMessage{
            data: CLAW_DATA,
            command: MotorCommand::Backward(5)
        }
    }

    pub fn stop_claw() -> MotorMessage{
        MotorMessage{
            data: CLAW_DATA,
            command: MotorCommand::Stop()
        }
    }

    pub fn open_hand() -> MotorMessage{
        MotorMessage{
            data: HAND_DATA,
            command: MotorCommand::Forward(5)
        }
    }

    pub fn close_hand() -> MotorMessage{
        MotorMessage{
            data: HAND_DATA,
            command: MotorCommand::Backward(5)
        }
    }

    pub fn stop_hand() -> MotorMessage{
        MotorMessage{
            data: HAND_DATA,
            command: MotorCommand::Stop()
        }
    }

    pub fn extend_fore_arm() -> MotorMessage{
        MotorMessage{
            data: FOREARM_DATA,
            command: MotorCommand::Forward(5)
        }
    }

    pub fn contract_fore_arm() -> MotorMessage{
        MotorMessage{
            data: FOREARM_DATA,
            command: MotorCommand::Backward(5)
        }
    }

    pub fn stop_fore_arm() -> MotorMessage{
        MotorMessage{
            data: FOREARM_DATA,
            command: MotorCommand::Stop()
        }
    }

    pub fn extend_strong_arm() -> MotorMessage{
        MotorMessage{
            data: STRONGARM_DATA,
            command: MotorCommand::Forward(5)
        }
    }

    pub fn contract_strong_arm() -> MotorMessage{
        MotorMessage{
            data: STRONGARM_DATA,
            command: MotorCommand::Backward(5)
        }
    }

    pub fn stop_strong_arm() -> MotorMessage{
        MotorMessage{
            data: STRONGARM_DATA,
            command: MotorCommand::Stop()
        }
    }


    pub fn rotate_shoulder_clockwise() -> MotorMessage{
        MotorMessage{
            data: SHOULDER_DATA,
            command: MotorCommand::Forward(5)
        }
    }

    pub fn rotate_shoulder_counter_clockwise() -> MotorMessage{
        MotorMessage{
            data: SHOULDER_DATA,
            command: MotorCommand::Backward(5)
        }
    }

    pub fn stop_shoulder() -> MotorMessage{
        MotorMessage{
            data: SHOULDER_DATA,
            command: MotorCommand::Stop()
        }
    }

    pub fn stop_everything() -> MotorMessage{
        MotorMessage{
            data: STOP_EVERYTHING,
            command: MotorCommand::Stop()
        }
    }
}