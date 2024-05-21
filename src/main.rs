mod favorite;
mod jwt;
mod movie;
mod user;

use std::sync::Arc;

use axum::{
    middleware::from_fn,
    routing::{get, post},
    Router,
};

use favorite::*;
use firebase_rs::Firebase;
use jwt::*;
use movie::*;
use tower_http::cors::CorsLayer;
use user::*;

#[derive(Clone)]
pub struct ServerConfig {
    pub firebase: Arc<Firebase>,
}

#[tokio::main]
async fn main() {
    // let cors = CorsLayer::new().allow_origin(Any);
    let firebase = Firebase::new("https://infs803-group7-default-rtdb.firebaseio.com/").unwrap();
    let firebase = Arc::new(firebase);

    let server_config = ServerConfig { firebase: firebase };

    let app = Router::new()
        .route("/login", post(login_account))
        .route("/register", post(register_account))
        .route("/forget", post(forget_account))
        .route("/refresh", post(refresh_token))
        .route("/users", get(fetch_users).route_layer(from_fn(validate)))
        .route("/movies", get(fetch_movies).route_layer(from_fn(validate)))
        .route(
            "/favorites",
            get(fetch_favorites).route_layer(from_fn(validate)),
        )
        .route(
            "/users/:id",
            get(fetch_user)
                .post(create_user)
                .patch(update_user)
                .delete(delete_user)
                .route_layer(from_fn(validate)),
        )
        .route(
            "/favorites/:id",
            post(create_favorite)
                .patch(update_favorite)
                .route_layer(from_fn(validate)),
        )
        .route(
            "/movies/:id",
            get(fetch_movie)
                .post(create_movie)
                .patch(update_movie)
                .delete(delete_movie)
                .route_layer(from_fn(validate)),
        )
        // .layer(cors)
        .layer(CorsLayer::permissive())
        .with_state(server_config);

    // run our app with hyper, listening globally on port 8000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
