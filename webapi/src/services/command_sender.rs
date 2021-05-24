use models::{MotorMessage};

use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

pub struct CommandSender{
    robot_ip_address: String
}

const SIZE : usize = std::mem::size_of::<MotorMessage>();

impl CommandSender{
    pub fn new(robot_ip_address: String) -> CommandSender {
        CommandSender{
            robot_ip_address
        }
    }
    
    pub fn send_commands(&self, command_messages: Vec<MotorMessage>) -> Result<u8, ()>{
        let mut stream = match TcpStream::connect(&self.robot_ip_address){
            Ok(stream) => {
                stream
            },
            Err(_) => {
                log::error!("Failed establishing TCP connection to the server. Ensure the server is listening.");
                return Result::Err(());
            }
        };

        stream.set_read_timeout(Some(Duration::from_secs(3))).expect("Could not set read duration for TcpStream.");
        log::info!("connection established!");

        let num_commands : u8 = command_messages.len() as u8;
        
        // bincode::serialize_into(&mut byte[..], &14).expect("Failed to serialize");
        stream.write(&num_commands.to_be_bytes()).expect("Failed ending bytes");
       
        if !receive_ack(&stream) {
            log::error!("Did not get ACK response from server. Canceling request.");
            return Result::Err(());
        }

        for message in command_messages.into_iter(){
            let mut buf = [0; SIZE + 10];
    
            bincode::serialize_into(&mut buf[..], &message).expect("Failed to serialize direction!");
        
            log::info!("Sending command: len: {}, {:?}", &buf.len(), &buf);
        
            stream.write(&buf).expect("Failed to send command!");

            if !receive_ack(&stream) {
                log::error!("Did not get ACK response from server - (some) commands not sent. Canceling request.");
                return Result::Err(());
            }
        }

        let goal_count = receive_u8(&stream);

        return Result::Ok(goal_count);
    }
}

fn receive_ack(stream : &TcpStream) -> bool{
    return receive_u8(stream) == 1;
}

fn receive_u8(mut stream: &TcpStream) -> u8{
    let mut response = [0;1];
    stream.read(&mut response).expect("Failed getting response.");
   
    bincode::deserialize(&response).expect("Failed deserializing u8")
}
