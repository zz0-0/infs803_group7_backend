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
pub struct Favorite {
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

pub async fn fetch_favorites(
    State(server_config): State<ServerConfig>,
    Path(user_id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let data = server_config.firebase.at("favorites").at(&user_id);
    let favorites = data.get::<Vec<Option<Favorite>>>().await;

    match favorites {
        Ok(_) => {
            let json_response = serde_json::json!({"favorites": favorites.as_ref().unwrap()});
            Ok((StatusCode::OK, Json(json_response)))
        }
        Err(e) => Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": format!("list favorites fail: { }", e)})),
        )),
    }
}

pub async fn create_favorite(
    State(server_config): State<ServerConfig>,
    Path((user_id, favorite_id)): Path<(String, String)>,
    // Path(favorite_id): Path<String>,
    Json(favorite): Json<Favorite>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let data = server_config
        .firebase
        .at("favorites")
        .at(&user_id)
        .at(&favorite_id);
    let favorite = data.update::<Favorite>(&favorite).await;
    match favorite {
        Ok(s) => Ok((StatusCode::OK, Json(serde_json::json!({"message": s.data})))),
        Err(e) => Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": format!("user fail: { }", e)})),
        )),
    }
}

pub async fn update_favorite(
    State(server_config): State<ServerConfig>,
    Path((user_id, favorite_id)): Path<(String, String)>,
    Json(favorite): Json<Favorite>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let data = server_config
        .firebase
        .at("favorites")
        .at(&user_id)
        .at(&favorite_id);
    let favorite = data.update::<Favorite>(&favorite).await;
    match favorite {
        Ok(s) => Ok((StatusCode::OK, Json(serde_json::json!({"message": s.data})))),
        Err(e) => Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": format!("user fail: { }", e)})),
        )),
    }
}
