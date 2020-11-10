use std::net::UdpSocket;

fn main() {
    let client = UdpSocket::bind("127.0.0.1:7870").expect("Failed to bind client UDP socket.");

    loop
    {
        let mut buf = [0; 10];

        client.recv(&mut buf).expect("Failed receiving message.");

        println!("Recieved a message: {:?}", buf);
    }
}
