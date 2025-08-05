use auth_service::{utils::constants::JWT_COOKIE_NAME, ErrorResponse};
use reqwest::Url;

use crate::helpers::{get_random_email, TestApp};
use auth_service::domain::Email;
use auth_service::utils::auth::generate_auth_cookie;

#[tokio::test]
async fn should_return_200_if_valid_jwt_cookie() {
    let app = TestApp::new().await;
    let email = Email::parse("foo@example.com".to_string()).unwrap();

    app.cookie_jar.add_cookie_str(
        &generate_auth_cookie(&email).unwrap().to_string(),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let response = app.logout().await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn should_return_400_if_logout_called_twice_in_a_row() {
    //todo!()
    let app = TestApp::new().await;
    let email = Email::parse("foo@example.com".to_string()).unwrap();

    app.cookie_jar.add_cookie_str(
        &generate_auth_cookie(&email).unwrap().to_string(),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let _ = app.logout().await;
    let response = app.logout().await;

    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    let app = TestApp::new().await;
    let response = app.logout().await;
    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    // add invalid cookie - it's not a jwt.
    app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let response = app.logout().await;

    assert_eq!(response.status().as_u16(), 401);
}
