# Live Subscription Contract

## Purpose

Defines the externally visible contract for live vault and note subscriptions in the first rollout of centralized data flow.

## Subscription Scopes

The system exposes typed live scopes rather than arbitrary query strings.

### Supported initial scopes

- `vault-list/personal`
  - Returns the caller’s personal vault list for a server/home context.
- `vault-list/shared`
  - Returns the caller’s accessible shared server vault list.
- `note-list`
  - Returns the ordered note list for a specific vault context.
- `note`
  - Returns one note document for a specific vault/note context.

## Consumer Contract

### Subscribe request

A consumer requests a live scope by providing:

- `scope_kind`
- `server_slug`
- `owner_context` when applicable
- `vault_id` when applicable
- `note_id` when applicable

Equivalent requests must resolve to the same canonical scope identity.

### Delivery guarantees

- The first successful delivery for a scope is a complete snapshot.
- Subsequent deliveries replace the prior snapshot for that same scope.
- If multiple consumers request the same scope, they share one underlying active subscription.
- When the last consumer releases the scope, background observation ends.

## Event Envelope

Every delivered event must include:

- `scope`: the canonical scope identity
- `type`: snapshot, replace, keepalive, or error
- `version`: monotonically increasing snapshot version for that scope
- `timestamp`: event creation time
- `data`: payload for snapshot/replace events
- `message`: human-readable error context for error events when applicable

## Error Contract

### Invalid scope

- Returned when the requested scope is incomplete or malformed.
- No subscription is established.

### Forbidden scope

- Returned when the caller is not allowed to observe the requested scope.
- No subscription is established, or an existing subscription is terminated.

### Transient interruption

- Exposed as an error or reconnecting status without discarding the last known successful snapshot.

## Store Contract

Frontend stores consume live results through the centralized data-access layer.

- Stores must receive updates scoped to the originating canonical query.
- Stores remain the only supported read surface for views and components.
- Late subscribers must see the latest known snapshot immediately from shared state before waiting for a future event.
