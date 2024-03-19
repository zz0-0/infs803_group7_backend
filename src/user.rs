use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds_option;

#[derive(Serialize, Deserialize)]
struct User {
    id: Option<i32>,
    name: String,
    #[serde(with = "ts_seconds_option")]
    created_at: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds_option")]
    updated_at: Option<DateTime<Utc>>
}

async fn fetch_users() -> Json<Vec<User>> {
    let users = vec![];
    Json(users)
}

async fn create_user() -> impl IntoResponse {
    Response::builder().status(StatusCode::CREATED).body(Body::from("User created successfully"))
    .unwrap()
}

async fn fetch_user() -> impl IntoResponse {}

async fn update_user() -> impl IntoResponse {}

async fn delete_user() -> impl IntoResponse {}