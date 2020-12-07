use super::motor_controller::MotorControlData;
use super::controller_master::ControllerMaster;

use std::net::UdpSocket;
use models::MotorMessage;
use models::MotorCommand;

const SIZE : usize = std::mem::size_of::<MotorMessage>();

pub fn listen(){
    let client = UdpSocket::bind("192.168.1.226:7870").expect("Failed to bind client UDP socket.");

    println!("Connected!");

    let mut master = ControllerMaster::new();

    loop
    {
        let mut buf = [0; SIZE + 10];

        client.recv(&mut buf).expect("Failed receiving message.");

        let message : MotorMessage = bincode::deserialize(&buf).expect("Could not deserialize MotorDirection!");
        
        println!("Recieved a message: {:?}", message);

        // empty pin means to stop everything
        if message.data.gpio_pin == 0 {
            master.command_all_controllers(MotorCommand::Stop());
            continue;
        }

        match master.get_controller(message.data.gpio_pin){
            Some(controller) => {controller.update(message.command)},
            None => {
                match MotorControlData::register(message.data){
                    Ok(controller) => {
                        controller.update(message.command);
                        master.add_controller(message.data.gpio_pin, controller);
                    },
                    Err(error) => { println!("{}",error); }
                };
            }
        }
    }
}