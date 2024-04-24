use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::serde::ts_seconds_option;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Movie {
    pub names: String,
    #[serde(rename = "date_x")]
    pub date_x: String,
    pub score: i64,
    pub genre: String,
    pub overview: String,
    pub crew: String,
    #[serde(rename = "orig_title")]
    pub orig_title: String,
    pub status: String,
    #[serde(rename = "orig_lang")]
    pub orig_lang: String,
    #[serde(rename = "budget_x")]
    pub budget_x: i64,
    pub revenue: i64,
    pub country: String,
}

pub async fn fetch_movies() -> Json<Vec<Movie>> {
    let movies = vec![];
    Json(movies)
}

pub async fn create_movie() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from("movie created successfully"))
        .unwrap()
}

pub async fn fetch_movie() -> impl IntoResponse {}

pub async fn update_movie() -> impl IntoResponse {}

pub async fn delete_movie() -> impl IntoResponse {}
