# Authentication (Firebase)

## Purpose

Implements the `AuthStore` trait from `notes-core` using Firebase Authentication. The app never manages passwords or sessions itself — Firebase handles identity, and the backend only verifies Firebase ID tokens.

## Design Decision

**Firebase is used for auth; the filesystem is used for content.** This is a deliberate split:

- Firebase manages user identity, login providers (email, Google, etc.), and session tokens
- The Rust backend verifies Firebase ID tokens on every request
- No users table, no session management, no password hashing in this app

This means the backend is **stateless with respect to auth** — it trusts Firebase and uses the verified `uid` as `user_id` for storage namespacing.

## Token Verification Flow

```
Client                  Backend              Firebase
  |                        |                    |
  |-- Bearer <id_token> -->|                    |
  |                        |-- verify JWT ----->|
  |                        |<-- uid, email -----|
  |                        |                    |
  |                   (proceed with uid)        |
```

1. The frontend obtains a Firebase ID token via the Firebase JS SDK (`getIdToken()`)
2. The token is sent as `Authorization: Bearer <token>` on every API request
3. The backend verifies the JWT signature using Firebase's public keys (fetched from Google's JWKS endpoint and cached)
4. On success: extract `uid`, `email`, `name` from the token claims → `User`

## Implementation

```rust
pub struct FirebaseAuthStore {
    project_id: String,
    // cached public keys, refreshed when expired
    jwks_cache: Arc<RwLock<JwksCache>>,
}

impl AuthStore for FirebaseAuthStore {
    fn find_user(&self, user_id: &str) -> Result<Option<User>, AuthError> {
        // Not used in token-only flow; could call Firebase Admin REST API if needed
        todo!()
    }

    fn verify_token(&self, token: &str) -> Result<User, AuthError> {
        // 1. Decode JWT header to get `kid`
        // 2. Fetch/cache Google public keys for `kid`
        // 3. Verify signature, expiry, audience (== project_id), issuer
        // 4. Extract uid, email, display_name from claims
        // 5. Return User
        todo!()
    }
}
```

### JWKS Caching

Firebase public keys are fetched from Google's JWKS endpoint. The response includes `Cache-Control: max-age=<N>` headers. The implementation must cache keys and only re-fetch when expired.

No Firebase service account key is needed for token verification — only the `FIREBASE_PROJECT_ID` and the public JWKS are required.

## Configuration

| Env var | Required | Description |
|---|---|---|
| `FIREBASE_PROJECT_ID` | Yes | Firebase project ID (used to validate `aud` claim) |

## Single-User / Dev Mode

For local development without Firebase, a dev-mode bypass can be enabled:

- `AUTH_DEV_MODE=true` — skip token verification, use a hardcoded `user_id: "dev"`
- Should **never** be enabled in production

## Multi-User Notes Namespacing

Once a token is verified, the `User.id` (Firebase `uid`) is used as the directory name under `$NOTES_ROOT/users/`. Each user's personal vaults live at `$NOTES_ROOT/users/<uid>/`.

## Future Considerations

- Offline/local-only mode: allow using the app without Firebase (dev or air-gapped)
- Custom auth backend: `AuthStore` trait makes it swappable
