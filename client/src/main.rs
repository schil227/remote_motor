mod input_monitor;
mod message_sender;

use std::thread;
use std::sync::{Arc, Mutex};

use models::MotorCommand;



fn main() {
    let command = Arc::new(Mutex::new(MotorCommand::Stop()));
    let other_command = Arc::clone(&command);
    
    thread::spawn(move || message_sender::send_command(Arc::clone(&command)));

    input_monitor::listen_for_command(Arc::clone(&other_command));
}