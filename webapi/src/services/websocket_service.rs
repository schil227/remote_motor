extern crate ws;

use std::thread;
use std::sync::RwLock;
use std::sync::Arc;
use std::time::Duration;

use ws::{ Handler, Sender, Handshake, Result, Message};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ServerState{
    AcceptingInput,
    Warning,
    Locked
}

pub struct WebSocketHandler {
    _out: Sender
}

impl Handler for WebSocketHandler {
    fn on_open(&mut self, _: Handshake) -> Result<()>{
        Ok(())
    }

    fn on_message(&mut self, _: Message) -> Result<()>{
        Ok(())
    }
}

pub struct WebSocketServer{
    pub server_state: Arc<RwLock<ServerState>>
}

impl<'a> WebSocketServer{
    pub fn new() -> WebSocketServer{
        WebSocketServer{
            server_state: Arc::new(RwLock::new(ServerState::AcceptingInput))
        }
    }

    pub fn set_server_state(&mut self, state: ServerState){
        let mut server_state = self.server_state.write().unwrap();
        *server_state = state;
    }
}

pub fn run(state : Arc<RwLock<ServerState>>){
    ws::listen("192.168.1.248:8001", |out| {
        let out_copy = out.clone();
        
        let state_copy = Arc::clone(&state);

        thread::spawn(move || {
            state_change_listener(state_copy, out_copy);
        });

        WebSocketHandler{
            _out: out
        }
    })
    .expect("Failed to create websocket server on ::8001")
}

fn state_change_listener(state: Arc<RwLock<ServerState>>, out : Sender) {
    let mut last_state = *(state.read().unwrap());

    println!("Running state change listener.");

    loop {
        println!("Reading current state.");

        let current_state = *(state.read().unwrap());

        println!("States: {:?}, {:?}", last_state, current_state);

        if last_state != current_state {
            println!("New state! {:?}", current_state);

            last_state = current_state;

            let msg = format!("{{\"state\": {:?}}}", last_state);

            out.send(msg).expect("Failed to send message.");
        }

        thread::sleep(Duration::from_secs(1));
    }
}