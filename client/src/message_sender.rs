use std::net::UdpSocket;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::env;

use models::MotorCommand;
use services::config_reader::ConnectionConfig;

const SIZE : usize = std::mem::size_of::<MotorCommand>();

pub fn send_command(shared_command: Arc<Mutex<MotorCommand>>){
    let working_directory = env::current_dir().unwrap().into_os_string().into_string().unwrap();

    let conf_file = format!("{}\\config.json", working_directory);


    let config = ConnectionConfig::get_connection_config(conf_file.as_str());

    let client = UdpSocket::bind(&config.this_machine_binding).expect("Failed to bind client UDP socket.");

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

        client.send_to(&buf, &config.target_machine_binding).expect("Failed to send message!");

        println!("Sent.");

        
    }
}