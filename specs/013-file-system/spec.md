# Feature Spec: File System Architecture Alignment

## Summary

Align the full Nyx Notes specification and implementation with `docs/architecture/file-system.md`.

This feature replaces the current `users/<uid>/...` and `teams/<team-id>/...` storage model with the new server-first layout built around:

- `<server-slug>/homes/<home-slug>/...` for personal vaults
- `<server-slug>/vaults/<vault-slug>/...` for shared server vaults
- `local/...` for local-only and future synced local vaults

`docs/architecture/file-system.md` is the source of truth for this change.

## Clarifications

### Session 2026-04-03

- Q: Should `local/` be implemented as an active runtime namespace in this feature? → A: Reserved in the filesystem contract, but runtime support is deferred for this feature.
- Q: How is the MVP server slug determined? → A: It is derived from the server name at startup.
- Q: What is the canonical MVP API identifier for servers, homes, vaults, and notes? → A: Slugs are the canonical API identifiers in MVP; stable internal IDs are still generated and maintained in metadata for future use.

## Goals

- Update all affected docs so they consistently describe the new filesystem structure
- Update the Rust backend, CLI, auth, and frontend contracts to match the new structure
- Remove the current MVP dependency on team ownership and team routes
- Preserve metadata files and stable internal IDs while using immutable slugs for filesystem paths

## MVP Scope

- Single-server setup
- The server slug is derived from the server name at startup
- Two server roles: `admin` and `user`
- Personal vaults live under one user home per server
- Shared vaults live under the server namespace
- `admin` can create and delete server vaults
- `user` can view and edit notes in existing server vaults, but cannot create or delete server vaults
- Slug renames are not supported

## Out Of Scope

- Multi-server runtime behavior
- Team-based permissions and team management
- Username changes
- Local sync behavior beyond reserving the filesystem structure
- Runtime support for the `local/` namespace
- Migration of existing test data

## Functional Requirements

### FR-1 Filesystem Layout

The active filesystem layout must match `docs/architecture/file-system.md`, including:

- `<server-slug>/.server.json`
- `<server-slug>/homes/<home-slug>/.home.json`
- `<server-slug>/homes/<home-slug>/<vault-slug>/.vault.json`
- `<server-slug>/vaults/<vault-slug>/.vault.json`
- `local/.local.json`

Each vault directory stores note markdown files and optional `.comments.json` sidecars using stable note IDs as file names.

`local/` remains part of the on-disk contract, but this feature does not require active runtime support for local-namespace vault operations.

### FR-2 Slug Rules

Filesystem-facing slugs must:

- be immutable in the MVP
- match `^[a-z0-9_-]{1,64}$`
- be lowercase ASCII only
- reject spaces, path separators, `.` and `..`
- be unique within their documented parent scope

When a server namespace is first created in the MVP, its slug is derived from the configured server name at startup and must still satisfy the same slug validation rules.

### FR-3 Stable IDs

In the MVP, server and home namespaces remain slug-based. Vaults use slugs as their canonical path and API identifiers. Notes use stable IDs as their canonical path and API identifiers.

### FR-4 Domain Model

The domain model must stop treating teams as the MVP sharing primitive.

- Team-specific domain types and ownership assumptions must be removed or retired from MVP-facing docs and code paths
- Vault ownership and metadata must represent personal-home-backed vaults and server-owned vaults

### FR-5 Authorization Model

The MVP authorization model must support:

- `admin`
- `user`

Server roles govern server-vault administration. Personal home vaults remain owner-scoped.

### FR-6 API And CLI Alignment

All API, CLI, and frontend docs and contracts must be updated to remove team-centric addressing, to describe the new home/server-vault model, to use vault slugs as canonical vault identifiers, and to use stable note IDs as canonical note identifiers.

### FR-7 Code Alignment

The following implementation areas must be updated to match the new docs:

- `notes-core`
- `notes-storage-fs`
- `notes-server-axum`
- `notes-auth-local`
- `notes-cli`
- frontend shared types and any routes or stores that currently assume teams

## Acceptance Criteria

- All architecture docs reference the new storage structure consistently
- `README.md` and `.env.example` reflect the new MVP terminology and configuration
- Team routes and team-only MVP behavior are removed or clearly retired from the active spec
- The storage implementation resolves vaults using the new server/home/server-vault layout
- CLI and server no longer depend on `users/<NOTES_USER_ID>/` as the filesystem contract
- `local/` remains documented in the filesystem contract without requiring active runtime support in this feature
- The derived server slug is documented and validated against the slug rules
- Vault slugs are documented as canonical vault identifiers while note IDs remain stable canonical note identifiers
- Relevant tests are updated for the new layout and role model
