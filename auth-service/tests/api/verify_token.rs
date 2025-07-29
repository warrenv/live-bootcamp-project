use crate::helpers::TestApp;

#[tokio::test]
async fn verify_token_returns_200() {
    let app = TestApp::new().await;

    let response = app.verify_token().await;

    assert_eq!(response.status().as_u16(), 200);
    //     assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}
