use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
 };
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds_option;

#[derive(Serialize, Deserialize)]
pub struct Movie {
    id: Option<i32>,
    name: String,
    #[serde(with = "ts_seconds_option")]
    created_at: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds_option")]
    updated_at: Option<DateTime<Utc>>
}

pub async fn fetch_movies() -> Json<Vec<Movie>> {
    let movies = vec![];
    Json(movies)
}

pub async fn create_movie() -> impl IntoResponse {
    Response::builder().status(StatusCode::CREATED).body(Body::from("movie created successfully"))
    .unwrap()
}

pub async fn fetch_movie() -> impl IntoResponse {}

pub async fn update_movie() -> impl IntoResponse {}

pub async fn delete_movie() -> impl IntoResponse {}