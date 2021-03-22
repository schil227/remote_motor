use models::MotorMessage;
use models::{CLAW_DATA, HAND_DATA, FOREARM_DATA, STRONGARM_DATA, SHOULDER_DATA, STOP_EVERYTHING};
use models::MotorCommand;
use models::MotorName;

pub struct MotorMessageConstants{
}

impl MotorMessageConstants{
    pub fn open_claw() -> MotorMessage{
        MotorMessage{
            data: CLAW_DATA,
            command: MotorCommand::Go(CLAW_DATA.max)
        }
    }

    pub fn close_claw() -> MotorMessage{
        MotorMessage{
            data: CLAW_DATA,
            command: MotorCommand::Go(CLAW_DATA.min)
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
            command: MotorCommand::Go(HAND_DATA.max)
        }
    }

    pub fn close_hand() -> MotorMessage{
        MotorMessage{
            data: HAND_DATA,
            command: MotorCommand::Go(HAND_DATA.min)
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
            command: MotorCommand::Go(FOREARM_DATA.max)
        }
    }

    pub fn contract_fore_arm() -> MotorMessage{
        MotorMessage{
            data: FOREARM_DATA,
            command: MotorCommand::Go(FOREARM_DATA.max)
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
            command: MotorCommand::Go(STRONGARM_DATA.max)
        }
    }

    pub fn contract_strong_arm() -> MotorMessage{
        MotorMessage{
            data: STRONGARM_DATA,
            command: MotorCommand::Go(STRONGARM_DATA.min)
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
            command: MotorCommand::Go(SHOULDER_DATA.max)
        }
    }

    pub fn rotate_shoulder_counter_clockwise() -> MotorMessage{
        MotorMessage{
            data: SHOULDER_DATA,
            command: MotorCommand::Go(SHOULDER_DATA.min)
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