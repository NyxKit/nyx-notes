# Authentication

## Design Principle

**Auth is pluggable.** The `AuthStore` trait in `notes-core` abstracts identity verification. The concrete implementation is selected at startup via `AUTH_MODE`.

This means the app works fully offline, air-gapped, and without any external dependencies when the right mode is chosen.

---

## Auth Modes

| Mode | Use case | External dependency |
|---|---|---|
| `local` | Single-user, local machine, native app | None |
| `secret_key` | Self-hosted NAS/server, small group | None |
| `oidc` | Any deployment with an identity provider | Any OIDC provider (Authentik, Keycloak, Authelia, Firebase, etc.) |

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

Frontend behaviour: a local workspace profile skips `LoginView` entirely. No token is attached to API requests for that profile. The backend ignores the `Authorization` header.

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
    encoding_key: jsonwebtoken::EncodingKey,
    decoding_key: jsonwebtoken::DecodingKey,
    users: Arc<RwLock<Vec<LocalUser>>>,
}

impl AuthStore for SecretKeyAuthStore {
    fn verify_token(&self, token: &str) -> Result<User, AuthError> {
        // decode JWT (HS256), verify signature + expiry, extract sub (user_id), look up user
    }

    fn login(&self, username: &str, password: &str) -> Result<LoginToken, AuthError> {
        // find user by username, verify Argon2 password hash, issue signed JWT
    }
}
```

**Dependencies:** `jsonwebtoken` for JWT encode/decode (HS256); `argon2` for password hashing. Both live in `notes-auth-local` only — not pulled into `notes-core` or `notes-server-axum`.

Frontend behaviour: remote server profiles in this feature use a simple username + password form. On success, the client stores the token in the active profile session and attaches it to that profile's API requests as `Authorization: Bearer <token>`.

---

## `oidc` — OpenID Connect

Verifies tokens issued by any OIDC-compliant identity provider. Suitable for any deployment: self-hosted (Authentik, Keycloak, Authelia), cloud-managed (Firebase Auth, Auth0), or any other OIDC provider.

**When to use:** NAS or server with an existing identity provider (Authentik, Keycloak, Authelia, etc.).

### Configuration

| Env var | Required | Description |
|---|---|---|
| `OIDC_ISSUER_URL` | Yes | OIDC discovery URL (e.g. `https://auth.example.com`) |
| `OIDC_CLIENT_ID` | Yes | Client ID registered with the provider |
| `OIDC_AUDIENCE` | No | Expected `aud` claim value; defaults to `OIDC_CLIENT_ID` |

The provider's JWKS endpoint is discovered from `{OIDC_ISSUER_URL}/.well-known/openid-configuration` and cached per `Cache-Control: max-age` headers.

```rust
pub struct OidcAuthStore {
    issuer: String,
    client_id: String,
    jwks_cache: Arc<RwLock<JwksCache>>,
}
```

Frontend behaviour: redirect to the OIDC provider's login page. On return, exchange the auth code for tokens. Attach the access token as `Bearer` on API requests.

---

## Multi-Profile Client Behaviour

The client may hold one local workspace profile and multiple remote server profiles at the same time.

- Each remote profile is scoped to one `server_url + username` pair
- The same server URL may appear more than once when each saved profile uses a different username
- Every remote profile keeps its own auth mode discovery result, credentials, and signed-in state
- Signing out of or failing authentication for one remote profile must not affect any other saved profile
- For this feature, remote profiles support username/password login only; remote `oidc` flows remain unsupported in the multi-profile client flow

`Set up a new server` remains a guided onboarding branch for server managers, but server provisioning itself is outside the client-auth contract.

---

## Auth Mode Discovery

The frontend needs to know which auth mode the selected server profile is running so it can show the correct login UI (or none at all).

```
GET /api/auth/mode
→ 200 OK  { "mode": "local" | "secret_key" | "oidc", "issuer"?: "...", "client_id"?: "...", "server_id"?: "...", "server_name"?: "...", "api_version"?: "..." }
```

This endpoint is unauthenticated. The frontend calls it when bootstrapping the active profile and before attempting remote login.

---

## Crate Structure

Auth implementations live in separate crates to avoid pulling in unnecessary dependencies:

```
crates/
  notes-auth-local/      # LocalAuthStore + SecretKeyAuthStore (no external deps)
  notes-auth-oidc/       # OidcAuthStore
```

`notes-server-axum` selects which crate to link based on the `AUTH_MODE` at runtime (all modes are compiled in; selection is a runtime branch, not a compile-time feature flag, for simplicity).

---

## Removed

- The `AUTH_DEV_MODE=true` environment variable hack is replaced by `AUTH_MODE=local`. It should not be used.
- The `firebase` auth mode has been removed. Users who want Google/email login or other managed identity should use `AUTH_MODE=oidc` with Firebase Auth as the OIDC provider (`OIDC_ISSUER_URL=https://securetoken.google.com/<project-id>`).
