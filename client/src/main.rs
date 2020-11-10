use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

fn main() {
    let client = UdpSocket::bind("127.0.0.1:7879").expect("Failed to bind client UDP socket.");

    let mut count: u8 = 0;

    loop
    {
        let mut buf = [0;10];

        buf[0] = count;

        println!("Sending message: {:?}", buf);

        client.send_to(&buf, "127.0.0.1:7870").expect("Failed to send message!");

        println!("Sent.");

        if count == 255 {
            count = 0;
        }else{
            count += 1;
        }

        thread::sleep(Duration::from_secs(3));
    }
}
