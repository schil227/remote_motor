use models::MotorMessage;
use models::MotorData;
use models::MotorCommand;

static CLAW_DATA: MotorData = MotorData{
    gpio_pin: 24,
    max: 0.12,
    min: 0.02
};

static HAND_DATA: MotorData = MotorData{
    gpio_pin: 17,
    max: 0.12,
    min: 0.02
};

static FOREARM_DATA: MotorData = MotorData{
    gpio_pin: 18,
    max: 0.12,
    min: 0.02
};

static STRONGARM_DATA: MotorData = MotorData{
    gpio_pin: 27,
    max: 0.12,
    min: 0.02
};

static SHOULDER_DATA: MotorData = MotorData{
    gpio_pin: 22,
    max: 0.12,
    min: 0.02
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
}