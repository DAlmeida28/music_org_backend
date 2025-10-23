use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
pub struct Genre {
    pub id: String,
    pub name: String,
}

pub async fn get_genre(State(Pool): State<PgPool>) -> impl IntoResponse {
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
