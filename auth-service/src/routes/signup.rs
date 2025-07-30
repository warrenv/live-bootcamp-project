use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, User},
};

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct SignupResponse {
    pub message: String,
}

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    println!("IN SIGNUP");
    let email = request.email;
    let password = request.password;

    // - email is empty or does not contain '@'
    if email == "".to_string() || !email.contains("@") {
        println!("EMAIL CHECKS FAILED");
        return Err(AuthAPIError::InvalidCredentials);
    }

    // - password is less than 8 characters
    if password.len() < 8 {
        println!("SHORT PASSWORD");
        return Err(AuthAPIError::InvalidCredentials);
    }

    let user = User::new(email.clone(), password, request.requires_2fa);

    let mut user_store = state.user_store.write().await;

    // TODO: early return AuthAPIError::UserAlreadyExists if email exists in user_store.
    match user_store.get_user(&email.clone()) {
        Ok(_) => {
            println!("USER ALREADY EXISTS: {:?}", email);
            return Err(AuthAPIError::UserAlreadyExists);
        }
        Err(_) => {
            println!("USER NOT FOUND");
        }
    }

    //
    // TODO: instead of using unwrap, early return
    // AuthAPIError::UnexpectedError if add_user() fails.

    match user_store.add_user(user) {
        Ok(_) => {
            let response = Json(SignupResponse {
                message: "User created successfully!".to_string(),
            });
            println!("USER CREATED");

            Ok((StatusCode::CREATED, response))
        }
        Err(_) => {
            println!("USER CREATE FAILED");
            Err(AuthAPIError::UnexpectedError)
        }
    }
}
