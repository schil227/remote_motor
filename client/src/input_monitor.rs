use std::sync::{Arc, Mutex};

use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use models::MotorMessage;

use crate::motor_constatnts::MotorConstants;

pub fn listen_for_command(shared_command: Arc<Mutex<MotorMessage>>) {
    enable_raw_mode().unwrap();

    loop
    {
        match read().unwrap(){
            // CLAW 
            Event::Key(KeyEvent { code: KeyCode::Char('q'), modifiers: _ }) =>{
                println!("Opening Claw!");
                let mut message = shared_command.lock().unwrap();
                (*message) = MotorConstants::open_claw();
            },
            Event::Key(KeyEvent { code: KeyCode::Char('e'), modifiers: _ }) =>{
                println!("Closing Claw!");
                let mut message = shared_command.lock().unwrap();
                (*message) = MotorConstants::close_claw();
            },
            Event::Key(KeyEvent { code: KeyCode::Char('w'), modifiers: _ }) =>{
                println!("Stopping Claw!");
                let mut message = shared_command.lock().unwrap();
                (*message) = MotorConstants::stop_claw();
            },

            // HAND 
            Event::Key(KeyEvent { code: KeyCode::Char('a'), modifiers: _ }) =>{
                println!("Opening Hand!");
                let mut message = shared_command.lock().unwrap();
                (*message) = MotorConstants::open_hand();
            },
            Event::Key(KeyEvent { code: KeyCode::Char('d'), modifiers: _ }) =>{
                println!("Closing Hand!");
                let mut message = shared_command.lock().unwrap();
                (*message) = MotorConstants::close_hand();
            },
            Event::Key(KeyEvent { code: KeyCode::Char('s'), modifiers: _ }) =>{
                println!("Stopping Hand!");
                let mut message = shared_command.lock().unwrap();
                (*message) = MotorConstants::stop_hand();
            },

            // FOREARM 
            Event::Key(KeyEvent { code: KeyCode::Char('u'), modifiers: _ }) =>{
                println!("Extending Forearm!");
                let mut message = shared_command.lock().unwrap();
                (*message) = MotorConstants::extend_fore_arm();
            },
            Event::Key(KeyEvent { code: KeyCode::Char('o'), modifiers: _ }) =>{
                println!("Contracting Forearm!");
                let mut message = shared_command.lock().unwrap();
                (*message) = MotorConstants::contract_fore_arm();
            },
            Event::Key(KeyEvent { code: KeyCode::Char('i'), modifiers: _ }) =>{
                println!("Stoping Forearm!");
                let mut message = shared_command.lock().unwrap();
                (*message) = MotorConstants::stop_fore_arm();
            },

            // STRONGARM 
            Event::Key(KeyEvent { code: KeyCode::Char('j'), modifiers: _ }) =>{
            println!("Extending Strongarm!");
            let mut message = shared_command.lock().unwrap();
            (*message) = MotorConstants::extend_strong_arm();
            },
            Event::Key(KeyEvent { code: KeyCode::Char('l'), modifiers: _ }) =>{
                println!("Contracting Strongarm!");
                let mut message = shared_command.lock().unwrap();
                (*message) = MotorConstants::contract_strong_arm();
            },
            Event::Key(KeyEvent { code: KeyCode::Char('k'), modifiers: _ }) =>{
                println!("Stoping Strongarm!");
                let mut message = shared_command.lock().unwrap();
                (*message) = MotorConstants::stop_strong_arm();
            },
            
            // Shoulder 
            Event::Key(KeyEvent { code: KeyCode::Char('m'), modifiers: _ }) =>{
                println!("Rotating Shoulder Clockwise!");
                let mut message = shared_command.lock().unwrap();
                (*message) = MotorConstants::rotate_shoulder_clockwise();
            },
            Event::Key(KeyEvent { code: KeyCode::Char('.'), modifiers: _ }) =>{
                println!("Rotating Shoulder Counter-Clockwise!");
                let mut message = shared_command.lock().unwrap();
                (*message) = MotorConstants::rotate_shoulder_counter_clockwise();
            },
            Event::Key(KeyEvent { code: KeyCode::Char(','), modifiers: _ }) =>{
                println!("Stopping Shoulder!");
                let mut message = shared_command.lock().unwrap();
                (*message) = MotorConstants::stop_shoulder();
            }, 

            Event::Key(KeyEvent { code: KeyCode::Esc, modifiers: _}) |
            Event::Key(KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL})              
            =>{
                println!("Leaving!");
                disable_raw_mode().unwrap();
                std::process::exit(0);
            }
            _ => {
                println!("EMERGENCY STOP!");
                let mut message = shared_command.lock().unwrap();
                (*message) = MotorConstants::stop_everything();
            }
        }
    }
}