mod float_comparison;
mod motor_controller;
mod message_receiver;
mod controller_master;

use rppal::gpio::Gpio;
use rppal::gpio::OutputPin;
use std::thread;
use std::time::Duration;

extern crate linux_embedded_hal as hal;
extern crate pwm_pca9685 as pca9685;

use pca9685::{Channel, Pca9685, Address};

fn main() {
    // let led_pin = Gpio::new().unwrap().get(23).expect("Failed to obtain GPIO pin 23!").into_output();
    // thread::spawn(move || blink_led(led_pin));

    // let handler = thread::spawn(|| message_receiver::listen());

    // handler.join().unwrap();

    test_motors();
}

fn test_motors(){
    let i2c_device = hal::I2cdev::new("/dev/i2c-1").unwrap();

    // 0x40, the standard address for pwm9685 
    let address = Address::default();

    let mut pwm = Pca9685::new(i2c_device, address).unwrap();

    pwm.set_prescale(121).unwrap();

    //turn on Channel 0 with no delay(immediately turn on for the first tick of 4096 cycle)
    pwm.set_channel_on(Channel::C0, 0).unwrap();

    //turn off Channel 0 after 6% of 4096 ticks has elapsed (4096 * 0.06 ~= 246)
    pwm.set_channel_off(Channel::C0, 246).unwrap();

    // let mut claw = Gpio::new().unwrap().get(24).unwrap().into_output();
    // let mut hand = Gpio::new().unwrap().get(17).unwrap().into_output();
    // let mut forearm = Gpio::new().unwrap().get(18).unwrap().into_output();
    // let mut strongarm = Gpio::new().unwrap().get(27).unwrap().into_output();
    // let mut shoulder = Gpio::new().unwrap().get(22).unwrap().into_output();

    // claw.set_pwm_frequency(50.0, 0.05).unwrap();
    // hand.set_pwm_frequency(50.0, 0.05).unwrap();
    // forearm.set_pwm_frequency(50.0, 0.05).unwrap();
    // strongarm.set_pwm_frequency(50.0, 0.05).unwrap();
    // shoulder.set_pwm_frequency(50.0, 0.05).unwrap();

    thread::sleep(Duration::from_secs(5));
}

fn blink_led(mut led_pin: OutputPin){
    loop{
        led_pin.set_high();

        thread::sleep(Duration::from_secs(10));

        led_pin.set_low();

        thread::sleep(Duration::from_secs(10));
    }
}