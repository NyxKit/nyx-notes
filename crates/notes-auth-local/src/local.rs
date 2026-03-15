use notes_core::{AuthError, AuthStore, User};

/// `AuthStore` implementation for `AUTH_MODE=local`.
///
/// No verification — every request is treated as the configured local user.
/// The `Authorization` header is ignored entirely by the `AuthenticatedUser` extractor
/// when this mode is active.
pub struct LocalAuthStore {
    user: User,
}

impl LocalAuthStore {
    /// `name` is taken from `NOTES_LOCAL_USER_NAME`, defaulting to `"Local User"`.
    pub fn new(name: String) -> Self {
        Self {
            user: User {
                id: "local".into(),
                email: "local@localhost".into(),
                display_name: name,
            },
        }
    }
}

impl AuthStore for LocalAuthStore {
    fn find_user(&self, user_id: &str) -> Result<Option<User>, AuthError> {
        if user_id == "local" {
            Ok(Some(self.user.clone()))
        } else {
            Ok(None)
        }
    }

    fn verify_token(&self, _token: &str) -> Result<User, AuthError> {
        Ok(self.user.clone())
    }
}
