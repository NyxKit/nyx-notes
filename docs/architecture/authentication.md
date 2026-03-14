# Authentication

## Design Principle

**Auth is pluggable.** The `AuthStore` trait in `notes-core` abstracts identity verification. The concrete implementation is selected at startup via `AUTH_MODE`. Firebase is one option — not a requirement.

This means the app works fully offline, air-gapped, and without any Google account when the right mode is chosen.

---

## Auth Modes

| Mode | Use case | External dependency |
|---|---|---|
| `local` | Single-user, local machine, native app | None |
| `secret_key` | Self-hosted NAS/server, small group | None |
| `firebase` | Cloud deployment, managed multi-user | Google Firebase |
| `oidc` | Self-hosted with an identity server | Authentik / Keycloak / Authelia / any OIDC provider |

Set via the `AUTH_MODE` environment variable. Default: `local`.

---

## `local` — No Authentication

No verification. The server always returns a hardcoded single user. There is no login screen.

**When to use:** Local machine install, native Tauri app, development, air-gapped environments.

```rust
pub struct LocalAuthStore {
    user: User, // constructed from NOTES_LOCAL_USER_NAME env var, defaults to "local"
}

impl AuthStore for LocalAuthStore {
    fn verify_token(&self, _token: &str) -> Result<User, AuthError> {
        Ok(self.user.clone())
    }
}
```

Frontend behaviour: skip `LoginView` entirely. No token is attached to API requests. The backend ignores the `Authorization` header.

---

## `secret_key` — Local JWT Signing

The server generates (or loads) a secret key on startup and signs its own JWTs. No external service. Works offline and air-gapped.

**When to use:** Self-hosted NAS, single-user or small group, no internet dependency.

### Key management

On first run, a random 256-bit key is generated and saved to `$NOTES_SECRET_KEY_PATH` (default: `~/.config/nyx-notes/secret.key`). On subsequent runs it is loaded from that file.

Alternatively, the key can be provided directly via `NOTES_SECRET_KEY` env var (useful in containers).

### Login flow

```
POST /api/auth/login
Body: { "username": "...", "password": "..." }
→ 200 OK  { "token": "<signed JWT>", "expires_in": 86400 }
→ 401     { "error": "invalid credentials" }
```

Users are stored as a simple list in `$NOTES_ROOT/.users.json`. Passwords are hashed with Argon2. The first user is created on first run if none exist (interactive or via env vars).

The returned JWT is short-lived (24h by default, configurable). The frontend stores it in memory and refreshes it before expiry.

```rust
pub struct SecretKeyAuthStore {
    key: Hmac<Sha256>,
    users: Arc<RwLock<Vec<LocalUser>>>,
}

impl AuthStore for SecretKeyAuthStore {
    fn verify_token(&self, token: &str) -> Result<User, AuthError> {
        // verify HMAC signature + expiry, extract user_id, look up user
    }
}
```

Frontend behaviour: show a simple login form (username + password). On success, store the token and attach it to all API requests as `Authorization: Bearer <token>`.

---

## `firebase` — Firebase Authentication

Firebase manages user identity and issues ID tokens (JWTs). The backend verifies token signatures using Firebase's public JWKS endpoint.

**When to use:** Cloud deployments, managed multi-user, when you want Google/email login providers without running your own identity server.

### Token verification flow

```
Client                  Backend              Firebase
  |                        |                    |
  |-- Bearer <id_token> -->|                    |
  |                        |-- verify JWT ----->|
  |                        |<-- uid, email -----|
  |                        |                    |
  |                   (proceed with uid)        |
```

Firebase's public keys are fetched from Google's JWKS endpoint and cached per `Cache-Control: max-age` headers.

### Configuration

| Env var | Required | Description |
|---|---|---|
| `FIREBASE_PROJECT_ID` | Yes | Used to validate the `aud` claim |

No service account key is needed for token verification.

```rust
pub struct FirebaseAuthStore {
    project_id: String,
    jwks_cache: Arc<RwLock<JwksCache>>,
}
```

Frontend behaviour: Firebase JS SDK handles login UI and token lifecycle (`getIdToken()` for refresh).

---

## `oidc` — OpenID Connect

Verifies tokens issued by any OIDC-compliant identity provider. Drop-in replacement for Firebase for users who self-host their identity layer.

**When to use:** NAS or server with an existing identity provider (Authentik, Keycloak, Authelia, etc.).

### Configuration

| Env var | Required | Description |
|---|---|---|
| `OIDC_ISSUER_URL` | Yes | OIDC discovery URL (e.g. `https://auth.example.com`) |
| `OIDC_CLIENT_ID` | Yes | Client ID registered with the provider |
| `OIDC_AUDIENCE` | No | Expected `aud` claim value; defaults to `OIDC_CLIENT_ID` |

The provider's JWKS endpoint is discovered from `{OIDC_ISSUER_URL}/.well-known/openid-configuration` and cached the same way as Firebase.

```rust
pub struct OidcAuthStore {
    issuer: String,
    client_id: String,
    jwks_cache: Arc<RwLock<JwksCache>>,
}
```

Frontend behaviour: redirect to the OIDC provider's login page. On return, exchange the auth code for tokens. Attach the access token as `Bearer` on API requests.

---

## Auth Mode Discovery

The frontend needs to know which auth mode the server is running so it can show the correct login UI (or none at all).

```
GET /api/auth/mode
→ 200 OK  { "mode": "local" | "secret_key" | "firebase" | "oidc", "oidc_issuer"?: "..." }
```

This endpoint is unauthenticated. The frontend calls it on startup before rendering anything.

---

## Crate Structure

Auth implementations live in separate crates to avoid pulling in unnecessary dependencies:

```
crates/
  notes-auth-local/      # LocalAuthStore + SecretKeyAuthStore (no external deps)
  notes-auth-firebase/   # FirebaseAuthStore
  notes-auth-oidc/       # OidcAuthStore
```

`notes-server-axum` selects which crate to link based on the `AUTH_MODE` at runtime (all modes are compiled in; selection is a runtime branch, not a compile-time feature flag, for simplicity).

---

## Removed

The `AUTH_DEV_MODE=true` environment variable hack is replaced by `AUTH_MODE=local`. It should not be used.
