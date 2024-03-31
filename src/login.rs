use axum::{http::StatusCode, Json};
use chrono::{Duration, Local};
use firebase_rs::Firebase;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}
#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    sub: String,
    iat: i64,
    exp: i64,
}

pub async fn login(
    Json(loginRequest): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let firebase = Firebase::new("https://infs803-group7-default-rtdb.firebaseio.com/").unwrap();
    let data = firebase.at("users");
    // let user = data.set::<User>(&user).await;

    let mut header = Header::new(jsonwebtoken::Algorithm::RS256);
    header.typ = Some("JWT".to_string());

    let now = Local::now();
    let iat = now.timestamp();
    let exp = (now + Duration::hours(1)).timestamp();
    let claims = Claims {
        sub: loginRequest.username,
        iat,
        exp,
    };

    let secret = "secret";

    let token = encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    );

    let login_response = LoginResponse {
        token: token.unwrap(),
    };
    // let json_response = serde_json::json!({"jwt": loginResponse});

    Ok(Json(login_response))
}

// pub fn signup() -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
//     let firebase = Firebase::new("https://infs803-group7-default-rtdb.firebaseio.com/").unwrap();
//     let data = firebase.at("users");
//     let user = data.set::<User>(&user).await;
//     match user {
//         Ok(_) => Ok((
//             StatusCode::OK,
//             Json(serde_json::json!({"message": "create user successful"})),
//         )),
//         Err(e) => Ok((
//             StatusCode::INTERNAL_SERVER_ERROR,
//             Json(serde_json::json!({"message": format!("create user fail: { }", e)})),
//         )),
//     }
// }
