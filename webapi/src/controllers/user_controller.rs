use crate::services::user_service::UserService;
use crate::models::controller_models::ApiResponse;

use std::sync::{Mutex, Arc};

use uuid::Uuid;

use rocket::http::RawStr;
use rocket::State;
use rocket_contrib::json;

#[post("/heartbeat/<user_id>", format = "application/json")]
pub fn heartbeat(user_id: &RawStr, user_service: State<Arc<Mutex<UserService>>>) -> ApiResponse{

    let user_count = user_service.lock().expect("Failed to obtain command sender!")
        .heartbeat_user(Uuid::parse_str(user_id.as_str()).unwrap());
        
    ApiResponse{
        json: json!({"status": "success", "user_count": user_count}),
        status: rocket::http::Status::Ok
    }
}