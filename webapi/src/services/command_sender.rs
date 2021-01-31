use models::{MotorMessage};

use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

pub struct CommandSender{
    robot_ip_address: String
}

const SIZE : usize = std::mem::size_of::<MotorMessage>();

impl CommandSender{
    pub fn new(
        robot_ip_address: String) -> CommandSender {
        CommandSender{
            robot_ip_address
        }
    }
    
    pub fn send_commands(&self, command_messages: Vec<MotorMessage>){
       
        let mut stream = match TcpStream::connect("192.168.1.186:7870"){
            Ok(stream) => {
                stream
            },
            Err(e) => {
                println!("Failed establishing TCP connection to the server. Ensure the server is listening.");
                return;
            }
        };

        stream.set_read_timeout(Some(Duration::from_secs(3))).expect("Could not set read duration for TcpStream.");
        println!("connection established!");

        let num_commands : u8 = command_messages.len() as u8;
        
        // bincode::serialize_into(&mut byte[..], &14).expect("Failed to serialize");
        stream.write(&num_commands.to_be_bytes()).expect("Failed ending bytes");
       
        if !receive_ack(&stream) {
            println!("Did not get ACK response from server. Canceling request.");
            return;
        }

        for message in command_messages.into_iter(){
            let mut buf = [0; SIZE + 10];
    
            bincode::serialize_into(&mut buf[..], &message).expect("Failed to serialize direction!");
        
            println!("Sending command: len: {}, {:?}", &buf.len(), &buf);
        
            stream.write(&buf).expect("Failed to send command!");

            if !receive_ack(&stream) {
                println!("Did not get ACK response from server. Some commands sent. Canceling request.");
                return;
            }
        }
    }
}

fn receive_ack(mut stream : &TcpStream) -> bool{
    let mut response = [0;1];
    stream.read(&mut response).expect("Failed getting response.");
   
    let ack : u8 = bincode::deserialize(&response).expect("Failed deserializing ACK");

    return ack == 1;
}