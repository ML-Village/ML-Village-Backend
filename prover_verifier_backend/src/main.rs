use rocket::serde::json::Json;

#[macro_use] extern crate rocket;

#[post("/infer")]
fn infer() -> &'static str {

}

#[post("/generate_proof")]
fn generate_proof() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![infer, generate_proof])
}