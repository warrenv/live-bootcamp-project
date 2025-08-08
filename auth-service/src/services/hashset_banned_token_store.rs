use std::collections::HashSet;

use crate::domain::{BannedTokenStore, BannedTokenStoreError};

#[derive(Clone, Default, Eq, PartialEq, Debug)]
pub struct HashsetBannedTokenStore {
    pub tokens: HashSet<String>,
}

#[async_trait::async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {
    async fn add_token(&mut self, token: String) -> Result<(), BannedTokenStoreError> {
        println!("in add_token");
        println!("adding token : {:?}", token);
        self.tokens.insert(token.clone());
        println!("add tokens NOW {:?}", self.tokens);
        Ok(())
    }

    async fn get_token(&self, token: &String) -> bool {
        println!("            tokens: {:?}", self.tokens);
        println!("         get token: {:?}", token);
        self.tokens.contains(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_token() {
        let mut banned_tokens_store = HashsetBannedTokenStore::default();
        let token = "one_used_token".to_string();

        // Test adding a new user
        let result = banned_tokens_store.add_token(token.clone()).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_existing_token() {
        let mut banned_tokens_store = HashsetBannedTokenStore::default();
        let token = "another_used_token".to_string();
        let _ = banned_tokens_store.add_token(token.clone()).await;

        let result = banned_tokens_store.get_token(&token).await;
        assert_eq!(result, true);
    }

    #[tokio::test]
    async fn test_get_nonexistent_token() {
        let banned_tokens_store = HashsetBannedTokenStore::default();
        let result = banned_tokens_store.get_token(&"foo".to_string()).await;
        assert_eq!(result, false);
    }
}
