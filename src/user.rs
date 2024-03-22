use std::{fs::File, io::Read};

use axum::{
    body::Body, http::StatusCode, response::{IntoResponse, Response}, Json
    
};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds_option;
use serde_json::{json, Value};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub level: Option<i32>,
    #[serde(with = "ts_seconds_option")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>
}

#[derive(Serialize, Deserialize)]
pub struct UserResponse {
    pub result: User,
}

#[derive(Serialize, Deserialize)]
pub struct UserListResponse {
    pub result: Vec<User>,
}

// #[derive(Debug)]
// pub struct AppError {
//     code: StatusCode,
//     message: String,
// }

pub  async fn fetch_users() -> (StatusCode, Json<Value>) {
    // let users = vec![];
    let mut file = File::open("src/example.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    // let json = serde_json::from_str(&data);

    let mut json_result: Vec<User> = Vec::new();
    json_result.push(User { id: Some(1), name: "a".to_owned(),level:Some(1),  created_at: Some(Utc::now()), updated_at: Some(Utc::now()) });
    
    // Ok(Json(json!(UserListResponse{result : json_result})))
    (StatusCode::OK,Json(json!(json_result)))
    // Json(str)
}

pub async fn create_user() -> impl IntoResponse {
    Response::builder().status(StatusCode::CREATED).body(Body::from("User created successfully"))
    .unwrap()
}

pub async fn fetch_user() -> impl IntoResponse {}

pub async fn update_user() -> impl IntoResponse {}

pub async fn delete_user() -> impl IntoResponse {}