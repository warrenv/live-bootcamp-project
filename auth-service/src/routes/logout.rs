use axum::{http::StatusCode, response::IntoResponse};
use axum_extra::extract::{cookie, CookieJar};

use crate::{
    domain::AuthAPIError,
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
};

pub async fn logout(jar: CookieJar) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    // Retrieve JWT cookie from the `CookieJar`
    // Return AuthAPIError::MissingToken is the cookie is not found
    println!("JAR: {:?}", jar);
    let cookie = match jar.get(JWT_COOKIE_NAME) {
        Some(x) => {
            println!("COOKIE: {:?}", x);
            x
        }
        None => {
            println!("NO COOKIE");
            return (jar, Err(AuthAPIError::MissingToken));
        }
    };

    let token = cookie.value().to_owned();

    // TODO: Validate JWT token by calling `validate_token` from the auth service.
    // If the token is valid you can ignore the returned claims for now.
    let _ = match validate_token(&token).await {
        Ok(x) => x,
        Err(_) => return (jar, Err(AuthAPIError::InvalidToken)),
    };

    let jar = jar.clone().remove(JWT_COOKIE_NAME);

    return (jar, Ok(StatusCode::OK));
}
