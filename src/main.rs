mod jwt;
mod movie;
mod user;

use std::sync::Arc;

use axum::{
    middleware::from_fn,
    routing::{get, post},
    Router,
};

use firebase_rs::Firebase;
use jwt::*;
use movie::*;
use tower_http::cors::{Any, CorsLayer};
use user::*;

#[derive(Clone)]
pub struct ServerConfig {
    pub firebase: Arc<Firebase>,
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any);
    let firebase = Firebase::new("https://infs803-group7-default-rtdb.firebaseio.com/").unwrap();
    let firebase = Arc::new(firebase);

    let server_config = ServerConfig { firebase: firebase };

    let app = Router::new()
        .route("/login", post(login_account))
        .route("/refresh", post(refresh_token))
        .route("/users", get(fetch_users))
        // .route_layer(from_fn(validate))
        .route(
            "/user/:id",
            get(fetch_user)
                .post(create_user)
                .patch(update_user)
                .delete(delete_user),
        )
        // .route_layer(from_fn(validate))
        .route(
            "/movie/:id",
            get(fetch_movie)
                .post(create_movie)
                .patch(update_movie)
                .delete(delete_movie),
        )
        // .route_layer(from_fn(validate))
        .layer(cors)
        .with_state(server_config);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
