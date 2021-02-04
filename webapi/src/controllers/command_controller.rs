use crate::services::motor_message_creator::MotorMessageCreator;
use crate::services::command_sender::CommandSender;
use crate::models::command_models::CommandData;
use crate::models::controller_models::ApiResponse;

use std::sync::Mutex;

use rocket::State;
use rocket_contrib::json;
use rocket_contrib::json::Json;

#[get("/command")]
pub fn get_most_recent_command(last_command: State<Mutex<CommandData>>) -> ApiResponse{
    
    let last_command = last_command.lock().expect("[GET /command] failed to lock last command");

    ApiResponse{
        json: json!({
            "claw": last_command.claw,
            "hand":  last_command.hand,
            "forearm":  last_command.forearm,
            "strongarm":  last_command.strongarm,
            "shoulder":  last_command.shoulder,
        }),
        status: rocket::http::Status::Ok
    }
}

#[post("/command", format = "application/json", data= "<command_data>")]
pub fn command(
    command_data: Json<CommandData>, 
    command_sender_mutex: State<Mutex<CommandSender>>,
    last_command: State<Mutex<CommandData>>
) -> ApiResponse{
    println!("Command Data: claw: {}, hand: {}, fore: {}, strong: {}, shoulder {}", 
    command_data.claw, 
    command_data.hand,
    command_data.forearm, 
    command_data.strongarm, 
    command_data.shoulder);

    let messages = MotorMessageCreator::get_messages(*command_data);

    {
        let command_sender = command_sender_mutex.lock().expect("Failed to obtain command sender!");
        
        command_sender.send_commands(messages);
    };

    {
        let mut last_command =  last_command.lock().expect("[POST /command] failed to consume last command");

        *last_command = *command_data;
    }

    ApiResponse{
        json: json!({"status": "success"}),
        status: rocket::http::Status::Ok
    }
}
