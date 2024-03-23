use std::{collections::HashMap, fs::File, io::Read};

use axum::{
    extract::Path, http::StatusCode, response::IntoResponse, Json
    
};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds_option;
use serde_json::{json, Value};
use firebase_rs::Firebase;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub level: Option<i32>,
    #[serde(with = "ts_seconds_option")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>
}

pub  async fn fetch_users() -> (StatusCode, Json<Value>) {
    let firebase =  Firebase::new("").unwrap();
    let firebase = firebase.at("users");
    let users = firebase.get::<HashMap<String, User>>().await;

    let mut file = File::open("src/example.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let mut json_result: Vec<User> = Vec::new();
    json_result.push(User { id: Some(1), name: "a".to_owned(),level:Some(1),  created_at: Some(Utc::now()), updated_at: Some(Utc::now()) });
    
    (StatusCode::OK,Json(json!(json_result)))
}

pub async fn create_user(Json(user): Json<User>) -> impl IntoResponse {
    let firebase =  Firebase::new("").unwrap();
    let firebase = firebase.at("users");
    let _users = firebase.set::<User>(&user).await;
}

pub async fn fetch_user(Path(id): Path<String>) -> impl IntoResponse {
    let firebase =  Firebase::new("").unwrap();
    let firebase = firebase.at("users").at(&id);
    let user = firebase.get::<User>().await;
}

pub async fn update_user(Json(user): Json<User>) -> impl IntoResponse {
    let firebase =  Firebase::new("").unwrap();
    let firebase = firebase.at("users").at(&user.id.unwrap().to_string());
    let _user = firebase.update::<User>(&user).await;
}

pub async fn delete_user(Path(id): Path<String>) -> impl IntoResponse {
    let firebase =  Firebase::new("").unwrap();
    let firebase = firebase.at("users").at(&id);
    let _result = firebase.delete().await;
}