# Quickstart: Centralized Subscription Data Flow

## Goal

Validate that the new centralized subscription model provides live vault and note updates, reuses shared subscriptions, and cleans up correctly when consumers unmount.

## Prerequisites

- The application can render authenticated personal and shared vault flows.
- The backend can serve live subscription scopes for vault lists, note lists, and note documents.
- Test fixtures or helper flows exist to trigger note and vault changes while a subscription is active.

## Validation Outcomes

### ✅ Scenario 1: Vault list auto-updates
**Status**: Implemented
- NyxBase subscribes to vault_list_personal scope on mount
- Backend publishes to broker on vault create/delete/update
- Frontend receives live updates via SSE

### ✅ Scenario 2: Note list auto-updates
**Status**: Implemented
- NyxBase subscribes to note_list scope per vault
- Backend publishes on note CRUD operations
- Notes store updates reactively via subscription callback

### ✅ Scenario 3: Note document auto-updates
**Status**: Implemented
- NyxBase subscribes to note document scope
- NoteView uses useSubscription to call subscribeNote
- Active note updates via live snapshot

### ✅ Scenario 4: Shared subscription reuse
**Status**: Implemented and tested
- NyxBase singleton tracks refCount per vault key
- First subscriber loads snapshot + starts EventSource
- Second+ subscriber increments refCount, reuses connection
- Only refCount=0 triggers cleanup

### ✅ Scenario 5: Query-switch stale update protection
**Status**: Implemented
- Generation increments on release (subscriptionManager)
- Listeners receive generation in callback
- Stale updates can be detected by comparing generation

### ✅ Scenario 6: Transient interruption handling
**Status**: Hardened
- NyxBase retries initial snapshot up to 3 times
- EventSource reconnect with exponential backoff (max 5 attempts)
- Max retry delay capped at 30 seconds
- subscriptionManager fails with descriptive error after max retries

## Test Coverage

- Frontend: subscriptionManager.dedupe.spec.ts, subscriptionManager.generation.spec.ts, useSubscription.spec.ts, vaultBase.spec.ts
- Backend: subscriptions_api.rs (live scopes, broker fanout, version tracking)

## API Reference

### Frontend
```typescript
// Subscribe to live updates
const handle = NyxBase.subscribe(query, (snapshot) => {
  store.value = snapshot
})

// Release when done
handle.release()

// Query builders
NyxBase.createNoteListQuery(serverSlug, vaultId)
NyxBase.createNoteQuery(serverSlug, vaultId, noteId)
```

### Backend Routes
- `GET /api/live?collection=...&scope_kind=...&server_slug=...&vault_id=...&note_id=...`
