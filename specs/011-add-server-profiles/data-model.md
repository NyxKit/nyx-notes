# Data Model: Multi-Instance Access Profiles

## Overview

This feature introduces client-owned profile entities. They do not change note, vault, team, or comment persistence on disk.

## Entities

### Workspace Profile

Represents one selectable app context in the client.

| Field | Description | Validation |
|---|---|---|
| `profile_id` | Stable client-generated identifier | Required; immutable after creation |
| `profile_type` | `local` or `remote` | Required |
| `display_name` | User-facing label shown in switcher | Required; non-empty |
| `is_active` | Whether this is the current workspace | Exactly one profile may be active at a time |
| `last_used_at` | Last successful activation time | Optional; updated on activation |
| `last_route` | Last in-app route visited for this profile | Optional |

Relationships:
- One workspace profile may own zero or one remote connection profile.
- One workspace profile may own zero or one server account session.
- The local workspace profile is a singleton.

### Remote Connection Profile

Stores the connection details for one remote Nyx Notes server account.

| Field | Description | Validation |
|---|---|---|
| `profile_id` | Owning workspace profile ID | Required; matches a remote workspace profile |
| `server_url` | Base URL for the remote server | Required; normalized before persistence |
| `username` | Username used on that server | Required; non-empty |
| `password_ref` | Reference to stored secret/password entry | Required for remote profiles |
| `server_label` | Optional server-provided or user-facing label | Optional |
| `server_id` | Optional stable server identity returned by discovery | Optional; immutable once trusted unless explicitly refreshed |
| `api_version` | Optional server compatibility indicator | Optional |
| `connection_status` | Current connectivity state | One of `unknown`, `reachable`, `unreachable`, `invalid_server`, `auth_failed` |

Uniqueness rules:
- `local` profile cannot be duplicated.
- Remote profiles must be unique on normalized `server_url + username`.
- The same `server_url` may appear multiple times if `username` differs.

### Server Account Session

Represents the current signed-in state for one remote profile.

| Field | Description | Validation |
|---|---|---|
| `profile_id` | Owning workspace profile ID | Required |
| `auth_mode` | Auth mode discovered from the target server | Required after successful probe |
| `access_token` | Session token for the remote profile | Optional until sign-in succeeds; isolated per profile |
| `expires_at` | Token expiry if applicable | Optional |
| `session_state` | Current auth/session state | One of `signed_out`, `probing`, `signing_in`, `signed_in`, `expired`, `error` |
| `last_error` | Last connection or auth error summary | Optional; profile-scoped only |

Constraints:
- This feature supports remote username/password login only.
- Session records must never be reused across profiles.
- Signing out one profile must not alter any other profile's session.

### Installation Mode

Represents the initial onboarding choice.

| Field | Description | Validation |
|---|---|---|
| `mode` | `local` or `server` | Required at first-run setup |
| `server_onboarding_choice` | `setup_new_server` or `connect_existing_server` | Required when `mode = server` |

## Lifecycle

### Workspace Profile Lifecycle

1. Created during first-run setup or profile-add flow.
2. Validated if remote by probing discovery before activation.
3. Activated by switching the client into that profile.
4. Updated when label, credentials, or last route changes.
5. Removed by user action; if active, the client must choose a fallback profile or restart onboarding.

### Remote Session Lifecycle

1. `signed_out` after profile creation.
2. `probing` while checking discovery/compatibility.
3. `signing_in` during credential submission.
4. `signed_in` after successful login.
5. `error` or `expired` when the server becomes unreachable, rejects credentials, or invalidates the token.
6. Returns to `signed_out` on explicit sign-out.

## Derived Behaviors

- Activating a profile clears visible server-scoped data before loading the new profile state.
- Local profile activation bypasses login.
- Remote profile activation requires a successful discovery probe and valid username/password authentication.
- Broken remote profiles remain saved so the user can retry or edit them later.
