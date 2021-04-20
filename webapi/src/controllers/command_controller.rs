use crate::services::motor_message_creator::MotorMessageCreator;
use crate::services::factory::Factory;
use crate::models::command_models::CommandData;
use crate::models::controller_models::ApiResponse;

use std::{sync::Mutex};

use rocket::State;
use rocket_contrib::json;
use rocket_contrib::json::Json;

#[get("/command")]
pub fn get_most_recent_command(last_command: State<Mutex<CommandData>>) -> ApiResponse{
    
    let last_command = last_command.lock().expect("[GET /command] failed to lock last command");

    ApiResponse::new(
        json!({
            "claw": last_command.claw,
            "hand":  last_command.hand,
            "forearm":  last_command.forearm,
            "strongarm":  last_command.strongarm,
            "shoulder":  last_command.shoulder,
        }),
        rocket::http::Status::Ok,
    )
}

#[post("/command", format = "application/json", data= "<command_data>")]
pub fn command(
    command_data: Json<CommandData>, 
    factory_mutex: State<Mutex<Factory>>,
    last_command: State<Mutex<CommandData>>
    // mut cookies: Cookies
) -> ApiResponse{
    println!("Command Data: claw: {}, hand: {}, fore: {}, strong: {}, shoulder {}", 
    command_data.claw, 
    command_data.hand,
    command_data.forearm, 
    command_data.strongarm, 
    command_data.shoulder);

    let messages = MotorMessageCreator::get_messages(*command_data);

    {
        let factory = factory_mutex.lock().expect("Failed to obtain Factory!");
        
        factory.command_sender().send_commands(messages);
    };

    {
        let mut last_command =  last_command.lock().expect("[POST /command] failed to consume last command");

        (*last_command).copy_from(&(*command_data));
    }

    ApiResponse::new(
        json!({"status": "success"}),
        rocket::http::Status::Ok
    )
}
