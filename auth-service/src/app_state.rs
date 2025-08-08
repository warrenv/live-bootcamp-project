use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::{BannedTokenStore, UserStore};

pub type UserStoreType = Arc<RwLock<dyn UserStore + Send + Sync>>;
pub type BannedtokenStoreType = Arc<RwLock<dyn BannedTokenStore + Send + Sync>>;

#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStoreType,
    pub banned_token_store: BannedtokenStoreType,
}

impl AppState {
    pub fn new(user_store: UserStoreType, banned_token_store: BannedtokenStoreType) -> Self {
        Self {
            user_store,
            banned_token_store,
        }
    }
}
