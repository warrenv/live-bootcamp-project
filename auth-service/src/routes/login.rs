use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password},
    utils::auth::generate_auth_cookie,
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
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let email = match Email::parse(request.email.clone()) {
        Ok(x) => x,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let password = match Password::parse(request.password.clone()) {
        Ok(x) => x,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let user_store = &state.user_store.read().await;

    if user_store.validate_user(&email, &password).await.is_err() {
        return (jar.clone(), Err(AuthAPIError::IncorrectCredentials));
    }

    if user_store.get_user(&email).await.is_err() {
        return (jar.clone(), Err(AuthAPIError::IncorrectCredentials));
    }

    let auth_cookie = match generate_auth_cookie(&email) {
        Ok(x) => x,
        Err(_) => return (jar.clone(), Err(AuthAPIError::UnexpectedError)),
    };

    let updated_jar = jar.clone().add(auth_cookie);

    (updated_jar, Ok(StatusCode::OK.into_response()))
}
