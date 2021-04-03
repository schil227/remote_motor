use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use models::MotorMessage;
use services::config_reader::ConnectionConfig;

const SIZE: usize = std::mem::size_of::<MotorMessage>();

pub fn send_command(shared_command: Arc<Mutex<MotorMessage>>) {
    let mut previous_message: MotorMessage;

    {
        let tmp_command = shared_command.lock().expect("Failed to lock command!");
        previous_message = *tmp_command;
    }

    loop {
        let mut buf = [0; SIZE + 10];

        {
            let current_command = shared_command.lock().expect("Failed to lock command!");

            if previous_message == *current_command {
                continue;
            } else {
                previous_message = *current_command;
            }

            bincode::serialize_into(&mut buf[..], &*current_command)
                .expect("Failed to serialize direction!");
        }

        send_command_tcp(vec![previous_message])
    }
}

fn send_command_tcp(command_messages: Vec<MotorMessage>) {
    let mut stream = match TcpStream::connect("192.168.1.38:7870") {
        Ok(stream) => stream,
        Err(_) => {
            println!(
                "Failed establishing TCP connection to the server. Ensure the server is listening."
            );
            return;
        }
    };

    stream
        .set_read_timeout(Some(Duration::from_secs(3)))
        .expect("Could not set read duration for TcpStream.");
    println!("connection established!");

    let num_commands: u8 = command_messages.len() as u8;
    // bincode::serialize_into(&mut byte[..], &14).expect("Failed to serialize");
    stream
        .write(&num_commands.to_be_bytes())
        .expect("Failed ending bytes");
    if !receive_ack(&stream) {
        println!("Did not get ACK response from server. Canceling request.");
        return;
    }

    for message in command_messages.into_iter() {
        let mut buf = [0; SIZE + 10];

        bincode::serialize_into(&mut buf[..], &message).expect("Failed to serialize direction!");
        println!("Sending command: len: {}, {:?}", &buf.len(), &buf);
        stream.write(&buf).expect("Failed to send command!");

        if !receive_ack(&stream) {
            println!(
                "Did not get ACK response from server. Some commands sent. Canceling request."
            );
            return;
        }
    }
}

fn receive_ack(mut stream: &TcpStream) -> bool {
    let mut response = [0; 1];
    stream
        .read(&mut response)
        .expect("Failed getting response.");

    let ack: u8 = bincode::deserialize(&response).expect("Failed deserializing ACK");
    return ack == 1;
}
