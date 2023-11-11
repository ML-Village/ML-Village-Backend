use rocket::tokio::fs::create_dir;

use rocket::form::{Form, Strict};
use rocket::fs::TempFile;
use rocket::http::Status;
use rocket::response::status::{self, NotFound};
use rocket::tokio;
use rocket::{
    form, fs::NamedFile, http::ContentType, response::status::BadRequest, serde::json::Json,
    FromForm,
};
use rocket_db_pools::{sqlx, Connection, Database};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use uuid::Uuid;

mod giza_utils;
mod cors;
use cors::CORS;

#[macro_use]
extern crate rocket;

const MODEL_PATH: &str = "models";
/**
 * Database setup
 */
#[derive(Database)]
#[database("prover_backend_db")]
struct ProverBackendDB(sqlx::SqlitePool);

/**
 * Routes
 */


 /**
  * --- Upload Model ---
  */
#[derive(FromForm)]
struct UploadModelForm<'r> {
    name: &'r str,
    description: &'r str,
    onnx_file: TempFile<'r>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct UploadModelResult {
    model_id: String,
}

#[post("/upload_model", data = "<model_data>")]
async fn upload_model(
    mut db: Connection<ProverBackendDB>,
    mut model_data: Form<Strict<UploadModelForm<'_>>>,
) -> Result<Json<UploadModelResult>, BadRequest<String>> {
    let model_id = Uuid::new_v4();
    let model_path = format!("{}/{}", MODEL_PATH, model_id.to_string());
    let model_onnx_path = format!("{}/{}/{}.onnx", MODEL_PATH, model_id.to_string(), model_id.to_string());

    // Create dir for the model
    match create_dir(&model_path).await {
        Ok(_) => Ok::<(), String>(()),
        Err(err) => return Err(BadRequest(err.to_string()))
    };

    // Persist the model
    match model_data.onnx_file.persist_to(&model_onnx_path).await {
        Ok(_) => (),
        Err(err) => return Err(BadRequest(err.to_string())),
    };

    // Now call the transpile function with the saved file
    let transpile_result = giza_utils::transpile_onnx_to_orion(&model_onnx_path, &model_path)
    .await;
    match transpile_result {
        Ok(_) => Ok::<(), String>(()),
        Err(err) =>  return Err(BadRequest(err.to_string())),
    };

    let insert_result = sqlx::query(
        "INSERT INTO ml_models (id, name, description, model_path) VALUES
         (?, ?, ?, ?)",
    )
    .bind(model_id.to_string())
    .bind(model_data.name)
    .bind(model_data.description)
    .bind(model_path)
    .execute(&mut **db).await;

    match insert_result {
        Ok(_) => Ok::<(), String>(()),
        Err(err) => return Err(BadRequest(err.to_string()))
    };

    Ok(Json(
        UploadModelResult { model_id: model_id.to_string() }
    ))
}


/**
 * --- Infer ---
 */
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


/**
 * --- Get Proof ---
 */
#[get("/proof/<proof_id>")]
async fn get_proof(
    mut db: Connection<ProverBackendDB>,
    proof_id: String,
) -> Result<(ContentType, NamedFile), BadRequest<String>> {
    let query_result = sqlx::query("SELECT * FROM ml_proofs WHERE id = ?")
    .bind(&proof_id)
    .fetch_one(&mut **db)
    .await
    .ok();

    // Check if proof id exists
    match query_result {
        Some(row) => row,
        None => return Err(BadRequest("proof not found".to_string()))
    };

    // Get and return the generated proof
    let content_type = ContentType::new("application", "octet-stream");
    let file = match NamedFile::open(format!("inference_result/{}.proof", &proof_id)).await {
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
        .mount("/", routes![upload_model, infer, get_proof])
}
