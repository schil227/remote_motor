use std::net::UdpSocket;
use models::MotorCommand;
use rppal::gpio::Gpio;
use std::time::Duration;

const SIZE : usize = std::mem::size_of::<MotorCommand>();
const PERIOD_MS: u64 = 20;
const PULSE_MIN_US: u64 = 1200;
const PULSE_NEUTRAL_US: u64 = 1500;
const PULSE_MAX_US: u64 = 1800;

fn main() {
    let client = UdpSocket::bind("192.168.1.226:7870").expect("Failed to bind client UDP socket.");

    println!("Connected!");

    let mut led_pin = Gpio::new().unwrap().get(23).expect("Failed to obtain GPIO pin 23!").into_output();
    let mut motor_pin = Gpio::new().unwrap().get(24).expect("Failed to obtain GPIO pin 24!").into_output();

    // move motor to neutral position (90 degrees)
    motor_pin.set_pwm_frequency(
        50.0,
        0.07
    ).unwrap();

    loop
    {
        let mut buf = [0; SIZE + 10];

        client.recv(&mut buf).expect("Failed receiving message.");

        let direction : MotorCommand = bincode::deserialize(&buf).expect("Could not deserialize MotorDirection!");
        
        println!("Recieved a direction: {:?}", direction);

        match direction{
            MotorCommand::Forward(_num) =>{
                println!("Forward!");
                led_pin.set_high();

                motor_pin.set_pwm_frequency(
                    50.0,
                    0.12
                ).unwrap();
            },
            MotorCommand::Backward(_num) =>{
                println!("Backward!");

                motor_pin.set_pwm_frequency(
                    50.0,
                    0.02
                ).unwrap();
            },
            MotorCommand::Stop() =>{
                println!("Stop!");
                led_pin.set_low();

            motor_pin.set_pwm_frequency(
                50.0,
                0.07
            ).unwrap();
            },
        }
    }
}
