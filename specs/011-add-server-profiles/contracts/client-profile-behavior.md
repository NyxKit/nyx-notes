# Contract: Client Profile Behavior

## Purpose

Defines the user-visible behavior contract for adding, activating, editing, and removing workspace profiles.

## Profile Types

| Type | Description |
|---|---|
| `local` | Local workspace with no login requirement |
| `remote` | Remote server profile with URL, username, password, and isolated session |

## Add Profile Flow

### Local Profile

- Created during first-run setup when the user selects `Local`
- Exactly one local profile may exist
- Activating it must bypass login and open the local workspace immediately

### Remote Profile

- Created during onboarding or later from profile management
- Requires `server_url`, `username`, and `password`
- Must probe discovery before login
- Must save as a separate profile when the same `server_url` is reused with a different `username`
- Must reject duplicates for the same normalized `server_url + username`

## Switching Behavior

- Switching profiles must clear visible notes, vaults, comments, and account context from the prior profile before loading the selected profile
- Switching to a local profile must not show a login form
- Switching to a remote profile must re-bootstrap the auth flow for that profile only
- In-flight requests from the previous profile must not update the new active profile view

## Failure Behavior

| Condition | Required Behavior |
|---|---|
| Invalid URL or unreachable server | Show profile-scoped connection error and allow retry or edit |
| Unsupported remote auth mode | Explain incompatibility and prevent activation |
| Invalid credentials | Keep profile saved, show auth failure for that profile only |
| Remote profile sign-out | Remove only that profile's active session |

## Removal Behavior

- Removing a non-active profile deletes only that saved profile and its isolated session data
- Removing the active profile must transition the app to another available profile or restart setup if none remain
- Removing one profile must never remove or alter another profile's saved credentials or session state
