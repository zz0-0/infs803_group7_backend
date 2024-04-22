use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{ServerConfig, User};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}
#[derive(Serialize, Deserialize)]
pub struct RefreshRequest {
    refresh_token: String,
}

pub async fn login_account(
    State(server_config): State<ServerConfig>,
    Json(login_request): Json<LoginRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    // let data = server_config
    //     .firebase
    //     .at("users")
    //     .at(&login_request.username);

    let data = server_config.firebase.at("users");

    let users = data.get::<Vec<User>>().await.ok().unwrap();

    let user = users.iter().find(|f| f.username == login_request.username);

    let username = &user.unwrap().username;
    let password = &user.unwrap().password;

    // let claims = Claims {
    //     sub: username.to_string(),
    //     exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
    // };
    // let token = encode_jwt(&claims, "secret").unwrap();

    if login_request.password == password.to_string() {
        match &user {
            Some(_) => Ok((StatusCode::OK, Json(json!({"token": "123"})))),
            None => Ok((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"message": format!("no user")})),
            )),
        }
    } else {
        Ok((
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"message": format!("password incorrect")})),
        ))
    }
}

pub async fn validate(request: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let auth_header = request.headers().get("Authorization");

    match auth_header {
        Some(header_value) => {
            let token = header_value.to_str().unwrap().trim_start_matches("Bearer ");
            let secret = "secret";

            match decode_jwt(token, secret) {
                Ok(_) => Ok(next.run(request).await),
                Err(_) => Ok(Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(Body::empty())
                    .unwrap()),
            }
        }
        None => Ok(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::empty())
            .unwrap()),
    }
}

pub async fn refresh_token(
    State(server_config): State<ServerConfig>,
    Json(refresh_request): Json<RefreshRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let refresh_data = decode_jwt(&refresh_request.refresh_token, "secret");

    let decode = refresh_data.unwrap();

    let user_data = server_config.firebase.at("users").at(&decode.claims.sub);
    let user = user_data.get::<User>().await.ok();

    // if user.refresh_token != refresh_request.refresh_token {
    //     return Ok((
    //         StatusCode::UNAUTHORIZED,
    //         Json(serde_json::json!({"message": format!("refresh token incorrect")})),
    //     ));
    // }

    let claims = Claims {
        sub: user.unwrap().username,
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .unwrap();

    Ok((StatusCode::OK, Json(json!({"token": token}))))
}

pub fn encode_jwt(claims: &Claims, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn decode_jwt(
    token: &str,
    secret: &str,
) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    // let validation = Validation {
    //     algorithms: vec![Algorithm::HS256],
    //     ..Default::default()
    // };
    decode(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
}
