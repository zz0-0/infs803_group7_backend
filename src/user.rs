use std::{collections::HashMap, fs::File, io::Read};

use axum::response::Response;
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use chrono::serde::ts_seconds_option;
use chrono::{DateTime, Utc};
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
    let firebase = firebase.at("users");
    let users = firebase.get::<Vec<User>>().await.map_err(|e| {
        let error = serde_json::json!({"message": format!("Database error: { }", e)});
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error))
    });
    let json_response = serde_json::json!({"users": &users.unwrap()});
    Ok((StatusCode::OK, Json(json_response)))
}

pub async fn create_user(Json(user): Json<User>) -> impl IntoResponse {
    let firebase = Firebase::new("").unwrap();
    let firebase = firebase.at("users");
    let _users = firebase.set::<User>(&user).await;
}

pub async fn fetch_user(Path(id): Path<String>) -> impl IntoResponse {
    let firebase = Firebase::new("").unwrap();
    let firebase = firebase.at("users").at(&id);
    let user = firebase.get::<User>().await;
}

pub async fn update_user(Json(user): Json<User>) -> impl IntoResponse {
    let firebase = Firebase::new("").unwrap();
    let firebase = firebase.at("users").at(&user.id.unwrap().to_string());
    let _user = firebase.update::<User>(&user).await;
}

pub async fn delete_user(Path(id): Path<String>) -> impl IntoResponse {
    let firebase = Firebase::new("").unwrap();
    let firebase = firebase.at("users").at(&id);
    let _result = firebase.delete().await;
}
