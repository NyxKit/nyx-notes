# Data Model: Centralized Subscription Data Flow

## 1. VaultBase Query

- **Purpose**: Describes one requested live data scope.
- **Fields**:
  - `collection`: logical dataset being observed, such as vaults, notes, note-document, or comments
  - `scope_kind`: whether the query targets a collection result or a single document result
  - `server_slug`: active server namespace for the query
  - `owner_context`: personal-home, shared-server, or other allowed ownership context
  - `user_context`: current user/home identity when required for personal scopes
  - `vault_id`: vault identifier when the scope is vault-bound or note-bound
  - `note_id`: note identifier when the scope is note-bound
  - `filters`: normalized optional scope refinements that affect returned data membership
- **Validation rules**:
  - Collection queries must include all required business identifiers for that collection type.
  - Document queries must include the target document identifier.
  - Equivalent queries must normalize to the same canonical identity.

## 2. Canonical Query Key

- **Purpose**: Stable identity used for deduplication and shared subscription ownership.
- **Fields**:
  - `resource_name`
  - `scope_kind`
  - `normalized_identifiers`
  - `normalized_filters`
- **Validation rules**:
  - All logically identical queries must produce the same key.
  - Two queries with different scope semantics must never share the same key.

## 3. Shared Subscription

- **Purpose**: Represents the single active underlying observation for one canonical query key.
- **Fields**:
  - `query_key`
  - `status`: loading, active, reconnecting, failed, released
  - `ref_count`
  - `generation`
  - `latest_snapshot`
  - `last_updated_at`
  - `last_error`
- **Relationships**:
  - One shared subscription can serve many consumer handles.
- **State transitions**:
  - `loading -> active` when the initial snapshot is delivered
  - `active -> reconnecting` on transient stream interruption
  - `reconnecting -> active` on successful resume
  - `active/reconnecting -> failed` on terminal subscription failure
  - `active/failed -> released` when `ref_count` reaches zero and cleanup completes

## 4. Subscription Handle

- **Purpose**: Represents one consumer’s active claim on a shared subscription.
- **Fields**:
  - `handle_id`
  - `query_key`
  - `consumer_scope`
  - `generation_at_attach`
  - `attached_at`
  - `released_at`
- **Validation rules**:
  - A released handle cannot continue receiving updates.
  - A handle bound to an older generation cannot mutate store state after the generation changes.

## 5. Subscription Manager Record

- **Purpose**: Internal record linking a canonical query key to its shared subscription and active handles.
- **Fields**:
  - `query_key`
  - `shared_subscription`
  - `consumer_handles`
  - `release_deadline` (optional short grace window before full teardown)
- **Relationships**:
  - Owns exactly one shared subscription per key.
  - Owns zero or more active handles.

## 6. Observed Snapshot

- **Purpose**: The latest complete result delivered for a live scope.
- **Fields**:
  - `scope`
  - `version`
  - `payload`
  - `received_at`
- **Validation rules**:
  - A snapshot must be complete enough for the consumer to render current state.
  - Older versions cannot replace newer versions for the same query key/generation.

## 7. Backend Scope Subscription

- **Purpose**: Server-side representation of an attached live scope.
- **Fields**:
  - `scope_id`
  - `authorized_actor`
  - `scope_definition`
  - `broker_key`
  - `status`
  - `latest_snapshot_version`
- **Validation rules**:
  - The scope must be authorized before attachment.
  - The broker key must match the normalized scope definition.

## 8. Broker Channel

- **Purpose**: Shared backend fanout channel for all listeners of the same live scope.
- **Fields**:
  - `broker_key`
  - `listener_count`
  - `latest_snapshot`
  - `last_publish_at`
- **Relationships**:
  - One broker channel can serve many backend scope subscriptions.
  - Backend write routes publish refreshed snapshots into one or more broker channels.
