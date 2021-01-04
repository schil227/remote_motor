use super::motor_controller::MotorControlData;
use super::controller_master::ControllerMaster;

use std::thread;
use std::time::Duration;

use std::net::UdpSocket;
use models::MotorMessage;
use models::MotorCommand;
use models::MotorName;

use services::config_reader::ConnectionConfig;

const SIZE : usize = std::mem::size_of::<MotorMessage>();

pub fn listen(){
    // let config = ConnectionConfig::get_connection_config_data();

    // TODO: Fix, cause finding the config file is broken on the server for some reason.
    // let client = UdpSocket::bind(&config.this_machine_binding).expect("Failed to bind client UDP socket.");
    let client = UdpSocket::bind("192.168.1.38:7870").expect("Failed to bind client UDP socket.");

    println!("Connected!");

    let mut master = ControllerMaster::new();

    loop
    {
        let mut buf = [0; SIZE + 10];

        client.recv(&mut buf).expect("Failed receiving message.");

        let message : MotorMessage = bincode::deserialize(&buf).expect("Could not deserialize MotorDirection!");
        
        println!("Recieved a message: {:?}", message);

        // empty pin means to stop everything
        if message.data.motor_name == MotorName::ALL {
            master.command_all_controllers(MotorCommand::Stop());
            continue;
        }

        match master.get_controller(message.data.motor_name){
            Some(controller) => {controller.update(message.command)},
            None => {
                match MotorControlData::register(message.data, &master.pwm_handle){
                    Ok(controller) => {
                        controller.update(message.command);
                        master.add_controller(message.data.motor_name, controller);
                    },
                    Err(error) => { println!("{}",error); }
                };
            }
        }
    }
}