use std::net::UdpSocket;
use std::thread;
use std::time::Duration;
use models::MotorCommand;

fn main() {
    let client = UdpSocket::bind("127.0.0.1:7879").expect("Failed to bind client UDP socket.");

    loop
    {
        let serialized_direction = bincode::serialize(&MotorCommand::Backward(5)).expect("Failed to serialize direction!");

        let mut buf = [0; 10];

        for (i, val) in serialized_direction.iter().enumerate(){
            buf[i] = *val;
        }

        println!("Sending message: len: {}, {:?}", &buf.len(), &buf);

        client.send_to(&buf, "127.0.0.1:7870").expect("Failed to send message!");

        println!("Sent.");

        thread::sleep(Duration::from_secs(3));
    }
}
