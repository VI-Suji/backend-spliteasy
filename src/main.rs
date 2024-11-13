use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get},
    Json, Router,
};
use serde_json::json;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::net::{TcpListener};
use crate::schema::{Splits};

pub mod schema;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Not found env");

    let db = std::env::var("DATABASE_URL").expect("Error while getting data");
    let pool = PgPoolOptions::new()
        .max_connections(16)
        .connect(&db)
        .await
        .expect("Error while getting connect");

    let addr = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3001".to_owned());
    let tcp = TcpListener::bind(addr)
        .await
        .expect("Error while trying to connect");

    let app = Router::new()
        .route("/", get(get_splits))
        .with_state(pool);

    axum::serve(tcp, app).await.expect("error")
}

async fn get_splits(State(pool):State<PgPool>) -> Result<(StatusCode, String),(StatusCode,String)> {
    let rows = sqlx::query_as!(Splits, " SELECT * FROM splits ORDER BY split_user").fetch_all(&pool).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"success":false, "message":e.to_string()}).to_string()
        )
    })?;

    Ok((
        StatusCode::OK,
        json!({"success":true, "data":rows}).to_string()
 
    ))
}