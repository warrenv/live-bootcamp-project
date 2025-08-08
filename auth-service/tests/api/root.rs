use crate::helpers::TestApp;
use auth_service::services::hashset_banned_token_store::HashsetBannedTokenStore;

// Tokio's test macro is used to run the test in an async environment
#[tokio::test]
async fn root_returns_auth_ui() {
    let app = TestApp::new(HashsetBannedTokenStore::default()).await;

    let response = app.get_root().await;

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}
