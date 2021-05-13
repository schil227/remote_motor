use crate::models::user_models::UserData;
use crate::models::command_models::CommandData;

use std::thread;
use std::net::SocketAddr;
use std::sync::{Mutex, Arc};
use std::collections::HashMap;

use chrono::Utc;
use uuid::Uuid;

pub struct UserService{
    data: Arc<Mutex<HashMap<Uuid, UserData>>>
}

impl UserService {
    pub fn new(user_data: Arc<Mutex<HashMap<Uuid, UserData>>>,) -> UserService{
        let service = UserService{
            data: user_data
        };

        service
    }

    pub fn heartbeat_user(&mut self, user_id: Uuid, ip: SocketAddr) -> usize{
        let mut data = self.data.lock().expect("failed to obtain user data lock.");

        match data.get_mut(&user_id){
            Some(user) => {
                user.refresh();
            },
            None => {
                data.insert(user_id, UserData::new(user_id, ip));
            }
        }

        data.keys().len()
    }

    pub fn set_command(&mut self, user_id: Uuid, command: CommandData, ip: SocketAddr) {
        let mut user_data_map = self.data.lock().expect("failed to obtain user data lock.");

        match user_data_map.get_mut(&user_id){
            Some(user) => {
                println!("Set command {:?}", command);
                user.set_command(command);
            },
            None => {
                println!("Set command {:?}, new user.", command);
                let mut user_data = UserData::new(user_id, ip);
                user_data.set_command(command);
                user_data_map.insert(user_id, user_data);
            }
        }
    }

    pub fn flush_commands(&mut self) -> Vec<CommandData> {
        {
            let mut user_data_map = self.data.lock().expect("Failed to lock user data");

            let mut commands = vec!();

            for (_key, user_data) in user_data_map.iter_mut(){
                match user_data.flush_command() {
                    Some(command) => {
                        commands.push(command);
                    },
                    None => {}
                }
            }

            commands
        }
    }
}

pub fn purge_expired_users(user_service: UserService){
    loop{
        let now = Utc::now();
        let cutoff = now - chrono::Duration::seconds(30);

        {
            let mut data = user_service.data.lock().expect("failed to obtain user data lock.");

            let expired :Vec<Uuid> = data
                .iter()
                .filter(|&(_,v)| (&v).is_expired(cutoff))
                .map(|(k,_)| k.clone())
                .collect();
                
            for user_id in expired{
                match data.remove(&user_id){
                    Some(user) => {
                        log::info!("User removed: {}", user.get_exit_info(now))
                    }
                    _ => {}
                }
            }
        }

        thread::sleep(std::time::Duration::from_secs(3));
    }
}