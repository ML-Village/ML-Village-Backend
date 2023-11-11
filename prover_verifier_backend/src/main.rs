use rand::RngCore;
use rocket::http::Status;
use rocket::tokio::fs::create_dir;

use rocket::form::{Form, Strict};
use rocket::fs::TempFile;
use rocket::response::status::{self, Custom, NotFound};
use rocket::{
    form, fs::NamedFile, http::ContentType, response::status::BadRequest, serde::json::Json,
    FromForm,
};
use rocket_db_pools::{sqlx, Connection, Database};
use serde::{Deserialize, Serialize};
use service::{register_model, register_subscription};
use sqlx::Row;
use starknet::core::types::FieldElement;
use uuid::Uuid;

mod cors;
mod giza_utils;
mod service;
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
    name: String,
    description: String,
    price: String,
    onnx_file: TempFile<'r>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct UploadModelResult {
    model_id: String,
    register_result: String,
}

#[post("/upload_model", data = "<model_data>")]
async fn upload_model(
    mut db: Connection<ProverBackendDB>,
    mut model_data: Form<Strict<UploadModelForm<'_>>>,
) -> Result<Json<UploadModelResult>, BadRequest<String>> {
    let model_id = Uuid::new_v4();
    let model_path = format!("{}/{}", MODEL_PATH, model_id.to_string());
    let model_onnx_path = format!(
        "{}/{}/{}.onnx",
        MODEL_PATH,
        model_id.to_string(),
        model_id.to_string()
    );

    // Create dir for the model
    match create_dir(&model_path).await {
        Ok(_) => Ok::<(), String>(()),
        Err(err) => return Err(BadRequest(err.to_string())),
    };

    // Persist the model
    match model_data.onnx_file.persist_to(&model_onnx_path).await {
        Ok(_) => (),
        Err(err) => return Err(BadRequest(err.to_string())),
    };

    // Now call the transpile function with the saved file
    let transpile_result = giza_utils::transpile_onnx_to_orion(&model_onnx_path, &model_path).await;
    match transpile_result {
        Ok(_) => Ok::<(), String>(()),
        Err(err) => return Err(BadRequest(err.to_string())),
    };

    let insert_result = sqlx::query(
        "INSERT INTO ml_models (id, name, description, price, model_path) VALUES
         (?, ?, ?, ?, ?)",
    )
    .bind(model_id.to_string())
    .bind(&model_data.name)
    .bind(&model_data.description)
    .bind(&model_data.price)
    .bind(model_path)
    .execute(&mut **db)
    .await;

    match insert_result {
        Ok(_) => Ok::<(), String>(()),
        Err(err) => return Err(BadRequest(err.to_string())),
    };

    let id = model_id.to_string();

    // Removing hyphens and converting to lowercase
    let hex_id = id.replace("-", "").to_lowercase();

    // Now use hex_id as needed
    let field = FieldElement::from_hex_be(&hex_id).unwrap();
    let register_result = register_model(field).await.transaction_hash;
    println!("register_result: {:?}", register_result);

    Ok(Json(UploadModelResult {
        model_id: model_id.to_string(),
        register_result: register_result.to_string(),
    }))
}

#[options("/<_..>")]
fn all_options() -> Custom<()> {
    Custom(Status::Ok, ())
}

/**
 * --- Infer ---
 */
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct InferParams<'r> {
    model_id: &'r str,
    input_data: &'r str,
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
        None => return Err(BadRequest("proof not found".to_string())),
    };

    // Get and return the generated proof
    let content_type = ContentType::new("application", "octet-stream");
    let file = match NamedFile::open(format!("inference_result/{}.proof", &proof_id)).await {
        Ok(file) => file,
        Err(_) => return Err(BadRequest("Failed to find proof".to_owned())),
    };

    Ok((content_type, file))
}

/**
 * --- Purchase Model ---
 */

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct PurchaseModelParams<'r> {
    api_key: &'r str,
    owner_address: &'r str,
    subscription_end_timestamp: &'r str,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct MlModel {
    id: String,
    name: String,
    description: String,
    price: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct MlPurchaseModel {
    id: String,
    name: String,
    description: String,
    price: String,
    transaction_hash: String,
}

#[post("/model/<model_id>/purchase", data = "<params>")]
async fn purchase_model(
    mut db: Connection<ProverBackendDB>,
    model_id: String,
    params: Json<PurchaseModelParams<'_>>,
) -> Result<Json<MlPurchaseModel>, BadRequest<String>> {
    // TODO: implement proper purchase model flow
    let query_result = sqlx::query("SELECT * FROM users WHERE api_key = ?")
        .bind(params.api_key)
        .fetch_one(&mut **db)
        .await
        .ok();

    println!("part 1");
    let user = match query_result {
        Some(row) => row,
        None => return Err(BadRequest(("Cannot find user".to_string()))),
    };
    println!("part 2");
    let insert_result: Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> =
        sqlx::query("INSERT INTO users_model (user_id, model_id) VALUES (?, ?)")
            .bind(user.get::<String, &str>("id"))
            .bind(&model_id)
            .execute(&mut **db)
            .await;
    println!("part 3");
    match insert_result {
        Ok(_) => Ok::<(), String>(()),
        Err(err) => return Err(BadRequest(err.to_string())),
    };

    println!("part 4");

    let query_result = sqlx::query("SELECT * FROM ml_models WHERE id = ?")
        .bind(&model_id)
        .fetch_one(&mut **db)
        .await
        .ok();

    println!("part 5");

    let model = match query_result {
        Some(row) => row,
        None => return Err(BadRequest(("Cannot find user".to_string()))),
    };

    println!("masuk");

    let register_subscription_result = register_subscription(
        FieldElement::from_hex_be(&params.owner_address).unwrap(),
        FieldElement::from_hex_be(&model_id).unwrap(),
        FieldElement::from_hex_be(&params.subscription_end_timestamp).unwrap(),
    )
    .await
    .transaction_hash;

    println!(
        "register_subscription_result: {:?}",
        register_subscription_result
    );

    Ok(Json(MlPurchaseModel {
        description: model.get("description"),
        id: model.get("id"),
        name: model.get("name"),
        price: model.get("price"),
        transaction_hash: register_subscription_result.to_string(),
    }))
}

/**
 * --- Create User ---
 */

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct UserResult {
    user_id: i64,
    api_key: String,
}

#[post("/create_user")]
async fn create_user(
    mut db: Connection<ProverBackendDB>,
) -> Result<Json<UserResult>, BadRequest<String>> {
    // TODO: implement proper flow for api keys

    let mut bytes = [0; 32];
    rand::thread_rng().fill_bytes(&mut bytes);

    let new_key = const_hex::encode(bytes);

    let insert_result = sqlx::query("INSERT INTO users (api_key) VALUES (?)")
        .bind(&new_key)
        .execute(&mut **db)
        .await;

    match insert_result {
        Ok(_) => Ok::<(), String>(()),
        Err(err) => return Err(BadRequest(err.to_string())),
    };

    let query_result = sqlx::query("SELECT * FROM  users where api_key = ?")
        .bind(&new_key)
        .fetch_one(&mut **db)
        .await
        .unwrap();

    Ok(Json(UserResult {
        user_id: query_result.get("id"),
        api_key: new_key,
    }))
}

/**
 * --- Get Me ---
 */

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct MeResult {
    models: Vec<MlModel>,
}

#[get("/me/<api_key>")]
async fn get_me(
    mut db: Connection<ProverBackendDB>,
    api_key: String,
) -> Result<Json<MeResult>, BadRequest<String>> {
    let query_result = sqlx::query(
        "
    SELECT ml_models.id, name, description, price FROM ml_models
        LEFT JOIN users_model ON users_model.model_id = ml_models.id
        LEFT JOIN users ON users_model.user_id = users.id
        WHERE users.api_key = ?",
    )
    .bind(&api_key)
    .fetch_all(&mut **db)
    .await
    .ok();

    let user_models = match query_result {
        Some(rows) => rows,
        None => return Err(BadRequest("Nothing found".to_string())),
    };

    let user_models = user_models
        .iter()
        .map(|row| MlModel {
            id: row.get("id"),
            description: row.get("description"),
            name: row.get("name"),
            price: row.get("price"),
        })
        .collect::<Vec<MlModel>>();

    Ok(Json(MeResult {
        models: user_models,
    }))
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .attach(ProverBackendDB::init())
        .mount(
            "/",
            routes![
                upload_model,
                infer,
                purchase_model,
                all_options,
                get_proof,
                create_user,
                get_me
            ],
        )
}
