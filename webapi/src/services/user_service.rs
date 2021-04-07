use crate::models::user_models::UserData;
use crate::models::command_models::CommandData;

use std::thread;
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

    pub fn heartbeat_user(&mut self, user_id: Uuid) -> usize{
        let mut data = self.data.lock().expect("failed to obtain user data lock.");

        match data.get_mut(&user_id){
            Some(user) => {
                user.refresh();
            },
            None => {
                data.insert(user_id, UserData::new(user_id));
            }
        }

        data.keys().len()
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
        let cutoff = Utc::now() - chrono::Duration::seconds(30);

        {
            let mut data = user_service.data.lock().expect("failed to obtain user data lock.");

            let expired :Vec<Uuid> = data
                .iter()
                .filter(|&(_,v)| (&v).is_expired(cutoff))
                .map(|(k,_)| k.clone())
                .collect();
                
            for user_id in expired{
                data.remove(&user_id);
            }
        }

        thread::sleep(std::time::Duration::from_secs(3));
    }
}