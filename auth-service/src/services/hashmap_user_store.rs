use std::collections::HashMap;

use crate::domain::{Email, Password, User, UserStore, UserStoreError};

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<Email, User>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }
        self.users.insert(user.email.clone(), user);
        Ok(())
    }

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        match self.users.get(email) {
            Some(user) => Ok(user.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<(), UserStoreError> {
        match self.users.get(email) {
            Some(user) => {
                if user.password.eq(password) {
                    Ok(())
                } else {
                    Err(UserStoreError::InvalidCredentials)
                }
            }
            None => Err(UserStoreError::UserNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut user_store = HashmapUserStore::default();
        let user = User {
            email: Email::parse("test@example.com".to_owned()).unwrap(),
            password: Password::parse("password".to_owned()).unwrap(),
            requires_2fa: false,
        };

        // Test adding a new user
        let result = user_store.add_user(user.clone()).await;
        assert!(result.is_ok());

        // Test adding an existing user
        let result = user_store.add_user(user).await;
        assert_eq!(result, Err(UserStoreError::UserAlreadyExists));
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut user_store = HashmapUserStore::default();
        let email = Email::parse("test@example.com".to_owned()).unwrap();

        let user = User {
            email: email.clone(),
            password: Password::parse("password".to_owned()).unwrap(),
            requires_2fa: false,
        };

        // Test getting a user that exists
        user_store.users.insert(email.clone(), user.clone());
        let result = user_store.get_user(&email).await;
        assert_eq!(result, Ok(user));

        // Test getting a user that doesn't exist
        let result = user_store
            .get_user(&Email::parse("nonexistent@example.com".to_owned()).unwrap())
            .await;

        assert_eq!(result, Err(UserStoreError::UserNotFound));
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut user_store = HashmapUserStore::default();
        let email = Email::parse("test@example.com".to_owned()).unwrap();
        let password = Password::parse("password".to_owned()).unwrap();

        let user = User {
            email: email.clone(),
            password: password.clone(),
            requires_2fa: false,
        };

        // Test validating a user that exists with correct password
        user_store.users.insert(email.clone(), user.clone());
        let result = user_store.validate_user(&email, &password).await;
        assert_eq!(result, Ok(()));

        // Test validating a user that exists with incorrect password
        let wrong_password = Password::parse("wrongpassword".to_owned()).unwrap();
        let result = user_store.validate_user(&email, &wrong_password).await;
        assert_eq!(result, Err(UserStoreError::InvalidCredentials));

        // Test validating a user that doesn't exist
        let result = user_store
            .validate_user(
                &Email::parse("nonexistent@example.com".to_string()).unwrap(),
                &password,
            )
            .await;

        assert_eq!(result, Err(UserStoreError::UserNotFound));
    }
}

//use crate::domain::{Email, Password, User, UserStore, UserStoreError};
//
//use std::collections::HashMap;
//
//#[derive(Debug, Default)]
//pub struct HashmapUserStore {
//    users: HashMap<Email, User>,
//}
//
//#[async_trait::async_trait]
//impl UserStore for HashmapUserStore {
//    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
//        if self.users.contains_key(&user.email) {
//            return Err(UserStoreError::UserAlreadyExists);
//        }
//        self.users.insert(user.email.clone(), user);
//        Ok(())
//    }
//
//    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
//        match self.users.get(email) {
//            Some(user) => Ok(user.clone()),
//            None => Err(UserStoreError::UserNotFound),
//        }
//    }
//
//    async fn validate_user(
//        &self,
//        email: &Email,
//        password: &Password,
//    ) -> Result<(), UserStoreError> {
//        match self.users.get(email) {
//            Some(user) => {
//                if user.password.eq(password) {
//                    Ok(())
//                } else {
//                    Err(UserStoreError::InvalidCredentials)
//                }
//            }
//            None => Err(UserStoreError::UserNotFound),
//        }
//    }
//}
//
//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[tokio::test]
//    async fn test_add_user_should_return_ok_when_email_does_not_exist() {
//        let expected = true;
//        let mut user_store = HashmapUserStore::default();
//        let user = User::new("foo@test.com".to_string(), "secretz".to_string(), true);
//
//        let actual = user_store.add_user(user).await;
//
//        assert_eq!(actual.is_ok(), expected);
//    }
//
//    #[tokio::test]
//    async fn test_add_user_should_fail_when_email_exists() {
//        let expected = true;
//        //Xlet mut users = HashmapUserStore::default();
//        let mut user_store = HashmapUserStore::default();
//        let user = User::new("foo@test.com".to_string(), "secretz".to_string(), true);
//        let user2 = User::new("foo@test.com".to_string(), "secretz".to_string(), true);
//
//        //Xlet actual = users.add_user(user);
//        //Xlet actual = users.add_user(user2);
//        let actual = user_store.add_user(user).await;
//        let actual = user_store.add_user(user2).await;
//
//        assert_eq!(actual.is_err(), expected);
//    }
//
//    #[tokio::test]
//    async fn test_get_user_should_return_a_user_when_user_exists() {
//        let expected = true;
//        //Xlet mut users = HashmapUserStore::default();
//        let mut user_store = HashmapUserStore::default();
//        let user = User::new("foo@test.com".to_string(), "secretz".to_string(), true);
//        let _ = user_store.add_user(user.clone()).await;
//
//        let actual = user_store.get_user(&user.email.clone()).await;
//
//        assert_eq!(actual.is_ok(), expected);
//    }
//
//    #[tokio::test]
//    async fn test_get_user_should_return_an_error_when_user_email_does_not_exist() {
//        let expected = UserStoreError::UserNotFound;
//        //Xlet mut users = HashmapUserStore::default();
//        let mut user_store = HashmapUserStore::default();
//        let user = User::new("foo@test.com".to_string(), "secretz".to_string(), true);
//        let _ = user_store.add_user(user.clone()).await;
//
//        let actual = user_store.get_user(&"baz.example.com".to_string()).await;
//
//        assert!(matches!(actual, Err(expected)));
//    }
//
//    #[tokio::test]
//    async fn test_validate_user_given_an_existing_user_and_matching_password_should_return_ok() {
//        let expected = true;
//        //Xlet mut users = HashmapUserStore::default();
//        let mut user_store = HashmapUserStore::default();
//        let user = User::new("foo@test.com".to_string(), "secretz".to_string(), true);
//        let _ = user_store.add_user(user.clone()).await;
//
//        let actual = user_store
//            .validate_user(&user.email.clone(), &user.password.clone())
//            .await;
//
//        assert_eq!(actual.is_ok(), expected);
//    }
//
//    #[tokio::test]
//    async fn test_validate_user_given_an_existing_user_and_mismatched_password_should_return_error()
//    {
//        let expected = UserStoreError::InvalidCredentials;
//        //Xlet mut users = HashmapUserStore::default();
//        let mut user_store = HashmapUserStore::default();
//        let user = User::new("foo@test.com".to_string(), "secretz".to_string(), true);
//        let _ = user_store.add_user(user.clone()).await;
//
//        let actual = user_store
//            .validate_user(&user.email.clone(), &"bad_passwd")
//            .await;
//
//        assert!(matches!(actual, Err(expected)));
//    }
//
//    #[tokio::test]
//    async fn test_validate_user_given_a_nonexisting_user_should_return_error() {
//        let expected = UserStoreError::UserNotFound;
//        //Xlet mut users = HashmapUserStore::default();
//        let mut user_store = HashmapUserStore::default();
//        let user = User::new("foo@test.com".to_string(), "secretz".to_string(), true);
//        let _ = user_store.add_user(user.clone()).await;
//
//        let actual = user_store
//            .validate_user(&"unknown email", &"bad_passwd")
//            .await;
//
//        assert!(matches!(actual, Err(expected)));
//    }
//}
