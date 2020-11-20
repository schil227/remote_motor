mod float_comparison;
mod motor_controller;

use std::net::UdpSocket;
use models::MotorCommand;
use rppal::gpio::Gpio;
use rppal::gpio::OutputPin;
use std::thread;
use std::time::Duration;
use motor_controller::MotorControlData;
use std::sync::{Arc, Mutex};

const SIZE : usize = std::mem::size_of::<MotorCommand>();

fn main() {
    let client = UdpSocket::bind("192.168.1.226:7870").expect("Failed to bind client UDP socket.");

    println!("Connected!");

    let mut led_pin = Gpio::new().unwrap().get(23).expect("Failed to obtain GPIO pin 23!").into_output();
    let mut motor_pin = Gpio::new().unwrap().get(24).expect("Failed to obtain GPIO pin 24!").into_output();
    let motor_control_data = Arc::new(Mutex::new(MotorControlData::new()));

    thread::spawn(move || blink_led(led_pin));

    // move motor to neutral position (90 degrees)
    let mut control_data = Arc::clone(&motor_control_data);
    thread::spawn(move || motor_controller::run_motor(&mut motor_pin, &mut control_data));
    
    loop
    {
        let mut buf = [0; SIZE + 10];

        client.recv(&mut buf).expect("Failed receiving message.");

        let direction : MotorCommand = bincode::deserialize(&buf).expect("Could not deserialize MotorDirection!");
        
        println!("Recieved a direction: {:?}", direction);

        motor_controller::update_motor(direction, &mut Arc::clone(&motor_control_data));
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