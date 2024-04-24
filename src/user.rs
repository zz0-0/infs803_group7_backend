use crate::ServerConfig;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    // #[serde(deserialize_with = "from_string")]
    pub id: i8,
    pub name: String,
    pub level: i8,
    pub username: String,
    pub password: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted: bool,
}

pub async fn fetch_users(
    State(server_config): State<ServerConfig>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let data = server_config.firebase.at("users");
    let users = data.get::<Vec<Option<User>>>().await;
    let json_response = serde_json::json!({"users": users.as_ref().unwrap()});
    match users {
        Ok(_) => Ok((StatusCode::OK, Json(json_response))),
        Err(e) => Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": format!("list users fail: { }", e)})),
        )),
    }
}

pub async fn create_user(
    State(server_config): State<ServerConfig>,
    Json(user): Json<User>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let data = server_config.firebase.at("users");
    let user = data.set::<User>(&user).await;
    match user {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(serde_json::json!({"message": "create user successful"})),
        )),
        Err(e) => Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": format!("create user fail: { }", e)})),
        )),
    }
}

pub async fn fetch_user(
    State(server_config): State<ServerConfig>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let data = server_config.firebase.at("users").at(&id);
    let user = data.get::<User>().await;
    let json_response = serde_json::json!(user.as_ref().unwrap());
    match user.as_ref() {
        Ok(_) => Ok((StatusCode::OK, Json(json_response))),
        Err(e) => Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": format!("create user fail: { }", e)})),
        )),
    }
}

pub async fn update_user(
    State(server_config): State<ServerConfig>,
    Path(id): Path<String>,
    Json(user): Json<User>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let data = server_config.firebase.at("users").at(&id);
    let user = data.update::<User>(&user).await;
    match user {
        Ok(s) => Ok((StatusCode::OK, Json(serde_json::json!({"message": s.data})))),
        Err(e) => Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": format!("update user fail: { }", e)})),
        )),
    }
}

pub async fn delete_user(
    State(server_config): State<ServerConfig>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let data = server_config.firebase.at("users").at(&id);

    let user = data.delete().await;
    match user {
        Ok(f) => Ok((StatusCode::OK, Json(serde_json::json!({"message": f.data})))),
        Err(e) => Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": format!("delte user fail: { }", e)})),
        )),
    }
}
