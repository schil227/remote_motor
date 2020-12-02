mod input_monitor;
mod message_sender;
mod motor_constatnts;

use std::thread;
use std::sync::{Arc, Mutex};

use motor_constatnts::MotorConstants;

fn main() {
    let message = MotorConstants::stop_everything();

    let command = Arc::new(Mutex::new(message));
    let other_command = Arc::clone(&command);
    
    let message_sender_handler = thread::spawn(move || message_sender::send_command(Arc::clone(&command)));

    let input_monitor_handler = thread::spawn(move ||input_monitor::listen_for_command(Arc::clone(&other_command)));

    println!("Running.");

    message_sender_handler.join().unwrap();
    input_monitor_handler.join().unwrap();
}