# Research: Centralized Subscription Data Flow

## Decision 1: Use a single app-level subscription manager keyed by canonical query identities

- **Decision**: Introduce one shared subscription manager in the frontend that owns all active live queries and deduplicates equivalent subscriptions by canonical query key.
- **Rationale**: This is the cleanest way to avoid redundant subscription work when multiple composables request the same vault or note scope. It also gives one place to own ref-counting, stale-update protection, reconnect behavior, and latest-snapshot replay.
- **Alternatives considered**:
  - Per-component subscriptions: simpler initially but duplicates work and leaks lifecycle logic everywhere.
  - One manager per store: reduces some duplication but fragments dedupe and reconnect behavior.

## Decision 2: Keep Pinia stores as the only frontend read surface

- **Decision**: `VaultBase` and the subscription manager write into domain stores; components and views continue to read only from Pinia-backed state.
- **Rationale**: The repo already centers state in domain stores. Preserving stores as the single read surface avoids split-brain state between transport-level refs and domain state.
- **Alternatives considered**:
  - Returning live payload refs directly from composables: faster to prototype, but creates competing data sources.
  - Letting components subscribe directly: violates the repo’s frontend constraints and spreads lifecycle complexity.

## Decision 3: Use typed query scopes and deterministic query-key normalization

- **Decision**: Represent subscription requests as typed collection/document scopes and derive stable query keys from normalized business identifiers such as collection, server, user/home, vault, and note.
- **Rationale**: Deduplication is only safe if logically identical queries produce the same identity. Typed scopes also make backend authorization much safer than a free-form query language.
- **Alternatives considered**:
  - Free-form query strings or DSL: flexible, but harder to authorize and normalize.
  - Object identity or raw JSON stringification: brittle across callers and parameter ordering.

## Decision 4: Use SSE as the primary backend delivery mechanism

- **Decision**: Expose live subscription scopes through server-sent events as the first live-delivery transport.
- **Rationale**: This codebase is currently REST-oriented and needs server-to-client updates more than bidirectional messaging. SSE fits that shape well, carries existing auth headers cleanly, and is operationally simpler than WebSockets for the initial rollout.
- **Alternatives considered**:
  - WebSockets: more flexible for dynamic multi-scope sessions, but adds protocol and lifecycle complexity too early.
  - Polling only: simplest operationally, but reintroduces redundant requests and slower live updates.

## Decision 5: Send snapshot-first, replace-style events

- **Decision**: Each live scope delivers an initial snapshot followed by full replacement updates, rather than fine-grained patch streams in the first rollout.
- **Rationale**: The data source is filesystem-backed and current route handlers already materialize coherent snapshots. Snapshot replacement is simpler to reason about than item-level merge logic and reduces edge-case complexity for the first live layer.
- **Alternatives considered**:
  - Fine-grained per-entity patches: more efficient later, but much more error-prone.
  - Invalidation-only events followed by REST refetch: easier backend design, but defeats the goal of centralizing and reducing redundant calls.

## Decision 6: Use ref-counting and generation fencing to prevent stale writes

- **Decision**: Shared subscriptions stay active while at least one consumer is attached, and every consumer/query transition increments a generation token so late events from released scopes are ignored.
- **Rationale**: Unsubscribe timing alone is not enough to prevent stale async events from mutating newer state after a route or query change.
- **Alternatives considered**:
  - Unsubscribe-only cleanup: simpler, but race-prone.
  - Timestamp-only freshness checks: weaker than an explicit generation fence tied to lifecycle changes.

## Decision 7: Publish backend changes into a broker from write routes

- **Decision**: Backend write handlers publish affected scope updates into a broker, which fans out refreshed snapshots to all listeners for that scope.
- **Rationale**: The server already knows when a note or vault changes during route handling. Reusing that knowledge avoids repeated rescans for every active listener.
- **Alternatives considered**:
  - Filesystem watcher only: useful later for out-of-band edits, but not enough by itself.
  - Per-client background rescans: simplest to wire, worst for redundant work.

## Decision 8: Keep authorization in the API layer for long-lived subscriptions

- **Decision**: Authorization for live scopes happens before a listener attaches to a subscription stream, and the API layer remains responsible for scope validation.
- **Rationale**: This preserves the repo’s existing layer boundary: `notes-server-axum` enforces permissions, while storage remains permission-agnostic.
- **Alternatives considered**:
  - Moving auth decisions into storage: violates the constitution.
  - Trusting subscription scopes after initial attachment forever: too risky for revocation and scope changes.
