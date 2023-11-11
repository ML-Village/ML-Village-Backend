use std::path::Path;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use tokio::fs;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let sqlite_options = SqliteConnectOptions::new()
        .create_if_missing(true)
        .filename("prover_backend_db")
        ;
    let pool = SqlitePoolOptions::new()
        .connect_with(sqlite_options)
        .await?;

    // Ml models table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS ml_models (
            id VARCHAR PRIMARY KEY NOT NULL,
            name VARCHAR NOT NULL,
            description VARCHAR NOT NULL,
            price VARCHAR NOT NULL,
            model_path VARCHAR NOT NULL
        );",
    )
    .execute(&pool)
    .await?;

    // Proof tables
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS ml_proofs (
            id VARCHAR PRIMARY KEY NOT NULL,
            model_id VARCHAR NOT NULL
        );",
    )
    .execute(&pool)
    .await?;

    // User table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            api_key VARCHAR NOT NULL
        );",
    ).execute(&pool)
    .await?;

    // User model purchase
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users_model (
            user_id INTEGER,
            model_id VARCHAR NOT NULL,
            FOREIGN KEY(user_id) REFERENCES users(id),
            FOREIGN KEY(model_id) REFERENCES ml_models(id)
        );",
    ).execute(&pool)
    .await?;

    if !Path::new("models").exists() {
        fs::create_dir("models").await?;
    }

    if !Path::new("inference_result").exists() {
        fs::create_dir("inference_result").await?;
    }
        
    Ok(())
}
