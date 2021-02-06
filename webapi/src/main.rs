#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod controllers;
mod services;
mod models;

use crate::controllers::command_controller;
use crate::controllers::user_controller;
use crate::controllers::debug_controller;
use crate::services::motor_message_creator::MotorMessageCreator;
use crate::services::command_sender::CommandSender;
use crate::services::user_service::UserService;
use crate::services::user_service;
use crate::models::command_models::CommandData;

use std::sync::{Mutex, Arc};

use rocket::http::Method;
use rocket::fairing::AdHoc;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

use chrono::Local;

// Always use a limit to prevent DoS attacks.
const _LIMIT: u64 = 256;

fn main() {
    let command_sender = CommandSender::new("192.168.1.38:7870".to_string());

    let user_service = Arc::new(Mutex::new(UserService::new()));
    let user_service_reference = Arc::clone(&user_service);

    std::thread::spawn(move || {
        user_service::purge_expired_users(user_service_reference)
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
        
        log::info!("    => Time: {}", time);
        log::info!("    => Client: {}", ip);
    }))
    .manage(Mutex::new(command_sender))
    .manage(Arc::clone(&user_service))
    .manage(Mutex::new(last_command))
    .launch();
}