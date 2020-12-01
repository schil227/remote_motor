use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use models::MotorCommand;
use models::MotorData;
use models::MotorMessage;

use crate::motor_constatnts::MotorConstants;

pub fn listen_for_command(shared_command: Arc<Mutex<MotorMessage>>) {
    enable_raw_mode().unwrap();

    loop
    {
        match read().unwrap(){
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                modifiers: _
            }) =>{
                println!("Going left!");
                let mut message = shared_command.lock().unwrap();
                (*message) = MotorConstants::open_claw();
            },
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                modifiers: _
            }) =>{
                println!("Going right!");
                let mut message = shared_command.lock().unwrap();
                (*message) = MotorConstants::close_claw();
            },
            Event::Key(KeyEvent {
                code: KeyCode::Esc,
                modifiers: _
            }) |
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL
            })              
            =>{
                println!("Leaving!");
                disable_raw_mode().unwrap();
                std::process::exit(0);
            }
            _ => {
                println!("STOP!");
                let mut message = shared_command.lock().unwrap();
                (*message) = MotorConstants::stop_claw();
            }
        }
    }
}