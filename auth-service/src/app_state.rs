//xuse std::sync::Arc;
//xuse tokio::sync::RwLock;
//x
//xuse crate::services::hashmap_user_store::HashmapUserStore;
//x
//x// Using a type alias to improve readability!
//xpub type UserStoreType = Arc<RwLock<HashmapUserStore>>;
//x
//x#[derive(Clone)]
//xpub struct AppState {
//x    pub user_store: UserStoreType,
//x}
//x
//ximpl AppState {
//x    pub fn new(user_store: UserStoreType) -> Self {
//x        Self { user_store }
//x    }
//x}

use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::UserStore;

pub type UserStoreType = Arc<RwLock<dyn UserStore + Send + Sync>>;

#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStoreType,
}

impl AppState {
    pub fn new(user_store: UserStoreType) -> Self {
        Self { user_store }
    }
}
