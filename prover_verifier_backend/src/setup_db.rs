use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};

#[tokio::main] // Requires the `attributes` feature of `async-std`
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

    Ok(())
}
