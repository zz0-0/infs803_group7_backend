mod user;
mod movie;

use axum::{
    http::StatusCode, routing::get, Json, Router
};
use chrono::Utc;
use serde_json::json;
use user::*;
use movie::*;

#[tokio::main]
async fn main() {
    // build our application with a single route

    let mut json_result: Vec<User> = Vec::new();
    json_result.push(User { id: Some(1), name: "a".to_owned(), created_at: Some(Utc::now()), updated_at: Some(Utc::now()) });
    let app = Router::new()
    .route("/", get(|| async { "Hello, World!" }))
    .route("/api/users", get((StatusCode::OK, Json(json!(json_result)))))
    .route("/api/user/:id", get(fetch_user).post(create_user).patch(update_user).delete(delete_user))
    .route("/api/movie/:id", get(fetch_movie).post(create_movie).patch(update_movie).delete(delete_movie));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


