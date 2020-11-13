use std::net::UdpSocket;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

use models::MotorCommand;

const SIZE : usize = std::mem::size_of::<MotorCommand>();

pub fn send_command(shared_command: Arc<Mutex<MotorCommand>>){
    let client = UdpSocket::bind("127.0.0.1:7879").expect("Failed to bind client UDP socket.");

    loop
    {
        let mut buf = [0; SIZE + 10];

        {
            let current_command = shared_command.lock().expect("Failed to lock command!");

            bincode::serialize_into(&mut buf[..], &*current_command).expect("Failed to serialize direction!");
        }

        println!("Sending message: len: {}, {:?}", &buf.len(), &buf);

        client.send_to(&buf, "127.0.0.1:7870").expect("Failed to send message!");

        println!("Sent.");

        thread::sleep(Duration::from_secs(3));
    }
}