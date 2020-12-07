mod float_comparison;
mod motor_controller;
mod message_receiver;
mod controller_master;

use rppal::gpio::Gpio;
use rppal::gpio::OutputPin;
use std::thread;
use std::time::Duration;

fn main() {
    let led_pin = Gpio::new().unwrap().get(23).expect("Failed to obtain GPIO pin 23!").into_output();
    thread::spawn(move || blink_led(led_pin));

    let handler = thread::spawn(|| message_receiver::listen());

    handler.join().unwrap();
}

fn blink_led(mut led_pin: OutputPin){
    loop{
        led_pin.set_high();

        thread::sleep(Duration::from_secs(10));

        led_pin.set_low();

        thread::sleep(Duration::from_secs(10));
    }
}