use models::MotorMessage;

use std::net::UdpSocket;

pub struct CommandSender{
    client: UdpSocket,
    robot_ip_address: String
}

const SIZE : usize = std::mem::size_of::<MotorMessage>();

impl CommandSender{
    pub fn new(client: UdpSocket, robot_ip_address: String) -> CommandSender {
        CommandSender{
            client,
            robot_ip_address
        }
    }
    
    pub fn send_commands(&self, command_messages: Vec<MotorMessage>){
        for message in command_messages.into_iter(){
            let mut buf = [0; SIZE + 10];
    
            bincode::serialize_into(&mut buf[..], &message).expect("Failed to serialize direction!");
        
            println!("Sending message: len: {}, {:?}", &buf.len(), &buf);
        
            self.client.send_to(&buf, &self.robot_ip_address).expect("Failed to send message!");
        }
    }
}