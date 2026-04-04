# Research: User Management

## Embedded Credential Store

Decision: Use a server-local embedded SQLite database for user accounts and credential records, replacing the current `.users.json` file approach for managed users.

Rationale: SQLite is small, fast, portable, and already well suited to single-process or lightly concurrent self-hosted deployments. It provides atomic updates, uniqueness constraints, and safer multi-step mutations than a flat JSON file. Those properties matter for duplicate-username prevention, protected admin deletion, and future user-management growth.

Alternatives considered: Keeping `.users.json` was rejected because file-level read/modify/write updates are brittle for concurrent admin actions and provide no built-in uniqueness or transaction support. `sled` and similar embedded KV stores were rejected because this feature is relational enough to benefit from explicit constraints and queryable rows, while SQLite has far broader operational familiarity.

## Password Storage Strategy

Decision: Store passwords only as Argon2 hashes with per-password salts; do not use reversible encryption for stored passwords.

Rationale: The feature requirement is secure local credential storage, and password verification only needs a non-reversible verifier. Argon2 is already used by the codebase and aligns with the current `secret_key` mode behavior. Reversible password storage would expand breach impact and add key-management risk without enabling any user value.

Alternatives considered: Reversible encryption was rejected because the backend never needs to recover original passwords. Simpler hashes were rejected because they are materially weaker against offline attacks.

## Backend Boundary For User Management

Decision: Keep all user and authentication operations behind backend-owned abstractions, with shared domain types in `notes-core`, persistence implementation in `notes-auth`, and admin authorization enforced in `notes-server-axum`.

Rationale: This preserves the constitution's layer model. The backend already owns authentication and request identity; extending that boundary to user management avoids exposing credential storage or business rules to the frontend. It also keeps future auth-mode swaps possible because the frontend talks only to backend contracts.

Alternatives considered: Letting the frontend talk directly to the embedded store was rejected because it violates the stated requirement and the architecture. Putting admin permission rules inside the auth-store implementation was rejected because authorization belongs in the API layer.

## Auth Abstraction Shape

Decision: Introduce explicit user-management capabilities at the shared domain layer rather than encoding admin CRUD directly into Axum handlers or filesystem/storage crates.

Rationale: The current `AuthStore` trait only supports identity lookup and login. User-management endpoints need list/create/update/delete operations and protected-account validation that should be mockable in server tests and reusable across auth-store implementations. Capturing that in `notes-core` keeps the server from depending on concrete store details.

Alternatives considered: Directly calling SQLite from `notes-server-axum` was rejected because it breaks layer boundaries and makes server tests harder to isolate. Overloading note-storage traits with auth concerns was rejected because content storage and credential storage are separate responsibilities.

## HTTP Contract Shape

Decision: Add an admin-only `/api/users` contract with list, create, update, and delete operations, while keeping authentication login and token verification on the existing auth routes.

Rationale: User management is a distinct administrative surface, not part of note or vault CRUD. A dedicated route family keeps responsibilities clear and lets the frontend model one users domain around a narrow contract.

Alternatives considered: Reusing `/api/auth/*` for admin CRUD was rejected because it mixes sign-in concerns with server administration. Nesting users under vault or server-vault routes was rejected because accounts are server-scoped.

## Frontend Integration Pattern

Decision: Add a top-level `frontend/src/users/` domain containing API module, composable/store, `UsersView`, and `CreateEditUser` modal component, then wire a `Users` entry into the authenticated sidebar directly above `Settings`.

Rationale: The existing frontend already follows domain-based structure with a persistent shell and route-driven pages. A dedicated users domain keeps the feature isolated, matches the stated requirement, and fits the constitution's barrel-export rules. The sidebar placement makes administration discoverable without overloading the settings page.

Alternatives considered: Extending the current settings view with an embedded users panel was rejected because the requested UX is a separate navigation destination. Scattering users logic across `auth/` and `shared/` was rejected because the feature is an administrative domain rather than sign-in state.

## Testing Impact

Decision: Add auth-store integration tests for persistence and protected admin rules, Axum integration tests for `/api/users`, frontend unit tests for users composables/views, and Playwright coverage for sidebar navigation plus create/edit/delete flows.

Rationale: The highest risks are broken permission enforcement, duplicate handling, and state drift between UI and backend results. Coverage needs to prove both contract correctness and the protected-account edge cases.

Alternatives considered: Relying only on server unit tests was rejected because the feature spans backend contracts and frontend administration flows. Skipping store-level auth tests was rejected because the embedded database introduces new persistence behavior that JSON-file tests did not cover.
