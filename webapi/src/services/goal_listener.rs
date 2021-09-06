use std::sync::{Arc, Mutex};
use std::io::{Read};
use std::net::{TcpListener};
use std::time::Duration;
use crate::services::websocket_service::{WebSocketServer};

pub fn listen(websocket_server_lock: Arc<Mutex<WebSocketServer>>){
    let listener = TcpListener::bind("31.220.52.19:7871").expect("Failed to bind listening TCP socket."); // 192.168.1.50:7871

    loop {
        match listener.accept(){
            Ok((mut stream, _addr)) => {
                stream.set_read_timeout(Some(Duration::from_secs(3))).expect("Failed to set read timeout.");

                // one byte => u8
                let mut goals_input = [0; 1];

                stream.read(&mut goals_input).expect("Could not read goal count from stream.");

                let goals: u8 = bincode::deserialize(&goals_input).expect("Failed to parse goal cout");

                log::info!("New goal scored! Goal Count: {}", goals);

                // let source_message = source_message_lock.read().unwrap();
                
                let mut websocket_server = websocket_server_lock.lock().unwrap();
                websocket_server.set_goal_count(goals);
            }
            Err(e) => log::error!("Failed to get TCP client: {:?}", e),
        }
    }
}
