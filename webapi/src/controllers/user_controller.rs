use crate::services::factory::Factory;
use crate::models::controller_models::ApiResponse;

use std::sync::Mutex;

use uuid::Uuid;

use rocket::http::Cookies;
use rocket::State;
use rocket_contrib::json;

#[post("/heartbeat", format = "application/json")]
pub fn heartbeat(mut cookies: Cookies, factory: State<Mutex<Factory>>) -> ApiResponse{
    let cookie = cookies.get_private("user_id").unwrap();

    let user_id = cookie.value();

    let mut user_service = factory.lock().expect("Failed to obtain Factory!").user_service();

    let user_count = user_service.heartbeat_user(Uuid::parse_str(user_id).unwrap());
        
    ApiResponse{
        json: json!({"status": "success", "user_count": user_count}),
        status: rocket::http::Status::Ok
    }
}