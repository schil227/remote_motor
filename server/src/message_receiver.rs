use super::motor_controller::MotorControlData;
use super::controller_master::ControllerMaster;

use std::thread;
use std::time::Duration;

use std::net::UdpSocket;
use models::MotorMessage;
use models::MotorCommand;

use services::config_reader::ConnectionConfig;

const SIZE : usize = std::mem::size_of::<MotorMessage>();

use rppal::gpio::Gpio;
use rppal::gpio::OutputPin;

pub fn listen(){
    let config = ConnectionConfig::get_connection_config_data();

    let client = UdpSocket::bind(&config.this_machine_binding).expect("Failed to bind client UDP socket.");

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

fn long_running_task(){
    let mut count : u32 = 1;
    let mut prev : u32 = 1;

    println!("Started task.");

    loop 
    {
        let tmp = prev;
        prev = count;
        count = count + tmp;

        thread::sleep(Duration::from_millis(20));

        if count > 100_000
        {
            break;
        }
    }

    println!("Finished task.")
}