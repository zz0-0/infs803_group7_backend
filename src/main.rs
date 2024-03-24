mod user;
mod movie;

use axum::{
    routing::get, Router
};

use user::*;
use movie::*;

#[tokio::main]
async fn main() {
    let app = Router::new()
    
    .route("/api/users", get(fetch_users))

    .route("/api/user/:id", get(fetch_user)
    .post(create_user)
    .patch(update_user)
    .delete(delete_user))
    

    .route("/api/movie/:id", get(fetch_movie)
    .post(create_movie)
    .patch(update_movie)
    .delete(delete_movie)
    
);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
