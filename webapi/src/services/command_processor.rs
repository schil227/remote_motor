use crate::models::command_models::CommandData;

use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::services::command_sender::CommandSender;
use crate::services::motor_message_creator::MotorMessageCreator;
use crate::services::user_service::UserService;
use crate::services::websocket_service::{WebSocketServer, ServerState};
use crate::services::command_aggregator;

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

    pub fn run(&mut self, websocket_server_lock : Arc<Mutex<WebSocketServer>>) {
        loop {
            thread::sleep(Duration::from_secs(10));

            log::info!("Warning: Input lock imminent");
            set_server_state(&websocket_server_lock, ServerState::Warning);

            thread::sleep(Duration::from_secs(5));

            log::info!("Warning: Input Locked");
            set_server_state(&websocket_server_lock,ServerState::Locked);

            let data = self.user_service.flush_commands();
            let num_commands = data.len();
            
            if num_commands == 0 {
                log::info!("No commands to process.");

                set_server_state(&websocket_server_lock, ServerState::AcceptingInput);

                continue;
            }

            log::info!("Found {} to process", num_commands);

            let average_command_data = command_aggregator::aggregate_commands(data);

            log::info!("Averaged Commands: {:?}", &average_command_data);

            set_command_data(&websocket_server_lock, &average_command_data);

            let aggregate_messages = MotorMessageCreator::get_messages(average_command_data);

            log::info!("Sending commands.");

            let goal_count_result = self.sender.send_commands(aggregate_messages);

            // wait a second so process accounts for robot moving
            thread::sleep(Duration::from_secs(2));

            set_server_state(&websocket_server_lock, ServerState::AcceptingInput);

            match goal_count_result {
                Ok(count) => {
                    println!(" =======> goal count: {}", count);
                    set_goal_count(&websocket_server_lock, count);
                    
                    log::info!("Commands sent.");
                },
                Err(()) => {
                    println!("no goal!");
                    log::error!("Commands failed to send.");
                }
            }
        }
    }
}

fn set_server_state(websocket_server_lock : &Arc<Mutex<WebSocketServer>>, state : ServerState){
    let mut websocket_server = websocket_server_lock.lock().unwrap();
    websocket_server.set_server_state(state);
}

fn set_command_data(websocket_server_lock : &Arc<Mutex<WebSocketServer>>, commands : &CommandData){
    let mut websocket_server = websocket_server_lock.lock().unwrap();
    websocket_server.set_command_data(&commands);
}

fn set_goal_count(websocket_server_lock : &Arc<Mutex<WebSocketServer>>, goal_count : u8){
    let mut websocket_server = websocket_server_lock.lock().unwrap();
    websocket_server.set_goal_count(goal_count);
}