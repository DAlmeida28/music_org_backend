#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! sqlx = { version = "0.8", features = [ "runtime-tokio", "uuid", "tls-native-tls", "postgres", "chrono" ] }
//! tokio = { version = "1", features = ["full"] }
//! dotenvy = { version = "0.15.7"}
//! uuid = { version = "1.18.1", features = [ "v4"] }
//! chrono = { version = "0.4", features = ["serde"]}
//! ```
use chrono::NaiveDate;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

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

    println!("Dropping tables..");

    sqlx::query("DROP TABLE genre, set_events, tracks, users, users_genre")
        .execute(&pool)
        .await?;

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
            event_date DATE,
            users UUID NOT NULL REFERENCES users(id)
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

    println!("All tables created succesfully!");

    let usernames = vec![
        "Ambient DJ",
        "Club DJ",
        "Jazz Head",
        "Metal Head",
        "Avant Garde Being",
    ];

    for username in usernames {
        let id = Uuid::new_v4();

        sqlx::query("INSERT INTO users (id, username) VALUES ($1, $2);")
            .bind(id)
            .bind(username)
            .execute(&pool)
            .await?;
    }

    println!("Test users created succesfully!");

    let genres = vec![
        "Drone",
        "Minimal",
        "Ambient",
        "Jersey Club",
        "Latin Club",
        "Bounce",
        "Brazillian Jazz",
        "Japenese Jazz",
        "Bebop",
        "MetalCore",
        "Death Metal",
        "Black Metal",
        "Classic Piano",
        "Orchestra Performance",
        "House",
        "Techo",
        "Field Recordings",
    ];

    for name in genres {
        let id = Uuid::new_v4();

        sqlx::query("INSERT INTO genre(id, name) VALUES ($1, $2);")
            .bind(id)
            .bind(name)
            .execute(&pool)
            .await?;
    }

    println!("Genre's created successfully!");

    let user_genres_ambient = vec!["Ambient Garde", "Post-Club", "Dungeon Synth"];

    let users_genres_avante = vec!["Alient Jazz", "Post Music"];

    let ambientdj_id: Uuid = sqlx::query_scalar("SELECT id FROM users WHERE username = $1")
        .bind("Ambient DJ")
        .fetch_one(&pool)
        .await?;

    let avantegarde_id: Uuid = sqlx::query_scalar("SELECT id FROM users WHERE username = $1")
        .bind("Avant Garde Being")
        .fetch_one(&pool)
        .await?;

    for name in user_genres_ambient {
        let id = Uuid::new_v4();

        sqlx::query("INSERT INTO users_genre(id, name, users) VALUES ($1, $2, $3);")
            .bind(id)
            .bind(name)
            .bind(ambientdj_id)
            .execute(&pool)
            .await?;
    }

    for name in users_genres_avante {
        let id = Uuid::new_v4();

        sqlx::query("INSERT INTO users_genre(id, name, users) VALUES($1, $2, $3);")
            .bind(id)
            .bind(name)
            .bind(avantegarde_id)
            .execute(&pool)
            .await?;
    }

    println!("Users genres create succesffull!");

    let events = vec!["Chill out", "Blowing Minds", "Intergalatic"];

    let event_date = NaiveDate::from_ymd_opt(2026, 8, 24).unwrap();

    for name in events {
        let id = Uuid::new_v4();

        sqlx::query("INSERT INTO set_events(id, name, event_date, users) VALUES($1, $2, $3, $4);")
            .bind(id)
            .bind(name)
            .bind(event_date)
            .bind(avantegarde_id)
            .execute(&pool)
            .await?;
    }

    println!("All set events created successfully");

    Ok(())
}
