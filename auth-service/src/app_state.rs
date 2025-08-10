use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::{BannedTokenStore, TwoFACodeStore, UserStore};

pub type UserStoreType = Arc<RwLock<dyn UserStore + Send + Sync>>;
pub type BannedtokenStoreType = Arc<RwLock<dyn BannedTokenStore + Send + Sync>>;
pub type TwoFACodeStoreType = Arc<RwLock<dyn TwoFACodeStore + Send + Sync>>;

#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStoreType,
    pub banned_token_store: BannedtokenStoreType,
    pub two_fa_code_store: TwoFACodeStoreType,
}

impl AppState {
    pub fn new(
        user_store: UserStoreType,
        banned_token_store: BannedtokenStoreType,
        two_fa_code_store: TwoFACodeStoreType,
    ) -> Self {
        Self {
            user_store,
            banned_token_store,
            two_fa_code_store,
        }
    }
}
