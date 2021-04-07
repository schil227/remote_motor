use crate::models::command_models::CommandData;

use std::thread;
use std::time::Duration;

use crate::services::command_sender::CommandSender;
use crate::services::motor_message_creator::MotorMessageCreator;
use crate::services::user_service::UserService;

pub struct CommandProcessor {
    sender : CommandSender,
    user_service: UserService
}

impl CommandProcessor {
    pub fn new(
        sender : CommandSender,
        user_service : UserService
    ) -> CommandProcessor {
        CommandProcessor{
            sender,
            user_service
        }
    }

    pub fn run(&mut self) {
        loop {
            thread::sleep(Duration::from_secs(10));

            let data = self.user_service.flush_commands();
            let num_commands = data.len();
            
            if num_commands == 0 {
                println!("No commands to process.");
                continue;
            }

            let mut command_parts : [usize; 5] = [0,0,0,0,0];

            for command in data.into_iter() {
                command_parts[0] += command.claw as usize;
                command_parts[1] += command.hand as usize;
                command_parts[2] += command.forearm as usize;
                command_parts[3] += command.strongarm as usize;
                command_parts[4] += command.shoulder as usize;
            }

            let command_parts : Vec<usize> = command_parts
                .into_iter()
                .map(|x| x / num_commands)
                .collect();

            let aggregate_messages = MotorMessageCreator::get_messages(CommandData{
                claw : command_parts[0] as u8,
                hand : command_parts[1] as u8,
                forearm : command_parts[2] as u8,
                strongarm : command_parts[3] as u8,
                shoulder : command_parts[4] as u8
            });

            self.sender.send_commands(aggregate_messages)
        }
    }
}