use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password, User},
};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

pub enum LoginError {
    AuthAPIError,
    StatusCode(StatusCode),
}

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email =
        Email::parse(request.email.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password =
        Password::parse(request.password.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;

    let user_store = &state.user_store.read().await;

    // TODO: call `user_store.validate_user` and return
    // `AuthAPIError::IncorrectCredentials` if valudation fails.
    user_store
        .validate_user(&email, &password)
        .await
        .map_err(|_| AuthAPIError::IncorrectCredentials)?;

    // TODO: call `user_store.get_user`. Return AuthAPIError::IncorrectCredentials if the operation fails.
    //let user = todo!();
    let user = user_store
        .get_user(&email)
        .await
        .map_err(|_| AuthAPIError::IncorrectCredentials)?;

    Ok(StatusCode::OK.into_response())
}
