use crate::services::factory::Factory;
use crate::models::command_models::CommandData;
use crate::models::controller_models::ApiResponse;

use std::sync::Mutex;

use uuid::Uuid;

use rocket::http::Cookies;
use std::net::SocketAddr;
use rocket::State;
use rocket_contrib::json;
use rocket_contrib::json::Json;

#[post("/heartbeat", format = "application/json")]
pub fn heartbeat(mut cookies: Cookies, ip: SocketAddr, factory: State<Mutex<Factory>>) -> ApiResponse{
    let cookie = cookies.get_private("user_id").unwrap();

    let user_id = cookie.value();

    let mut user_service = factory.lock().expect("Failed to obtain Factory!").user_service();

    let user_count = user_service.heartbeat_user(Uuid::parse_str(user_id).unwrap(), ip);
        
    ApiResponse::new(
        json!({"status": "success", "user_count": user_count}),
        rocket::http::Status::Ok
    )
}

#[post("/set-command", format = "application/json", data= "<command_data>")]
pub fn set_command(
    command_data: Json<CommandData>, 
    mut cookies: Cookies, 
    ip: SocketAddr,
    factory: State<Mutex<Factory>>
) -> ApiResponse{
    let cookie = cookies.get_private("user_id").unwrap();

    let user_id = cookie.value();

    let mut user_service = factory.lock().expect("Failed to obtain Factory!").user_service();

    user_service.set_command(Uuid::parse_str(user_id).unwrap(), *command_data, ip);

    ApiResponse::new(
        json!({"status": "success", "command": *command_data}),
        rocket::http::Status::Ok
    )
}