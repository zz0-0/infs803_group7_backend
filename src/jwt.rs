use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    BoxError, Json,
};
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::ServerConfig;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    sub: String,
    exp: usize,
}

// pub async fn login_account(
//     State(server_config): State<ServerConfig>,
// ) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
//     let data = server_config.firebase.at("token");
//     // let users = data.get::<Vec<User>>().await;

//     match users {
//         Ok(_) => Ok((StatusCode::OK, Json(json_response))),
//         Err(e) => Ok((
//             StatusCode::INTERNAL_SERVER_ERROR,
//             Json(serde_json::json!({"message": format!("list users fail: { }", e)})),
//         )),
//     }
// }

// pub async fn authenticate(
//     State(server_config): State<ServerConfig>,
// ) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
//     let secret = "secret";
//     // let decode = decode_jwt(token, secret);
//     let claims = &Claims { sub: (), exp: () };
//     let encode = encode_jwt(claims, secret);
//     match encode {
//         Ok(_) => todo!(),
//         Err(_) => todo!(),
//     }
// }

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

// pub async fn refresh_token(
//     State(server_config): State<ServerConfig>,
// ) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
//     let data = server_config.firebase.at("token");

//     match users {
//         Ok(_) => Ok((StatusCode::OK, Json(json_response))),
//         Err(e) => Ok((
//             StatusCode::INTERNAL_SERVER_ERROR,
//             Json(serde_json::json!({"message": format!("list users fail: { }", e)})),
//         )),
//     }
// }

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
