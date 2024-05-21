use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

use crate::ServerConfig;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Movie {
    // pub id: i32,
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
    pub budget_x: f64,
    pub revenue: f64,
    pub country: String,
    pub deleted: bool,
}

pub async fn fetch_movies(
    State(server_config): State<ServerConfig>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let data = server_config.firebase.at("movies");
    let movies = data.get::<Vec<Option<Movie>>>().await;
    let json_response = serde_json::json!({"movies": movies.as_ref().unwrap()});
    match movies {
        Ok(_) => Ok((StatusCode::OK, Json(json_response))),
        Err(e) => Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": format!("list movies fail: { }", e)})),
        )),
    }
}

pub async fn create_movie(
    State(server_config): State<ServerConfig>,
    Json(movie): Json<Movie>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let data = server_config.firebase.at("movies");
    let movie = data.set::<Movie>(&movie).await;
    match movie {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(serde_json::json!({"message": "create movie successful"})),
        )),
        Err(e) => Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": format!("create movie fail: { }", e)})),
        )),
    }
}

pub async fn fetch_movie(
    State(server_config): State<ServerConfig>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let data = server_config.firebase.at("movies").at(&id);
    let movie = data.get::<Movie>().await;
    let json_response = serde_json::json!(movie.as_ref().unwrap());
    match movie.as_ref() {
        Ok(_) => Ok((StatusCode::OK, Json(json_response))),
        Err(e) => Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": format!("create movie fail: { }", e)})),
        )),
    }
}

pub async fn update_movie(
    State(server_config): State<ServerConfig>,
    Path(id): Path<String>,
    Json(movie): Json<Movie>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let data = server_config.firebase.at("movies").at(&id);
    let movie = data.update::<Movie>(&movie).await;
    match movie {
        Ok(s) => Ok((StatusCode::OK, Json(serde_json::json!({"message": s.data})))),
        Err(e) => Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": format!("update movie fail: { }", e)})),
        )),
    }
}

pub async fn delete_movie(
    State(server_config): State<ServerConfig>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let data = server_config.firebase.at("movies").at(&id);

    let movie = data.delete().await;
    match movie {
        Ok(f) => Ok((StatusCode::OK, Json(serde_json::json!({"message": f.data})))),
        Err(e) => Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": format!("delte movie fail: { }", e)})),
        )),
    }
}
