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

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum LoginResponse {
    RegularAuth,
    TwoFactorAuth(TwoFactorAuthResponse),
}

// If a user requires 2FA, this JSON body should be returned!
#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorAuthResponse {
    pub message: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
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

    //    if user_store.get_user(&email).await.is_err() {
    //        return (jar.clone(), Err(AuthAPIError::IncorrectCredentials));
    //    }
    let user = match user_store.get_user(&email).await {
        Ok(user) => user,
        Err(_) => return (jar, Err(AuthAPIError::IncorrectCredentials)),
    };

    let auth_cookie = match generate_auth_cookie(&email) {
        Ok(x) => x,
        Err(_) => return (jar.clone(), Err(AuthAPIError::UnexpectedError)),
    };

    let updated_jar = jar.clone().add(auth_cookie);

    //(updated_jar, Ok(StatusCode::OK.into_response()))
    match user.requires_2fa {
        true => handle_2fa(updated_jar).await,
        false => handle_no_2fa(&user.email, updated_jar).await,
    }
}

async fn handle_2fa(
    jar: CookieJar,
) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
    // TODO: Return a TwoFactorAuthResponse. The message should be "2FA required".
    // The login attempt ID should be "123456". We will replace this hard-coded login attempt ID soon!
    //todo!()
    (
        jar,
        Ok((
            StatusCode::PARTIAL_CONTENT,
            Json(LoginResponse::TwoFactorAuth(TwoFactorAuthResponse {
                message: "2FA required".to_string(),
                login_attempt_id: "123456".to_string(),
            })),
        )),
    )
}

// New!
async fn handle_no_2fa(
    email: &Email,
    jar: CookieJar,
) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
    (jar, Ok((StatusCode::OK, Json(LoginResponse::RegularAuth))))
}
