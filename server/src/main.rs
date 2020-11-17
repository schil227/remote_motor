use std::net::UdpSocket;
use models::MotorCommand;
use rppal::gpio::Gpio;
// use rppal::pwm::{Channel, Pwm};

const SIZE : usize = std::mem::size_of::<MotorCommand>();

fn main() {
    let client = UdpSocket::bind("192.168.1.226:7870").expect("Failed to bind client UDP socket.");

    println!("Connected!");

    let mut pin = Gpio::new().unwrap().get(23).expect("Failed to obtain GPIO pin 23!").into_output();

    loop
    {
        let mut buf = [0; SIZE + 10];

        client.recv(&mut buf).expect("Failed receiving message.");

        let direction : MotorCommand = bincode::deserialize(&buf).expect("Could not deserialize MotorDirection!");
        
        println!("Recieved a direction: {:?}", direction);

        match direction{
            MotorCommand::Forward(_num) =>{
                pin.set_high();
            },
            MotorCommand::Backward(_num) =>{

            },
            MotorCommand::Stop() =>{
                pin.set_low();
            },
        }
    }
}
