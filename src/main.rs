use axum::{
    Json, Router,
    extract::State,
    http::{Method, StatusCode},
    response::IntoResponse,
    routing::get,
};
use serde::Serialize;
use sqlx::PgPool;
use std::{env, error::Error, net::SocketAddr};
use tower_http::cors::CorsLayer;

#[derive(Serialize)]
struct Genre {
    id: String,
    name: String,
}

#[derive(Serialize)]
struct Sets {
    id: String,
    name: String,
}

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
        .route("/genre", get(get_genre).post(get_genre))
        .route("/sets", get(get_sets).post(get_sets))
        .layer(cors)
        .with_state(pool.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    println!("Running server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn get_sets(State(pool): State<PgPool>) -> impl IntoResponse {
    match sqlx::query_as!(Sets, "Select id, name FROM set_events")
        .fetch_all(&pool)
        .await
    {
        Ok(sets) => Json(sets).into_response(),
        Err(err_msg) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database query failed: {}", err_msg),
        )
            .into_response(),
    }
}

async fn get_genre(State(pool): State<PgPool>) -> impl IntoResponse {
    match sqlx::query_as!(Genre, "SELECT id, name FROM genre")
        .fetch_all(&pool)
        .await
    {
        Ok(genres) => Json(genres).into_response(),
        Err(err_msg) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database query failed: {}", err_msg),
        )
            .into_response(),
    }
}
