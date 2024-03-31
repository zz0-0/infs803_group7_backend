use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use firebase_rs::Firebase;
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub level: Option<i32>,
    // #[serde(with = "ts_seconds_option")]
    pub created_at: String,
    // #[serde(with = "ts_seconds_option")]
    pub updated_at: String,
}

pub async fn fetch_users() -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let firebase = Firebase::new("https://infs803-group7-default-rtdb.firebaseio.com/").unwrap();
    let data = firebase.at("users");
    let users = data.get::<Vec<User>>().await;
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
    Json(user): Json<User>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let firebase = Firebase::new("https://infs803-group7-default-rtdb.firebaseio.com/").unwrap();
    let data = firebase.at("users");
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
    Path(id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let firebase = Firebase::new("https://infs803-group7-default-rtdb.firebaseio.com/").unwrap();
    let data = firebase.at("users").at(&id);
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
    Path(id): Path<String>,
    Json(user): Json<User>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let firebase = Firebase::new("https://infs803-group7-default-rtdb.firebaseio.com/").unwrap();
    let data = firebase.at("users").at(&id);
    let user = data.update::<User>(&user).await;
    match user {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(serde_json::json!({"message": "update user successful"})),
        )),
        Err(e) => Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": format!("update user fail: { }", e)})),
        )),
    }
}

pub async fn delete_user(
    Path(id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let firebase = Firebase::new("https://infs803-group7-default-rtdb.firebaseio.com/").unwrap();
    let data = firebase.at("users").at(&id);
    let user = data.delete().await;
    match user {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(serde_json::json!({"message": "delete user successful"})),
        )),
        Err(e) => Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": format!("delte user fail: { }", e)})),
        )),
    }
}
