use rocket::tokio;
use rocket::{
    fs::NamedFile,
    http::{ContentType, Status},
    response::status::BadRequest,
    serde::json::Json,
};
use rocket_db_pools::{sqlx, Connection, Database};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use uuid::Uuid;

mod cors;
use cors::CORS;

#[macro_use]
extern crate rocket;

#[derive(Database)]
#[database("prover_backend_db")]
struct ProverBackendDB(sqlx::SqlitePool);

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct InferParams<'r> {
    model_id: &'r str,
    input_data: Vec<&'r str>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct InferResult<'r> {
    proof_id: &'r str,
    infer_result: &'r str,
}

#[post("/infer", data = "<params>")]
async fn infer(
    mut db: Connection<ProverBackendDB>,
    params: Json<InferParams<'_>>,
) -> Result<Json<InferResult>, BadRequest<String>> {
    let found_model = sqlx::query("SELECT * FROM ml_models WHERE id = ?")
        .bind(params.model_id)
        .fetch_one(&mut **db)
        .await
        .ok();

    let model_path: String = match found_model {
        Some(model) => model.get("model_path"),
        None => return Err(BadRequest("Failed to locate saved model.".to_owned())),
    };

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

    return Ok(Json(InferResult {
        infer_result: "",
        proof_id: "",
    }));
}

#[get("/proof/<proof_id>")]
async fn get_proof(
    mut db: Connection<ProverBackendDB>,
    proof_id: String,
) -> Result<(ContentType, NamedFile), BadRequest<String>> {
    // Get and return the generated proof
    let content_type = ContentType::new("application", "octet-stream");
    let file = match NamedFile::open(format!("inference_result/{}.proof", proof_id)).await {
        Ok(file) => file,
        Err(_) => return Err(BadRequest("Failed to find proof".to_owned())),
    };
    Ok((content_type, file))
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .attach(ProverBackendDB::init())
        .mount("/", routes![infer])
}
