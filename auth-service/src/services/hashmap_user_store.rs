use crate::domain::User;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn new() -> Self {
        HashmapUserStore {
            users: HashMap::new(),
        }
    }
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }

        self.users.insert(user.email.clone(), user);
        Ok(())
    }
    pub fn get_user(&self, email: &str) -> Result<&User, UserStoreError> {
        let user = self.users.get(email);
        match user {
            Some(user) => Ok(user),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    pub fn validate_user(&mut self, user: User) -> Result<(), UserStoreError> {
        let stored_user = self.get_user(&user.email);
        match stored_user {
            Ok(stored_user) => {
                if stored_user.password == user.password {
                    Ok(())
                } else {
                    Err(UserStoreError::InvalidCredentials)
                }
            }
            Err(UserStoreError::UserNotFound) => Err(UserStoreError::UserNotFound),
            Err(_) => Err(UserStoreError::UnexpectedError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_user() {
        // Create a new store instance
        let mut store = HashmapUserStore::new();

        // Create a test user
        let user = User::new(
            "test@example.com".to_string(),
            "test_password".to_string(),
            false,
        );
        let mut result = store.add_user(user.clone());
        assert!(result.is_ok());

        // Try to add the same user again
        result = store.add_user(user);
        assert_eq!(result.err(), Some(UserStoreError::UserAlreadyExists));
    }

    #[test]
    fn get_nonexistent_user_returns_error() {
        let store = HashmapUserStore::new();
        let email = "test@example.com";

        let result = store.get_user(email);
        assert_eq!(result.err(), Some(UserStoreError::UserNotFound));
    }

    #[test]
    fn validate_user_returns_user_not_found_err() {
        let mut store = HashmapUserStore::new();

        // Create a test user
        let user = User::new(
            "test@example.com".to_string(),
            "test_password".to_string(),
            false,
        );

        let result = store.validate_user(user);
        assert_eq!(result.err(), Some(UserStoreError::UserNotFound));
    }

    #[test]
    fn validate_user_returns_invalid_credentials_err() {
        let mut store = HashmapUserStore::new();

        // Create a test user
        let user = User::new(
            "test@example.com".to_string(),
            "test_password".to_string(),
            false,
        );

        // Add the user to the store
        let _ = store.add_user(user);

        // Create a test user with incorrect password
        let user = User::new(
            "test@example.com".to_string(),
            "wrong_password".to_string(),
            false,
        );

        let result = store.validate_user(user);
        assert_eq!(result.err(), Some(UserStoreError::InvalidCredentials));
    }

    #[test]
    fn validate_user_returns_ok() {
        let mut store = HashmapUserStore::new();

        // Create a test user
        let user = User::new(
            "test@example.com".to_string(),
            "test_password".to_string(),
            false,
        );

        // Add the user to the store
        let _ = store.add_user(user.clone());

        let result = store.validate_user(user);
        assert!(result.is_ok());
    }


}
