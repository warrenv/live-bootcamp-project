use std::collections::HashMap;

use crate::domain::{
    data_stores::{LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError},
    email::Email,
};

#[derive(Default)]
pub struct HashmapTwoFACodeStore {
    codes: HashMap<Email, (LoginAttemptId, TwoFACode)>,
}

// TODO: implement TwoFACodeStore for HashmapTwoFACodeStore
#[async_trait::async_trait]
impl TwoFACodeStore for HashmapTwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        self.codes.insert(email, (login_attempt_id, code));
        Ok(())
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        match self.codes.remove(email) {
            Some(_) => Ok(()),
            None => Err(TwoFACodeStoreError::EmailNotFound),
        }
    }

    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        match self.codes.get(email) {
            Some(x) => Ok(x.clone()),
            None => Err(TwoFACodeStoreError::UnexpectedError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn is_should_add_code_to_store_when_inputs_are_valid() {
        let expected = true;
        let mut twofa_store = HashmapTwoFACodeStore::default();
        let email = Email::parse("foo@example.com".to_string()).unwrap();
        let login_attempt_id = LoginAttemptId::default();
        let code = TwoFACode::default();

        let actual = twofa_store.add_code(email, login_attempt_id, code).await;

        assert_eq!(actual.is_ok(), expected);
    }

    #[tokio::test]
    async fn is_should_get_from_store_when_email_is_added() {
        let expected = true;
        let mut twofa_store = HashmapTwoFACodeStore::default();
        let email = Email::parse("foo@example.com".to_string()).unwrap();
        let login_attempt_id = LoginAttemptId::default();
        let code = TwoFACode::default();

        let _ = twofa_store
            .add_code(email.clone(), login_attempt_id, code)
            .await;
        let actual = twofa_store.get_code(&email).await;

        assert_eq!(actual.is_ok(), expected);
    }

    #[tokio::test]
    async fn is_should_remove_from_store_when_email_added() {
        let expected = true;
        let mut twofa_store = HashmapTwoFACodeStore::default();
        let email = Email::parse("foo@example.com".to_string()).unwrap();
        let login_attempt_id = LoginAttemptId::default();
        let code = TwoFACode::default();

        let _ = twofa_store
            .add_code(email.clone(), login_attempt_id, code)
            .await;
        let actual = twofa_store.remove_code(&email).await;

        assert_eq!(actual.is_ok(), expected);
    }

    #[tokio::test]
    async fn is_should_error_removing_from_store_when_email_not_added() {
        let expected = true;
        let mut twofa_store = HashmapTwoFACodeStore::default();
        let email = Email::parse("foo@example.com".to_string()).unwrap();

        let actual = twofa_store.remove_code(&email).await;

        assert_eq!(actual.is_err(), expected);
    }
}
