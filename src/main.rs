use axum::{Router, routing::get};
use sqlx::PgPool;
use std::env;
use std::error::Error;
use std::net::SocketAddr;

async fn test() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is missing");

    let pool = PgPool::connect(&database_url).await.unwrap();

    let app = Router::new()
        .route("/api/test", get(test).post(test))
        .route("/data", get(move || get_data(pool.clone())));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Listening on : {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn get_data(pool: PgPool) -> String {
    let row: (String,) = sqlx::query_as("SELECT * FROM music;")
        .fetch_one(&pool)
        .await
        .unwrap();

    row.0
}
