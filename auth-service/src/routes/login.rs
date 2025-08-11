use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, LoginAttemptId, Password, TwoFACode},
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
    //println!("email: {:?}", email);

    let password = match Password::parse(request.password.clone()) {
        Ok(x) => x,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };
    //println!("password: {:?}", password);

    let user_store = &state.user_store.read().await;

    if user_store.validate_user(&email, &password).await.is_err() {
        return (jar.clone(), Err(AuthAPIError::IncorrectCredentials));
    }
    //println!("user_store validate_user ok");

    //    if user_store.get_user(&email).await.is_err() {
    //        return (jar.clone(), Err(AuthAPIError::IncorrectCredentials));
    //    }
    let user = match user_store.get_user(&email).await {
        Ok(user) => user,
        Err(_) => return (jar, Err(AuthAPIError::IncorrectCredentials)),
    };
    //println!("got user: {:?}", user);

    let auth_cookie = match generate_auth_cookie(&email) {
        Ok(x) => x,
        Err(_) => return (jar.clone(), Err(AuthAPIError::UnexpectedError)),
    };
    //println!("got auth_cookie: {:?}", auth_cookie);

    let jar = jar.clone().add(auth_cookie);

    //(updated_jar, Ok(StatusCode::OK.into_response()))
    match user.requires_2fa {
        //        true => handle_2fa(updated_jar).await,
        //        false => handle_no_2fa(&user.email, updated_jar).await,
        true => handle_2fa(&user.email, &state, jar).await,
        false => handle_no_2fa(&user.email, jar).await,
    }
}

async fn handle_2fa(
    email: &Email,
    state: &AppState,
    jar: CookieJar,
) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
    println!(">>> entered handle_2fa");
    let login_attempt_id = LoginAttemptId::default(); //todo!();
    let two_fa_code = TwoFACode::default(); //todo!();

    let mut two_fa_code_store = state.two_fa_code_store.write().await;
    if two_fa_code_store
        .add_code(email.clone(), login_attempt_id.clone(), two_fa_code)
        .await
        .is_err()
    {
        return (jar, Err(AuthAPIError::UnexpectedError));
    };

    let email_client = state.email_client.read().await;
    if email_client
        .send_email(
            &Email::parse("baz@example.com".to_string()).unwrap(),
            &"2fa subject".to_string(),
            &"2fa content".to_string(),
        )
        .await
        .is_err()
    {
        return (jar, Err(AuthAPIError::UnexpectedError));
    };

    println!("CREATING response");
    let response = Json(LoginResponse::TwoFactorAuth(TwoFactorAuthResponse {
        message: "2FA required".to_owned(),
        login_attempt_id: login_attempt_id.clone().to_string(),
    }));

    (jar, Ok((StatusCode::PARTIAL_CONTENT, response)))
}

// New!
async fn handle_no_2fa(
    _: &Email,
    jar: CookieJar,
) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
    (jar, Ok((StatusCode::OK, Json(LoginResponse::RegularAuth))))
}
