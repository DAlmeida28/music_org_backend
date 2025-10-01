#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! sqlx = { version = "0.8", features = [ "runtime-async-std", "tls-native-tls", "postgres" ] }
//! tokio = { version = "1", features = ["full"] }
//! ```
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let database_url = env::Var("DATABASE_URL").expect("Error reading Database_Url");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    sqlx::query(
        "CREATE TABLE users(
        id UUID NOT NULL UNIQUE,
        username VARCHAR(150) UNIQUE
            );",
    )
    .execute(&pool)
    .await?;

    Ok(())
}
