use crate::helpers::{get_random_email, TestApp};
use auth_service::routes::SignupResponse;
use auth_service::services::hashset_banned_token_store::HashsetBannedTokenStore;
use auth_service::ErrorResponse;

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new(HashsetBannedTokenStore::default()).await;

    let random_email = get_random_email(); // Call helper method to generate email

    // TODO: add more malformed input test cases
    let test_cases = [serde_json::json!({
        "password": "password123",
        "requires2FA": true
    })];

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case); // call `post_signup`
        assert_eq!(
            response.await.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_return_201_if_valid_input() {
    let app = TestApp::new(HashsetBannedTokenStore::default()).await;

    let expected_response = SignupResponse {
        message: "User created successfully!".to_owned(),
    };

    let user = serde_json::json!({
        "email": "foo@example.com",
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&user);

    // Assert that we are getting the correct response body!
    assert_eq!(
        response
            .await
            .json::<SignupResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
        expected_response
    );
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new(HashsetBannedTokenStore::default()).await;

    let input = [
        serde_json::json!({
            "email": "foo",
            "password": "password123",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": "foo@example.com",
            "password": "pass",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": "",
            "password": "password1",
            "requires2FA": true
        }),
    ];

    for i in input.iter() {
        let response = app.post_signup(i).await;
        assert_eq!(response.status().as_u16(), 400, "Failed for input: {:?}", i);

        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "Invalid credentials".to_owned()
        );
    }
}

#[tokio::test]
async fn should_return_409_if_email_already_exists() {
    let app = TestApp::new(HashsetBannedTokenStore::default()).await;

    let user = serde_json::json!({
        "email": "foo@example.com",
        "password": "password123",
        "requires2FA": true
    });

    let dup_user = serde_json::json!({
        "email": "foo@example.com",
        "password": "Pxxx012345678",
        "requires2FA": true
    });

    let _ = app.post_signup(&user).await;
    let response = app.post_signup(&dup_user).await;

    //assert_eq!(response0.await.status().as_u16(), 409);
    assert_eq!(response.status().as_u16(), 409);

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "User already exists".to_owned()
    );
}
