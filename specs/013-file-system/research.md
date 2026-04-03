# Research: File System Architecture Alignment

## Decision 1: Use slugs as canonical MVP API identifiers while keeping stable IDs in metadata

- Decision: Slugs are the canonical MVP API identifiers and immutable human-readable path identifiers. Metadata files still keep stable internal IDs for cross-reference identity, future sync identity, and possible later API evolution.
- Rationale: This preserves readable paths and cheap filesystem lookup on the hot path while retaining an escape hatch for future identity decoupling.
- Alternatives considered:
  - Stable IDs as canonical API identifiers: rejected for MVP because every lookup would need an ID-to-path resolution step or an index despite immutable slugs.
  - Slugs as the only identifiers: rejected because future sync, cross-references, and any later rename support become fragile.
  - Opaque IDs as directory names: rejected because it undermines the human-readable filesystem goal.

## Decision 2: Replace team ownership with server/home/server-vault ownership for the MVP

- Decision: The active MVP model becomes:
  - one server namespace
  - one personal home per user on that server
  - server-owned shared vaults
  - optional local namespace reserved for future work
- Rationale: This matches `docs/architecture/file-system.md`, removes the current team-heavy MVP burden, and keeps a clean path to future server expansion and sync.
- Alternatives considered:
  - Keep teams and add server vaults beside them: rejected because it preserves duplicate sharing concepts in the MVP.
  - Make everything personal-only for MVP: rejected because shared server vaults are already part of the desired direction.

## Decision 3: Keep `NOTES_USER_ID` as the MVP identity and home lookup key

- Decision: For MVP, the authenticated/logged-in user ID and the resolved home slug stay aligned through `NOTES_USER_ID` rather than introducing a second required environment variable.
- Rationale: This minimizes configuration churn across CLI, local auth, server startup, and docs while still allowing `.home.json` to carry a stable internal home ID and owner reference.
- Alternatives considered:
  - Introduce `NOTES_HOME_SLUG`: rejected for MVP because it increases config surface before the multi-home or rename story exists.
  - Resolve home solely through `.home.json` scans: rejected because it would add lookup complexity where a direct MVP mapping is acceptable.

## Decision 4: Store server role assignment in the auth/user model, not only in filesystem metadata

- Decision: User role assignment for the MVP (`admin` or `user`) is an authentication concern; `.server.json` advertises server identity and supported roles, while the active user's effective role comes from auth/user records.
- Rationale: Authorization decisions happen in the API layer and already depend on the authenticated user. Storing role membership with user records avoids turning `FsStorage` into an auth lookup layer.
- Alternatives considered:
  - Store per-user role assignments in `.server.json`: rejected because it couples auth membership to storage scans.
  - Hardcode one role in server code: rejected because it does not support both `admin` and `user`.

## Decision 5: Replace team routes with server-vault administration routes

- Decision: Remove `/api/teams/*` from the MVP contract and replace team-vault administration with server-vault administration endpoints.
- Rationale: The filesystem and ownership model no longer use teams in MVP, so the HTTP contract should not pretend they exist.
- Alternatives considered:
  - Keep `/api/teams/*` as thin aliases: rejected because it would encode a false public model and increase maintenance burden.
  - Make all vault routes flat with no dedicated server admin routes: rejected because server-vault creation/deletion privileges differ from personal-vault privileges.

## Decision 6: Keep note files and sidecars unchanged while changing namespace layout

- Decision: The `.md` frontmatter format and `<note-slug>.comments.json` sidecars remain intact unless a separate feature changes them.
- Rationale: The current request is structural. Keeping note payload formats stable limits the blast radius and isolates the storage-namespace refactor.
- Alternatives considered:
  - Redesign note frontmatter now: rejected as unnecessary scope expansion.
  - Remove sidecars now: rejected because comments remain part of the product contract.

## Decision 7: Use explicit namespace metadata files

- Decision: `.server.json`, `.home.json`, `.vault.json`, and `.local.json` are first-class storage artifacts in the new model.
- Rationale: They provide stable IDs, display data, ownership information, and future extension points without losing human-readable paths.
- Alternatives considered:
  - Infer everything from paths alone: rejected because ownership, sync metadata, and future server capabilities would become brittle.
  - Central index file for all namespaces: rejected because it fights the filesystem-native model.

## Decision 8: Treat team removal as an MVP contract change, not a hidden implementation detail

- Decision: Architecture docs, interface docs, frontend route docs, CLI docs, and code contracts all remove or retire team-specific MVP behavior in the same feature.
- Rationale: Teams are currently a first-class concept across docs and code. Leaving them half-present would create inconsistent source-of-truth behavior.
- Alternatives considered:
  - Only change storage docs and code: rejected because the rest of the spec would remain inaccurate.
  - Keep teams documented as future but active endpoints: rejected because it would keep dead or misleading contracts alive.
