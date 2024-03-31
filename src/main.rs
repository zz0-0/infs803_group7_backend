mod login;
mod movie;
mod user;

use axum::{
    body::Body,
    http::{header, Request, StatusCode},
    middleware::{self, Next},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use jsonwebtoken::{decode, DecodingKey, Validation};
use login::Claims;
use movie::*;
use serde_json::Value;
use tower_http::cors::{Any, CorsLayer};
use user::*;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .route("/login", post(login::login))
        .route(
            "/api/users",
            get(fetch_users).route_layer(middleware::from_fn(auth)),
        )
        .route(
            "/api/user/:id",
            get(fetch_user)
                .post(create_user)
                .patch(update_user)
                .delete(delete_user)
                .route_layer(middleware::from_fn(auth)),
        )
        .route(
            "/api/movie/:id",
            get(fetch_movie)
                .post(create_movie)
                .patch(update_movie)
                .delete(delete_movie)
                .route_layer(middleware::from_fn(auth)),
        )
        .layer(cors);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub async fn auth(
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| {
            auth_value
                .strip_prefix("Bearer ")
                .map(|stripped| stripped.to_owned())
        });

    let token = token.ok_or_else(|| {
        let json_error = serde_json::json!({"message": "Missing bearer token".to_string()});
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?;

    let secret = "secret";

    let claims = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .unwrap()
    .claims;

    req.extensions_mut().insert(claims);
    Ok(next.run(req).await)
}
