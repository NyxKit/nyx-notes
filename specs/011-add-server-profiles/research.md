# Research: Multi-Instance Access Profiles

## Profile Persistence Boundaries

Decision: Persist workspace profiles entirely in client app configuration, with one reserved local profile and many remote profiles; keep connection metadata separate from per-profile auth secrets and session state.

Rationale: The existing architecture treats the filesystem and server API as note-delivery concerns, not profile-management concerns. Multi-profile switching is a client capability layered above the current API client, so the safest boundary is client-side persistence with isolated secret storage per remote profile.

Alternatives considered: Storing tokens and remote cache state in one flat shared blob was rejected because it increases cross-profile leakage risk. Treating the local profile as just another remote `localhost` profile was rejected because local mode has different trust and auth assumptions.

## Profile Identity And Uniqueness

Decision: Give every saved profile a stable client-generated `profile_id`; enforce remote profile uniqueness on normalized `server_url + username`; keep the local profile as a singleton.

Rationale: One server can host multiple users, so URL-only uniqueness is too strict. Display names are user-editable and cannot be identity keys. A stable internal ID prevents renaming from breaking references, while normalized URL plus username avoids duplicate entries for the same remote account.

Alternatives considered: Display-name uniqueness was rejected because labels are mutable. URL-only uniqueness was rejected because it blocks multiple accounts on the same server. Allowing duplicate URL+username pairs was rejected because the feature does not require parallel sessions for the same account.

## Session Isolation And Switching

Decision: Scope auth mode, session token, cached server data, and last-opened route to the active profile, and fully re-bootstrap client state on every profile switch.

Rationale: Each profile may target a different server, user identity, and auth mode. The existing frontend assumes one startup auth probe and one active auth context, so safe switching requires clearing in-memory state, aborting in-flight requests, probing the selected profile, and then hydrating only that profile's data.

Alternatives considered: Reusing a global auth store and only swapping the base URL was rejected because it risks leaking tokens and stale notes across profiles. Keeping multiple profile stores mounted in parallel was rejected as unnecessary complexity for the current architecture.

## Onboarding Scope Boundaries

Decision: Keep `Set up a new server` and `Connect to an existing server` as separate flows; this feature implements the connection flow and reserves provisioning as guided setup/out-of-scope work.

Rationale: Provisioning a server includes deployment, auth-mode configuration, storage location, and administrator setup, which are not normal authenticated API interactions. Connecting to an existing server is a client concern: probe the server, validate compatibility, and authenticate with username/password.

Alternatives considered: A single combined wizard for both provisioning and connecting was rejected because it mixes administrator tasks with end-user sign-in and creates ambiguous failure handling.

## Minimal Contract Additions

Decision: Keep `GET /api/auth/mode` as the unauthenticated discovery probe and `POST /api/auth/login` as the username/password sign-in endpoint. If server-side additions are needed, add only optional discovery metadata such as `server_id`, `server_name`, or `api_version`.

Rationale: The smallest stable interface for a multi-server client is one discovery call and the existing login call. Optional discovery metadata improves compatibility checks and UI labeling without turning client profile management into a server feature.

Alternatives considered: Adding new profile-management endpoints on the server was rejected because profiles are client-owned. Putting compatibility metadata only on the login endpoint was rejected because the client needs it before login.

## Failure Classification

Decision: Validate remote profiles by probing discovery first and classify failures into unreachable server, invalid server response, unsupported auth mode, and authentication failure.

Rationale: Users need actionable states and recovery paths. Discovery-first validation avoids mislabeling transport and compatibility errors as bad credentials and allows broken profiles to remain saved for later recovery.

Alternatives considered: Treating all failures as credential failures was rejected because it hides the real cause. Automatically deleting broken profiles was rejected because temporary outages are common.

## Test Strategy Impact

Decision: Add explicit frontend and server integration coverage for discovery, per-profile login gating, failure classification, and switch isolation.

Rationale: The main risks are boundary failures and state leakage, not domain logic. Tests need to prove that switching profiles clears stale UI state, attaches only the correct profile token, and preserves other saved profiles during isolated failures.

Alternatives considered: Relying mostly on happy-path unit tests was rejected because this feature's failure modes appear at API, routing, and session boundaries.
