use auth_service::ErrorResponse;
use serde::{Deserialize, Serialize};

use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;

    #[derive(Debug, Serialize, Deserialize)]
    struct EmptyBody {}

    let response = app.post_login(&EmptyBody {}).await;

    assert_eq!(response.status().as_u16(), 422);
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    // Call the log-in route with invalid credentials and assert that a
    // 400 HTTP status code is returned along with the appropriate error message.

    let app = TestApp::new().await;

    let invalid_creds = serde_json::json!({
        "email": "fooexample.com".to_string(),
        "password": "12345678".to_string(),
    });

    let response = app.post_login(&invalid_creds).await;

    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    // Call the log-in route with incorrect credentials and assert
    // that a 401 HTTP status code is returned along with the appropriate error message.

    let app = TestApp::new().await;

    let user = serde_json::json!({
        "email": "foo@example.com",
        "password": "password123",
        "requires2FA": true
    });

    let invalid_creds = serde_json::json!({
        "email": "foo@example.com",
        "password": "not-the-same",
        "requires2FA": true
    });

    let _ = app.post_signup(&user).await;

    let response = app.post_login(&invalid_creds).await;

    assert_eq!(response.status().as_u16(), 401);
}
