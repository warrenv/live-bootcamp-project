use crate::helpers::TestApp;
use auth_service::services::hashset_banned_token_store::HashsetBannedTokenStore;
use auth_service::services::mock_email_client::MockEmailClient;

#[tokio::test]
async fn verify_2fa_returns_200() {
    let app = TestApp::new(HashsetBannedTokenStore::default(), MockEmailClient {}).await;
    let response = app.verify_2fa().await;
    assert_eq!(response.status().as_u16(), 200);
}
