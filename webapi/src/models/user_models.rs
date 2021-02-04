use std::cmp::Ordering;

use chrono::Utc;
use uuid::Uuid;
use chrono::DateTime;

pub struct UserData{
    _user_id: Uuid,
    time_to_live: DateTime<Utc>
}

impl UserData{
    pub fn new(_user_id: Uuid) -> UserData{
        UserData{
            _user_id,
            time_to_live: Utc::now()
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
}