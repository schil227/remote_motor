use crate::models::user_models::UserData;

use crate::services::command_sender::CommandSender;
use crate::services::command_processor::CommandProcessor;
use crate::services::user_service::UserService;

use std::sync::{Mutex, Arc};
use std::collections::HashMap;

use uuid::Uuid;

pub struct Factory
{
    user_data: Arc<Mutex<HashMap<Uuid, UserData>>>,
    command_sender_target: String
}

impl Factory {
    pub fn new() -> Factory{
        Factory{
            user_data: Arc::new(Mutex::new(HashMap::new())),
            command_sender_target: "192.168.1.98:7870".to_string()
        }
    }

    pub fn command_sender(&self) -> CommandSender{
        CommandSender::new(self.command_sender_target.to_string())
    }

    pub fn user_service(&self) -> UserService{
        UserService::new(Arc::clone(&self.user_data))
    }

    pub fn command_processor(&self) -> CommandProcessor{
        CommandProcessor::new(
            self.command_sender(),
            self.user_service()
        )
    }
}
