use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{domain::AuthAPIError, utils::auth::validate_token};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct VerifyTokenRequest {
    token: String,
}

pub async fn verify_token(Json(request): Json<VerifyTokenRequest>) -> impl IntoResponse {
    println!("in verify_token: {}", request.token);

    if let Err(_) = validate_token(&request.token).await {
        println!("invalid token in verify_token: {}", request.token);
        return Err(AuthAPIError::InvalidToken);
    };

    Ok(StatusCode::OK.into_response())
}
