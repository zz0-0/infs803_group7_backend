use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::*;
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
pub struct RegisterRequest {
    username: String,
    name: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct ForgetRequest {
    username: String,
    email: String,
}

#[derive(Serialize, Deserialize)]
pub struct RefreshRequest {
    refresh_token: String,
}

pub async fn login_account(
    State(server_config): State<ServerConfig>,
    Json(login_request): Json<LoginRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let data = server_config.firebase.at("users");
    let users = data.get::<Vec<User>>().await.ok().unwrap();
    let user = users.iter().find(|f| f.username == login_request.username);

    match &user {
        Some(_) => {
            let username = &user.unwrap().username;
            let password = &user.unwrap().password;
            let level = &user.unwrap().level;

            let claims = Claims {
                sub: username.to_string(),
                exp: (chrono::Utc::now() + chrono::Duration::minutes(15)).timestamp() as usize,
            };

            let claims1 = Claims {
                sub: username.to_string(),
                exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
            };

            let access = encode_jwt(&claims, "secret").unwrap();
            let refresh = encode_jwt(&claims1, "secret").unwrap();

            if login_request.password == password.to_string() {
                Ok((
                    StatusCode::OK,
                    Json(json!({"access": access, "refresh": refresh, "level": level})),
                ))
            } else {
                Ok((
                    StatusCode::UNAUTHORIZED,
                    Json(serde_json::json!({"message": format!("password incorrect")})),
                ))
            }
        }
        None => Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": format!("no user")})),
        )),
    }
}

pub async fn register_account(
    State(server_config): State<ServerConfig>,
    Json(register_request): Json<RegisterRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let data = server_config.firebase.at("users");
    let users = data.get::<Vec<User>>().await.ok().unwrap();
    let user = users
        .iter()
        .find(|f| f.username == register_request.username);
    let mut username = String::new();
    match &user {
        Some(_) => Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": format!("Same username exist")})),
        )),
        None => {
            let id = users.last().unwrap().id - 1;
            let data1 = server_config.firebase.at("users").at(&id.to_string());

            let user1 = User {
                id: id + 1,
                name: register_request.name,
                level: 1,
                username: register_request.username,
                password: register_request.password,
                created_at: chrono::offset::Utc::now().to_string(),
                updated_at: chrono::offset::Utc::now().to_string(),
                deleted: false,
            };
            username = user1.clone().username;

            let user1 = data1.update::<User>(&user1).await;

            match user1 {
                Ok(_) => {
                    let claims = Claims {
                        sub: username.to_string(),
                        exp: (chrono::Utc::now() + chrono::Duration::minutes(15)).timestamp()
                            as usize,
                    };

                    let claims1 = Claims {
                        sub: username.to_string(),
                        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp()
                            as usize,
                    };

                    let access = encode_jwt(&claims, "secret").unwrap();
                    let refresh = encode_jwt(&claims1, "secret").unwrap();
                    Ok((
                        StatusCode::OK,
                        Json(json!({"access": access, "refresh": refresh})),
                    ))
                }
                Err(e) => Ok((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"message": format!("register user fail: { }", e)})),
                )),
            }
        }
    }
}

pub async fn forget_account(
    State(server_config): State<ServerConfig>,
    Json(forget_request): Json<ForgetRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let data = server_config.firebase.at("users");
    let users = data.get::<Vec<User>>().await.ok().unwrap();
    let user = users.iter().find(|f| f.username == forget_request.username);

    match &user {
        Some(_) => {
            let email = Message::builder()
                .from("".parse().unwrap())
                .to(forget_request.email.parse().unwrap())
                .subject("Reset your password")
                .header(ContentType::TEXT_PLAIN)
                .body(String::from("Follow the link to reset your password"))
                .unwrap();

            let creds = Credentials::new("smtp_username".to_owned(), "smtp_password".to_owned());

            // Open a remote connection to gmail
            let mailer = SmtpTransport::relay("smtp.gmail.com")
                .unwrap()
                .credentials(creds)
                .build();

            // Send the email
            match mailer.send(&email) {
                Ok(_) => Ok((
                    StatusCode::OK,
                    Json(json!({"message": "recovery email has been sent"})),
                )),
                Err(e) => Ok((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"message": format!("email cannot be sent")})),
                )),
            }
        }
        None => Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": format!("no user")})),
        )),
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

    let data = server_config.firebase.at("users");
    let users = data.get::<Vec<User>>().await.ok().unwrap();
    let user = users.iter().find(|f| f.username == decode.claims.sub);

    match user {
        Some(s) => {
            let claims = Claims {
                sub: s.username.to_string(),
                exp: (chrono::Utc::now() + chrono::Duration::minutes(15)).timestamp() as usize,
            };

            let access = encode_jwt(&claims, "secret").unwrap();

            Ok((
                StatusCode::OK,
                Json(json!({"access": access, "refresh": &refresh_request.refresh_token})),
            ))
        }
        None => Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "refresh token fail"})),
        )),
    }
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
    decode(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
}
