use rocket::serde::json::Json;

#[macro_use] extern crate rocket;

#[post("/generate_proof")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[post("/verify_proof")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}