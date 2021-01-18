#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use serde::{Deserialize, Serialize};

use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response;
use rocket::response::{Responder, Response};
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

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
struct CommandData{
    claw: u8,
    hand: u8,
    forearm: u8,
    strongarm: u8,
    shoulder: u8
}

#[post("/command", format = "application/json", data= "<command_data>")]
fn command(command_data: Json<CommandData>) -> ApiResponse{
    println!("Command Data: claw: {}, hand: {}, fore: {}, strong: {}, shoulder {}", 
    command_data.claw, 
    command_data.hand,
    command_data.forearm, 
    command_data.strongarm, 
    command_data.shoulder);

    ApiResponse{
        json: json!({"status": "success"}),
        status: rocket::http::Status::Ok
    }
}

fn main() {
    rocket::ignite()
    .mount("/", routes![index])
    .mount("/", routes![command])
    .launch();
}