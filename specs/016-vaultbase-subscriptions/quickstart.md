# Quickstart: Centralized Subscription Data Flow

## Goal

Validate that the new centralized subscription model provides live vault and note updates, reuses shared subscriptions, and cleans up correctly when consumers unmount.

## Prerequisites

- The application can render authenticated personal and shared vault flows.
- The backend can serve live subscription scopes for vault lists, note lists, and note documents.
- Test fixtures or helper flows exist to trigger note and vault changes while a subscription is active.

## Scenario 1: Vault list auto-updates

1. Open a view backed by the vault list subscription.
2. Create or delete a vault through another action path.
3. Confirm the visible vault list updates automatically without a manual refresh.
4. Confirm the list ordering and ownership grouping remain correct.

## Scenario 2: Note list auto-updates

1. Open a vault view backed by a note-list subscription.
2. Create, rename, or delete a note in that vault from a second action path.
3. Confirm the list updates automatically and reflects the correct note membership and ordering.

## Scenario 3: Note document auto-updates

1. Open a note view backed by a note-document subscription.
2. Modify the note content or metadata from another action path.
3. Confirm the note view reflects the change without route reload.

## Scenario 4: Shared subscription reuse

1. Mount two consumers for the same vault or note scope.
2. Confirm only one underlying active subscription exists for that canonical query.
3. Trigger a change and confirm both consumers receive the update.
4. Unmount one consumer and confirm the remaining consumer still receives updates.
5. Unmount the final consumer and confirm the scope is released.

## Scenario 5: Query-switch stale update protection

1. Subscribe to one vault or note scope.
2. Quickly switch to a different scope before the first scope finishes or before a delayed update arrives.
3. Confirm the old scope’s late update does not overwrite the current scope’s store state.

## Scenario 6: Transient interruption handling

1. Attach an active subscription and confirm a successful snapshot.
2. Simulate a temporary live-delivery interruption.
3. Confirm the consumer retains the last known snapshot and exposes a reconnecting or failed status.
4. Restore delivery and confirm live updates resume without duplicated subscriptions.
