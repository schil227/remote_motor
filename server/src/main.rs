mod controller_master;
mod message_receiver;
mod motor_controller;
mod scoreboard_runner;
mod scoreboard_service;

use std::thread;
use std::sync::{Arc, RwLock};

extern crate linux_embedded_hal as hal;
extern crate pwm_pca9685 as pca9685;

// use pca9685::{Address, Channel, Pca9685};

fn main() {
    // let led_pin = Gpio::new().unwrap().get(23).expect("Failed to obtain GPIO pin 23!").into_output();
    // thread::spawn(move || blink_led(led_pin));

    let goal_count = Arc::new(RwLock::new(0));
    let goal_count_copy = Arc::clone(&goal_count);

    thread::spawn(move || scoreboard_service::run_scoreboard(goal_count_copy));

    

    let handler = thread::spawn(|| message_receiver::listen(goal_count));

    handler.join().unwrap();
}

// fn test_motors() {
//     let i2c_device = hal::I2cdev::new("/dev/i2c-1").unwrap();

//     // 0x40, the standard address for pwm9685
//     let address = Address::default();

//     let mut pwm = Pca9685::new(i2c_device, address).unwrap();

//     pwm.set_prescale(121).unwrap();

//     //turn on Channel 0 with no delay(immediately turn on for the first tick of 4096 cycle)
//     pwm.set_channel_on(Channel::C0, 0).unwrap();

//     let min = 82;
//     let max = 492;

//     //turn off Channel 0 after 6% of 4096 ticks has elapsed (4096 * 0.06 ~= 246)
//     pwm.set_channel_off(Channel::C0, min as u16).unwrap();

//     pwm.enable().unwrap();

//     println!("Moving to min");

//     thread::sleep(Duration::from_millis(500));

//     let mut current: i16 = min;
//     let mut step: i16 = 1;

//     loop {
//         current += step;

//         if current >= max {
//             step = -1;
//         }

//         if current <= 0 {
//             break;
//         }

//         pwm.set_channel_off(Channel::C0, current as u16).unwrap();

//         thread::sleep(Duration::from_millis(5));
//     }

//     pwm.disable().unwrap();

//     let _i2c_device = pwm.destroy();
// }

// fn blink_led(mut led_pin: OutputPin){
//     loop{
//         led_pin.set_high();

//         thread::sleep(Duration::from_secs(10));

//         led_pin.set_low();

//         thread::sleep(Duration::from_secs(10));
//     }
// }
