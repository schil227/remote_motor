#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod controllers;
mod services;
mod models;

use crate::controllers::command_controller;
use crate::controllers::user_controller;
use crate::controllers::debug_controller;
use crate::services::motor_message_creator::MotorMessageCreator;
use crate::services::user_service;
use crate::services::factory::Factory;
use crate::models::command_models::CommandData;

use std::sync::Mutex;

use rocket::http::{Method, Cookie};
use rocket::fairing::AdHoc;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

use chrono::Local;
use uuid::Uuid;

// Always use a limit to prevent DoS attacks.
const _LIMIT: u64 = 256;

fn main() {
    let factory = Factory::new();

    let command_sender = factory.command_sender();

    let user_service = factory.user_service();   

    std::thread::spawn(move || {
        user_service::purge_expired_users(user_service)
    });

    let last_command = CommandData{
        claw:50,
        hand:50,
        forearm:50,
        strongarm:50,
        shoulder:50
    };

    ////////////// Initialize robot to 50..50 ///////////////
    let messages = MotorMessageCreator::get_messages(last_command.clone());
    command_sender.send_commands(messages);
    
    ////////////// Setup CORS ///////////////
    let allowed_origins = AllowedOrigins::All;

    // You can also deserialize this
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::All,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors().expect("Failed to create CORS.");

    rocket::ignite()
    .mount("/", routes![debug_controller::index])
    .mount("/", routes![debug_controller::echo])
    .mount("/", routes![command_controller::command])
    .mount("/", routes![command_controller::get_most_recent_command])
    .mount("/", routes![user_controller::heartbeat])
    .attach(cors)
    .attach(AdHoc::on_request("Request Logger", move |req, _| {
        let ip = match req.client_ip() 
        { 
            Some(ip) => ip.to_string(), 
            _ => "Unknown IP".to_string()
        };

        let time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        
        let mut is_new = false;
        let user_id = match req.cookies().get_private("user_id"){
            Some(cookie) => {
                Uuid::parse_str(cookie.value()).unwrap()
            },
            None => {   
                println!("no user id.");
                is_new = true;
                let id = Uuid::new_v4();
                id
            }
        };

        let is_new = if is_new {
            req.cookies().add_private(Cookie::new("user_id", user_id.to_string()));
            "(New)"
        } else{
            ""
        };

        log::info!("    => Time: {}", time);
        log::info!("    => Client: {}", ip);
        log::info!("    => UserId: {} {}", is_new, user_id);
    }))
    .manage(Mutex::new(last_command))
    .manage(Mutex::new(factory))
    .launch();
}