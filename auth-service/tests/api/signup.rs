use crate::helpers::{get_random_email, TestApp};
use auth_service::routes::SignupResponse;

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email(); //todo!(); // Call helper method to generate email

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
    //assert_eq!(response.status().as_u16(), 201);
    let app = TestApp::new().await;

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
