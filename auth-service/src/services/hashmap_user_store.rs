use crate::domain::User;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Debug, Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        match self.users.get(&user.email) {
            Some(_) => Err(UserStoreError::UserAlreadyExists),
            None => {
                self.users.insert(user.email.clone(), user);
                Ok(())
            }
        }
    }

    pub fn get_user(&self, email: &str) -> Result<&User, UserStoreError> {
        match self.users.get(email) {
            Some(u) => Ok(u),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        match self.get_user(email) {
            Ok(u) => {
                if u.password == password {
                    Ok(())
                } else {
                    Err(UserStoreError::InvalidCredentials)
                }
            }
            Err(_) => Err(UserStoreError::UserNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user_should_return_ok_when_email_does_not_exist() {
        let expected = true;
        let mut users = HashmapUserStore::default();
        let user = User::new("foo@test.com".to_string(), "secretz".to_string(), true);

        let actual = users.add_user(user);

        assert_eq!(actual.is_ok(), expected);
    }

    #[tokio::test]
    async fn test_add_user_should_fail_when_email_exists() {
        let expected = true;
        let mut users = HashmapUserStore::default();
        let user = User::new("foo@test.com".to_string(), "secretz".to_string(), true);
        let user2 = User::new("foo@test.com".to_string(), "secretz".to_string(), true);

        let actual = users.add_user(user);
        let actual = users.add_user(user2);

        assert_eq!(actual.is_err(), expected);
    }

    #[tokio::test]
    async fn test_get_user_should_return_a_user_when_user_exists() {
        let expected = true;
        let mut users = HashmapUserStore::default();
        let user = User::new("foo@test.com".to_string(), "secretz".to_string(), true);
        let _ = users.add_user(user.clone());

        let actual = users.get_user(&user.email.clone());

        assert_eq!(actual.is_ok(), expected);
    }

    #[tokio::test]
    async fn test_get_user_should_return_an_error_when_user_email_does_not_exist() {
        let expected = UserStoreError::UserNotFound;
        let mut users = HashmapUserStore::default();
        let user = User::new("foo@test.com".to_string(), "secretz".to_string(), true);
        let _ = users.add_user(user.clone());

        let actual = users.get_user(&"baz.example.com".to_string());

        assert!(matches!(actual, Err(expected)));
    }

    #[tokio::test]
    async fn test_validate_user_given_an_existing_user_and_matching_password_should_return_ok() {
        let expected = true;
        let mut users = HashmapUserStore::default();
        let user = User::new("foo@test.com".to_string(), "secretz".to_string(), true);
        let _ = users.add_user(user.clone());

        let actual = users.validate_user(&user.email.clone(), &user.password.clone());

        assert_eq!(actual.is_ok(), expected);
    }

    #[tokio::test]
    async fn test_validate_user_given_an_existing_user_and_mismatched_password_should_return_error()
    {
        let expected = UserStoreError::InvalidCredentials;
        let mut users = HashmapUserStore::default();
        let user = User::new("foo@test.com".to_string(), "secretz".to_string(), true);
        let _ = users.add_user(user.clone());

        let actual = users.validate_user(&user.email.clone(), &"bad_passwd");

        assert!(matches!(actual, Err(expected)));
    }

    #[tokio::test]
    async fn test_validate_user_given_a_nonexisting_user_should_return_error() {
        let expected = UserStoreError::UserNotFound;
        let mut users = HashmapUserStore::default();
        let user = User::new("foo@test.com".to_string(), "secretz".to_string(), true);
        let _ = users.add_user(user.clone());

        let actual = users.validate_user(&"unknown email", &"bad_passwd");

        assert!(matches!(actual, Err(expected)));
    }
}
