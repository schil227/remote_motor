use crate::models::controller_models::ApiResponse;

use rocket_contrib::json;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[post("/echo", format = "application/json", data= "<text>")]
pub fn echo(text: String) -> ApiResponse{
    println!("echo: {}", text);

    ApiResponse::new(
        json!({"status": "success", "text": text}),
        rocket::http::Status::Ok
    )
}