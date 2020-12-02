use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::env;

use models::MotorMessage;
use services::config_reader::ConnectionConfig;

const SIZE : usize = std::mem::size_of::<MotorMessage>();

pub fn send_command(shared_command: Arc<Mutex<MotorMessage>>){
    let config = get_config_data();

    let client = UdpSocket::bind(&config.this_machine_binding).expect("Failed to bind client UDP socket.");

    let mut previous_message: MotorMessage;

    {
        let tmp_command = shared_command.lock().expect("Failed to lock command!");
        previous_message = *tmp_command;
    }

    loop
    {
        let mut buf = [0; SIZE + 10];

        {
            let current_command = shared_command.lock().expect("Failed to lock command!");

            if previous_message == *current_command{
                continue;
            }
            else{
                previous_message = *current_command;
            }

            bincode::serialize_into(&mut buf[..], &*current_command).expect("Failed to serialize direction!");
        }

        println!("Sending message: len: {}, {:?}", &buf.len(), &buf);

        client.send_to(&buf, &config.target_machine_binding).expect("Failed to send message!");

        println!("Sent.");
    }
}

fn get_config_data() -> ConnectionConfig{
    let working_directory = env::current_dir().unwrap().into_os_string().into_string().unwrap();

    let conf_file = format!("{}\\config.json", working_directory);

    ConnectionConfig::get_connection_config(conf_file.as_str())
}