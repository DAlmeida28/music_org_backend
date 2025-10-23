use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
pub struct Sets {
    pub id: String,
    pub name: String,
}

pub async fn get_sets(State(pool): State<PgPool>) -> impl IntoResponse {
    match sqlx::query_as!(Sets, "SELECT id, name FROM set_events")
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
