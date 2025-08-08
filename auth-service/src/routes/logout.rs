use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;

use crate::{
    app_state::AppState,
    domain::AuthAPIError,
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
};

pub async fn logout(
    State(state): State<AppState>,
    jar: CookieJar,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let cookie = match jar.get(JWT_COOKIE_NAME) {
        Some(x) => x,
        None => {
            return (jar, Err(AuthAPIError::MissingToken));
        }
    };

    let token = cookie.value().to_owned();

    let _ = match validate_token(&token).await {
        Ok(x) => x,
        Err(_) => return (jar, Err(AuthAPIError::InvalidToken)),
    };

    let jar = jar.clone().remove(JWT_COOKIE_NAME);

    let mut banned_token_store = state.banned_token_store.write().await;
    println!("logout route token: {:?}", token);
    let _ = banned_token_store.add_token(token.clone()).await;
    println!(
        "token set? {:?}",
        banned_token_store.get_token(&token).await
    );

    (jar, Ok(StatusCode::OK))
}
