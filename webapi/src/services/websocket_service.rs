extern crate ws;

use std::thread;
use std::time::Duration;
use std::time;

use ws::{connect, Handler, Sender, Handshake, Result, Message, CloseCode};

pub struct WebSocketServer{
    pub out: Sender
}

impl Handler for WebSocketServer {
    fn on_open(&mut self, _: Handshake) -> Result<()>{
        Ok(())
    }

    fn on_message(&mut self, _: Message) -> Result<()>{
        Ok(())
    }
}

pub fn run(){
    ws::listen("192.168.1.248:8001", |out| {
        let out_copy = out.clone();

        thread::spawn(move || {
            let mut n = 0;

            while n < 20 {
                println!("sending: {}", format!("message {}", n));
                let msg = format!("{{\"foo\": {}}}", n);

                out_copy.send(msg).unwrap();

                n += 1;

                thread::sleep(Duration::from_secs(3));
            }
        });

        WebSocketServer{
            out: out
        }
    })
    .expect("Failed to create websocket server on ::8001")
}