# Data Model: User Management

## Overview

This feature introduces server-scoped user-management entities for authentication and administration in `secret_key` mode. It does not change note, vault, or comment filesystem storage.

## Entities

### User Account

Represents one managed identity on a Nyx Notes server.

| Field | Description | Validation |
|---|---|---|
| `id` | Stable internal user identifier | Required; immutable after creation |
| `username` | Sign-in name used for authentication | Required; normalized; unique server-wide |
| `display_name` | User-facing name shown in admin UI and authored metadata | Required; non-empty |
| `email` | Required account email address | Required; non-empty; unique server-wide |
| `role` | Server-level role | Required; one of `admin` or `user` |
| `status` | Administrative lifecycle state | Required; one of `active` or `deleted` for this feature scope |
| `created_at` | Creation timestamp | Required; set once |
| `updated_at` | Last account metadata update timestamp | Required; updated on changes |
| `last_password_change_at` | Timestamp of most recent password update | Optional until first password set |

Relationships:
- One user account owns exactly one active password credential record in this feature.
- One user account may be protected from deletion depending on bootstrap/admin rules.

### Password Credential

Represents the stored verifier used to authenticate a user.

| Field | Description | Validation |
|---|---|---|
| `user_id` | Owning account ID | Required; references an existing user account |
| `password_hash` | Non-reversible password verifier | Required; never exposed in API responses |
| `algorithm` | Hashing algorithm identifier | Required; must identify the active verifier scheme |
| `created_at` | Time the credential was first set | Required |
| `updated_at` | Time the credential was last rotated | Required |

Constraints:
- Plaintext passwords are accepted only during create/update requests and are never persisted.
- Credential records are backend-managed only and are not returned to clients.

### User Mutation Request

Represents an administrative create or update submission.

| Field | Description | Validation |
|---|---|---|
| `username` | Proposed sign-in name | Required on create; unique after normalization |
| `display_name` | Proposed visible name | Required; non-empty |
| `email` | Proposed account email address | Required; non-empty; unique after normalization |
| `role` | Proposed server role | Required |
| `password` | Proposed new password | Required on create; optional on edit unless being rotated |

Validation rules:
- Duplicate usernames must be rejected.
- Duplicate email addresses must be rejected.
- Blank required fields must be rejected.
- Password must be at least 12 characters and include at least 3 of these 4 categories: lowercase letters, uppercase letters, digits, and symbols.
- Editing without a replacement password must preserve the existing credential.

### Protected Account Rule

Represents deletion and downgrade safeguards around administrative access.

| Rule | Meaning |
|---|---|
| `last_admin_cannot_be_removed` | The final remaining administrator account cannot be deleted |
| `last_admin_cannot_be_demoted` | The final remaining administrator account cannot be changed to a non-admin role |
| `self_admin_cannot_be_removed` | The currently signed-in administrator cannot delete their own account |
| `self_admin_cannot_be_demoted` | The currently signed-in administrator cannot remove their own administrator role |
| `bootstrap_account_may_be_protected` | Initial bootstrap access may be blocked from deletion until another admin exists |

## Lifecycle

### User Account Lifecycle

1. Bootstrap admin exists or is created during auth-store initialization.
2. Administrator creates a new user with metadata and initial password.
3. User becomes available for future login immediately after persistence succeeds.
4. Administrator may update metadata, role, and optionally rotate password.
5. A signed-in administrator may edit their own metadata, but may not delete their own account or remove their own administrator role.
6. Administrator may delete another account only if protected-account rules still pass.

### Password Credential Lifecycle

1. Created together with a new account.
2. Verified during login.
3. Replaced on password rotation.
4. Removed when the owning user account is deleted.

## Derived Behaviors

- The users list exposes account metadata and allowed actions, never credential details.
- Create and edit flows share one modal surface but differ in required password behavior.
- Protected-account checks run before delete or role downgrade is committed.
- Self-management checks run before self-delete or self-demotion is committed.
- Successful create, update, and delete operations must be reflected in subsequent list responses without requiring a server restart.
