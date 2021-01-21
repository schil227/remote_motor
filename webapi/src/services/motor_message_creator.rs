use std::cmp::min;

use models::{MotorMessage, MotorData, MotorCommand};
use models::{CLAW_DATA, HAND_DATA, FOREARM_DATA, STRONGARM_DATA, SHOULDER_DATA};

use crate::CommandData;

pub struct MotorMessageCreator{
}

impl MotorMessageCreator{
    pub fn get_messages(data: CommandData) -> Vec<MotorMessage>{
        let foo = vec![
            MotorMessage{
                data: CLAW_DATA,
                command: MotorCommand::Go(get_adjusted_value(min(data.claw, 100), CLAW_DATA))
            },
            MotorMessage{
                data: HAND_DATA,
                command: MotorCommand::Go(get_adjusted_value(min(data.hand, 100), HAND_DATA))
            },
            MotorMessage{
                data: FOREARM_DATA,
                command: MotorCommand::Go(get_adjusted_value(min(data.forearm, 100), FOREARM_DATA))
            },
            MotorMessage{
                data: STRONGARM_DATA,
                command: MotorCommand::Go(get_adjusted_value(min(data.strongarm, 100), STRONGARM_DATA))
            },
            MotorMessage{
                data: SHOULDER_DATA,
                command: MotorCommand::Go(get_adjusted_value(min(data.shoulder, 100), SHOULDER_DATA))
            }
        ].into_iter().filter(|x| command_is_doing_something(x.command)).collect();

        foo
    }
}

pub fn command_is_doing_something(motor_command: MotorCommand) -> bool{
        match motor_command {
            MotorCommand::Go(position) =>{
                if position == 0 {
                    return false
                }

                true
            },
            MotorCommand::Stop() => {
                true
            }
        }
}

fn get_adjusted_value(value: u8, motor_data: MotorData) -> u16 {
    if value == 0
    {
        return 0
    }

    (((motor_data.max - motor_data.min) * value as u16) / 100 + motor_data.min) as u16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_adjusted_value__tests(){
        let motor_data = MotorData{
            min: 100,
            max: 200,
            motor_name: models::MotorName::ALL
        };

        assert_eq!(get_adjusted_value(0, motor_data), 0);
        assert_eq!(get_adjusted_value(1, motor_data), 101);
        assert_eq!(get_adjusted_value(50, motor_data), 150);
        assert_eq!(get_adjusted_value(100, motor_data), 200);
        assert_eq!(get_adjusted_value(25, motor_data), 125);
        assert_eq!(get_adjusted_value(75, motor_data), 175);
    }

    #[test]
    fn command_is_doing_something__when_go_with_position__returns_true(){
        let c = MotorCommand::Go(1);

        assert!(command_is_doing_something(c));
    }

    #[test]
    fn command_is_doing_something__when_go_with_zero_position__returns_false(){
        let c = MotorCommand::Go(0);

        assert!(!command_is_doing_something(c));
    }

    #[test]
    fn command_is_doing_something__when_stop_command__returns_true(){
        let c = MotorCommand::Stop();

        assert!(command_is_doing_something(c));
    }
}