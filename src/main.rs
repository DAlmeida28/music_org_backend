use axum::{Router, http::Method, routing::get};
use sqlx::PgPool;
use std::{env, error::Error, net::SocketAddr};
use tower_http::cors::CorsLayer;

#[path = "db/genre.rs"]
mod genre;
#[path = "db/sets.rs"]
mod sets;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    let allowed_origins = ["http://localhost:5173".parse().unwrap()];

    let cors = CorsLayer::new()
        .allow_origin(allowed_origins)
        .allow_methods(vec![Method::GET]);

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is missing");

    let pool = PgPool::connect(&database_url).await?;

    let app = Router::new()
        .route("/genre", get(genre::get_genre).post(genre::get_genre))
        .route("/sets", get(sets::get_sets).post(sets::get_sets))
        .layer(cors)
        .with_state(pool.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    println!("Running server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
