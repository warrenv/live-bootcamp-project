use std::sync::Arc;
use tokio::sync::RwLock;

use auth_service::{
    app_state::{AppState, UserStoreType},
    services::hashmap_user_store::HashmapUserStore,
    utils::constants::prod,
    Application,
};

#[tokio::main]
async fn main() {
    let user_store: UserStoreType = Arc::new(RwLock::new(HashmapUserStore::default()));
    let app_state = AppState::new(user_store);

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}
