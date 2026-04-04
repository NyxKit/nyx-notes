use notes_core::{AuthError, AuthStore, ServerRole, User};

/// `AuthStore` implementation for `AUTH_MODE=local`.
///
/// No verification — every request is treated as the configured local user.
/// The `Authorization` header is ignored entirely by the `AuthenticatedUser` extractor
/// when this mode is active.
pub struct LocalAuthStore {
    user: User,
}

impl LocalAuthStore {
    /// `user_id` is taken from `LOCAL_USER_ID` (defaults to `"local"`).
    /// `name` is taken from `NOTES_LOCAL_USER_NAME` (defaults to `"Local User"`).
    pub fn new(user_id: String, name: String) -> Self {
        Self {
            user: User {
                id: user_id.clone(),
                username: user_id.clone(),
                email: format!("{user_id}@localhost"),
                display_name: name,
                role: ServerRole::Admin,
            },
        }
    }
}

impl AuthStore for LocalAuthStore {
    fn find_user(&self, user_id: &str) -> Result<Option<User>, AuthError> {
        if user_id == self.user.id {
            Ok(Some(self.user.clone()))
        } else {
            Ok(None)
        }
    }

    fn verify_token(&self, _token: &str) -> Result<User, AuthError> {
        Ok(self.user.clone())
    }
}
