#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod services;

use crate::services::motor_message_creator::MotorMessageCreator;
use crate::services::command_sender::CommandSender;

use models::MotorMessage;

use std::sync::Mutex;
use std::net::UdpSocket;

use serde::{Deserialize, Serialize};

use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response;
use rocket::response::{Responder, Response};
use rocket::State;
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};

// Always use a limit to prevent DoS attacks.
const LIMIT: u64 = 256;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Debug)]
struct ApiResponse {
    json: JsonValue,
    status: Status,
}

impl<'r> Responder<'r> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.json.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

struct FooService{
    bar: u8
}

impl FooService{
    pub fn change_foo(&mut self, num: u8){
        self.bar += num
    }

    pub fn get_foo(&self) -> String{
        format!("Bar: {}", self.bar)
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct CommandData{
    claw: u8,
    hand: u8,
    forearm: u8,
    strongarm: u8,
    shoulder: u8
}

#[post("/command", format = "application/json", data= "<command_data>")]
fn command(command_data: Json<CommandData>, command_sender_mutex: State<Mutex<CommandSender>>) -> ApiResponse{
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

    // Does server need to be updated to accept an actual value instead of min/max?

    ApiResponse{
        json: json!({"status": "success"}),
        status: rocket::http::Status::Ok
    }
}

fn main() {
    let client = UdpSocket::bind("192.168.1.186:7870").expect("Failed to bind client UDP socket.");

    let command_sender = Mutex::new(CommandSender::new(client, "192.168.1.38:7870".to_string()));

    rocket::ignite()
    .mount("/", routes![index])
    .mount("/", routes![command])
    .manage(command_sender)
    .launch();
}