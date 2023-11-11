use rocket::tokio;
use rocket::{
    fs::NamedFile,
    http::{ContentType, Status},
    response::status::BadRequest,
    serde::json::Json,
};
use serde::Deserialize;
use uuid::Uuid;

mod cors;
use cors::CORS;

#[macro_use]
extern crate rocket;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct InferParams {
    model_id: String,
    input_data: Vec<String>,
}

#[post("/infer", data = "<params>")]
async fn infer(params: Json<InferParams>) -> Result<(ContentType, NamedFile), BadRequest<String>> {
    // !todo!("get and run the model");

    // Create arbritrary id for the proof generated
    let proof_id = Uuid::new_v4();
    // !todo!("generate the proof and save to a directory");

    // Get and return the generated proof
    let content_type = ContentType::new("application", "octet-stream");
    let file = match NamedFile::open(format!(
        "inference_result/{}/{}.proof",
        params.model_id, proof_id
    ))
    .await
    {
        Ok(file) => file,
        Err(_) => return Err(BadRequest("Failed to generate proof".to_owned())),
    };
    Ok((content_type, file))
}

#[launch]
async fn rocket() -> _ {
    rocket::build().attach(CORS).mount("/", routes![infer])
}
