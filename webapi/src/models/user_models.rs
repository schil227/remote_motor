use crate::models::command_models::CommandData;

use std::cmp::Ordering;
use std::net::SocketAddr;

use chrono::Utc;
use uuid::Uuid;
use chrono::DateTime;

pub struct UserData{
    _user_id: Uuid,
    time_to_live: DateTime<Utc>,
    register_time: DateTime<Utc>,
    ip: SocketAddr,
    pub command: Option<CommandData>,
}

impl UserData{
    pub fn new(_user_id: Uuid, ip: SocketAddr) -> UserData{
        UserData{
            _user_id,
            time_to_live: Utc::now(),
            register_time: Utc::now(),
            ip,
            command: None,
        }
    }

    pub fn refresh(&mut self) {
        self.time_to_live = Utc::now()
    }

    pub fn is_expired(&self, cutoff: DateTime<Utc>) -> bool {
        match self.time_to_live.cmp(&cutoff) {
            Ordering::Less => {true}
            _=> {false}
        }
    }

    pub fn set_command(&mut self, command: CommandData){
        self.command = Some(command);
    }

    pub fn flush_command(&mut self) -> Option<CommandData>{
        let command = self.command;
        self.command = None;

        command
    }

    pub fn get_exit_info(&self, expired: DateTime<Utc>) -> String{
        format!("{} Joined: {}, Left: {} (delta: {:?})", self.ip, self.register_time, expired, self.register_time - expired)
    }
}