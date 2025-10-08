use axum::http::StatusCode;
use axum::{Json, Router, debug_handler, extract::State, response::IntoResponse, routing::get};
use sqlx::PgPool;
use std::env;
use std::error::Error;
use std::net::SocketAddr;
use uuid::Uuid;

async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is missing");

    let pool = PgPool::connect(&database_url).await?;

    let app = Router::new()
        .route("/genre", get(get_genre).post(get_genre))
        .with_state(pool.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Listening on : {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[debug_handler]
async fn get_genre(State(pool): State<PgPool>) -> Result<impl IntoResponse, (StatusCode, String)> {
    match sqlx::query_scalar::<_, String>("SELECT name FROM genre")
        .fetch_all(&pool)
        .await
    {
        Ok(genres) => Ok(Json(genres)),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database query failed".to_string(),
        )),
    }
}
