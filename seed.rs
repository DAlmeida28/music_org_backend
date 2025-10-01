#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! sqlx = { version = "0.8", features = [ "runtime-async-std", "tls-native-tls", "postgres" ] }
//! tokio = { version = "1", features = ["full"] }
//! dotenvy = { version = "0.15.7"}
//! ```
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let env = dotenvy::dotenv().is_ok();
    if env {
        println!("env file found");
    } else {
        println!("env file not found");
    }

    let database_url = std::env::var("DATABASE_URL").expect("Error reading Database_Url");

    println!("Setting up DB Connection.");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("DB Connection esablished.");

    println!("Creating users table..");

    sqlx::query(
        "CREATE TABLE users(
            id UUID NOT NULL UNIQUE,
            username VARCHAR(150) NOT NULL UNIQUE
            );",
    )
    .execute(&pool)
    .await?;

    println!("Creating genre table..");

    sqlx::query(
        "CREATE TABLE genre(
            id UUID NOT NULL UNIQUE,
            name VARCHAR(150) NOT NULL UNIQUE
            );",
    )
    .execute(&pool)
    .await?;

    println!("Creating users_genre Table..");

    sqlx::query(
        "CREATE TABLE users_genre(
            id UUID NOT NULL UNIQUE,
            name VARCHAR(200) NOT NULL UNIQUE,
            users UUID NOT NULL REFERENCES users(id)
            );",
    )
    .execute(&pool)
    .await?;

    println!("Creating set_events table..");

    sqlx::query(
        "CREATE TABLE set_events(
            id UUID NOT NULL UNIQUE,
            name VARCHAR(500) NOT NULL UNIQUE,
            event_date DATE
        );",
    )
    .execute(&pool)
    .await?;

    println!("Creating tracks table..");

    sqlx::query(
        "CREATE TABLE tracks(
            id UUID NOT NULL UNIQUE,
            track_name TEXT,
            track_url TEXT,
            track_genre UUID REFERENCES genre(id),
            track_user_genre UUID REFERENCES users_genre(id),
            set_events UUID REFERENCES set_events(id)
            );",
    )
    .execute(&pool)
    .await?;

    println!("All tables succesfully!");

    Ok(())
}
