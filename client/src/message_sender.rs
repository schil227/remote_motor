use std::net::UdpSocket;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

use models::MotorCommand;

const SIZE : usize = std::mem::size_of::<MotorCommand>();

pub fn send_command(shared_command: Arc<Mutex<MotorCommand>>){
    let client = UdpSocket::bind("127.0.0.1:7879").expect("Failed to bind client UDP socket.");

    let mut previous_command = MotorCommand::Stop();

    {
        let tmp_command = shared_command.lock().expect("Failed to lock command!");
        previous_command = *tmp_command;
    }

    loop
    {
        let mut buf = [0; SIZE + 10];

        {
            let current_command = shared_command.lock().expect("Failed to lock command!");

            if previous_command == *current_command{
                continue;
            }
            else{
                previous_command = *current_command;
            }

            bincode::serialize_into(&mut buf[..], &*current_command).expect("Failed to serialize direction!");
        }

        println!("Sending message: len: {}, {:?}", &buf.len(), &buf);

        client.send_to(&buf, "127.0.0.1:7870").expect("Failed to send message!");

        println!("Sent.");

        
    }
}