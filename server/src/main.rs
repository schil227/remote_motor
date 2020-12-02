mod float_comparison;
mod motor_controller;

use std::net::UdpSocket;
use models::MotorMessage;
use models::MotorCommand;
use rppal::gpio::Gpio;
use rppal::gpio::OutputPin;
use std::thread;
use std::time::Duration;
use motor_controller::MotorControlData;
use std::collections::HashMap;

const SIZE : usize = std::mem::size_of::<MotorMessage>();

struct ControllerMaster{
    motor_controllers: HashMap<u8, MotorControlData>
}

impl ControllerMaster{
    pub fn new() -> ControllerMaster{
        ControllerMaster{
            motor_controllers: HashMap::new()
        }
    }

    pub fn get_controller(&self, pin: u8) -> Option<&MotorControlData>{
        self.motor_controllers.get(&pin)
    }

    pub fn add_controller(&mut self, pin: u8, controller: MotorControlData){
        self.motor_controllers.insert(pin, controller);
    }

    pub fn command_all_controllers(&self, command: MotorCommand){
        for (_pin, controller) in self.motor_controllers.iter(){
            controller.update(command);
        }
    }
}

fn main() {
    let client = UdpSocket::bind("192.168.1.226:7870").expect("Failed to bind client UDP socket.");

    println!("Connected!");

    let led_pin = Gpio::new().unwrap().get(23).expect("Failed to obtain GPIO pin 23!").into_output();
    thread::spawn(move || blink_led(led_pin));

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

fn blink_led(mut led_pin: OutputPin){
    loop{
        led_pin.set_high();

        thread::sleep(Duration::from_secs(10));

        led_pin.set_low();

        thread::sleep(Duration::from_secs(10));
    }
}