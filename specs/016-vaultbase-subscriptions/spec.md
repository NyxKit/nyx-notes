# Feature Specification: Centralized Subscription Data Flow

**Feature Branch**: `016-vaultbase-subscriptions`  
**Created**: 2026-04-09  
**Status**: Draft  
**Input**: User description: "this feature will be centralized around optimizing en centralizing data flow. we need:
- subscriptions => think of firebase collection/document subscriptions, for our backend
- this means abstracting a new layer out of the current api into a full-fletched \"VaultBase\"
- this means the domain stores are then calling VaultBase.subscribe with simple queries: collection (vaults vs notes - vaultId/noteId), server, user, ...)
- composables are then used to subscribe/unsubscribe on mount/unmount
- useVaults talks with the vault store but also subscribes and unsubscribes on mount/unmount
- since multiple composables can subscribe, to prevent multiple subscriptions (n) and having n redundant calls, we need a subscription manager as well

think DEEP on how to implement this, this is a highly advanced setup"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Live Vault And Note Updates (Priority: P1)

As a signed-in user, I want vault and note screens to stay current automatically so I do not need to trigger repeated manual refreshes while browsing or editing.

**Why this priority**: The core value of the feature is centralized, subscription-based data flow. If live updates are not reliable for vault and note views, the feature does not deliver its primary benefit.

**Independent Test**: Can be fully tested by opening a vault or note view, changing the underlying data from another session or action path, and confirming the visible state updates without a manual reload.

**Acceptance Scenarios**:

1. **Given** a user has a vault view open, **When** the set of notes in that vault changes, **Then** the visible note list updates automatically and remains ordered correctly.
2. **Given** a user has a note view open, **When** that note changes, **Then** the visible note content and metadata update automatically without requiring a route refresh.
3. **Given** a user has access to both personal and shared vaults, **When** they subscribe to one scope, **Then** updates are limited to the requested scope and do not overwrite unrelated store state.

---

### User Story 2 - Shared Subscription Reuse Across Screens (Priority: P2)

As a user navigating across screens that need the same data, I want the app to reuse active subscriptions so repeated mounts do not create redundant background work or inconsistent state.

**Why this priority**: Centralization only improves performance and correctness if the same data request is shared across callers rather than recreated for every composable or component instance.

**Independent Test**: Can be fully tested by mounting multiple consumers for the same data scope and verifying they share one active subscription while each still receives updates.

**Acceptance Scenarios**:

1. **Given** two active UI consumers request the same vault or note data, **When** both are mounted, **Then** the system maintains one shared subscription for that query and updates both consumers from the same source.
2. **Given** one of several consumers stops observing a shared query, **When** the others remain mounted, **Then** the shared subscription stays active and data updates continue for remaining consumers.
3. **Given** the final consumer of a shared query unsubscribes, **When** no other consumer needs that data, **Then** the subscription is released and no further background updates are requested for that query.

---

### User Story 3 - Predictable Composable Lifecycle Management (Priority: P3)

As a frontend developer extending the app, I want composables to manage subscribe and unsubscribe behavior consistently so screens can opt into live data without reimplementing lifecycle logic.

**Why this priority**: This feature is meant to simplify data flow across the app. Reusable lifecycle behavior is what makes the model scalable beyond the first vault and note screens.

**Independent Test**: Can be fully tested by adding a composable-backed consumer for an existing query type and verifying it receives data on mount, releases it on unmount, and leaves store state coherent after route transitions.

**Acceptance Scenarios**:

1. **Given** a composable subscribes to a data scope on mount, **When** the owning view is destroyed or navigated away from, **Then** the composable unregisters its interest automatically.
2. **Given** a composable changes from one query scope to another, **When** its inputs change, **Then** the old subscription is released and the new scope becomes active without duplicated updates.
3. **Given** a store receives updates from the centralized data layer, **When** a consumer remounts, **Then** the consumer sees the latest store state immediately and continues receiving live updates.

---

### Edge Cases

- What happens when multiple consumers subscribe to the same query at nearly the same time?
- What happens when a consumer switches from one query scope to another before the first scope finishes initializing?
- How does the system handle temporary connectivity loss while preserving the last known data state?
- How does the system behave when a subscription target becomes unauthorized or inaccessible after it was already active?
- How does the system prevent stale updates from a released subscription from overwriting newer state?
- What happens when a shared subscription fails for one query while other active queries remain healthy?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST provide a centralized data-access layer for vault-scoped and note-scoped data retrieval and live observation.
- **FR-002**: The centralized data-access layer MUST support subscription queries that identify the target data scope using business identifiers such as collection type, server context, user context, vault context, and note context.
- **FR-003**: The system MUST deliver an initial result for a valid subscription query before or together with subsequent live updates so consumers can render a complete first state.
- **FR-004**: Domain stores MUST own subscription-facing data access through the centralized data-access layer rather than issuing separate ad hoc fetch flows for the same data scope.
- **FR-005**: The system MUST include a shared subscription manager that deduplicates equivalent active queries across multiple consumers.
- **FR-006**: The subscription manager MUST maintain reference tracking so a shared query remains active until the last consumer releases it.
- **FR-007**: The system MUST ensure that unsubscribing one consumer never interrupts delivery for other active consumers of the same query.
- **FR-008**: The system MUST stop background observation for a query after the final consumer unsubscribes.
- **FR-009**: Composables that expose live data MUST call store-level subscribe and unsubscribe behavior during their active lifecycle and release that behavior automatically when the lifecycle ends.
- **FR-010**: The `useVaults` flow MUST both coordinate with the vault store and manage its live subscription lifecycle for the vault data it depends on.
- **FR-011**: The system MUST support both collection-style subscriptions and document-style subscriptions where the query scope requires either a list result or a single entity result.
- **FR-012**: The system MUST keep store updates scoped to the originating query so updates for one vault, note, user, or server context do not overwrite unrelated cached state.
- **FR-013**: The system MUST reject invalid or incomplete subscription queries with a clear failure state rather than creating an ambiguous partial subscription.
- **FR-014**: The system MUST expose subscription status to consumers so they can distinguish between loading, active, released, and failed observation states.
- **FR-015**: The system MUST preserve the most recent successful store state during transient subscription interruptions until a newer valid state is available or the consumer leaves the scope.
- **FR-016**: The system MUST prevent duplicate update delivery from the same underlying change when multiple consumers share a subscription.
- **FR-017**: The system MUST ensure that released or stale subscriptions cannot apply updates after a newer subscription has taken ownership of the same consumer context.
- **FR-018**: The system MUST support the initial live observation scopes for personal vault lists, shared vault lists, vault note lists, and individual note documents.

### Key Entities *(include if feature involves data)*

- **VaultBase Query**: A normalized request that describes which data scope should be observed, including collection type and the business identifiers needed to resolve that scope.
- **Subscription Handle**: A tracked consumer registration that represents one caller's active interest in a specific query and can be released without affecting unrelated callers.
- **Shared Subscription**: The canonical active observation for a deduplicated query, including its current lifecycle state, latest delivered result, and reference count.
- **Observed Dataset**: The vault, note, or other scoped data payload currently being kept in sync for consumers.
- **Subscription Manager Record**: Internal tracking state that links equivalent queries to their shared subscription and participating consumers.

## Assumptions

- The first rollout focuses on the existing vault and note domains, while leaving room to extend the same subscription model to additional domains later.
- Equivalent queries are defined by the full set of business identifiers required to address a data scope, not by which component or composable requested them.
- Consumers continue reading rendered state from domain stores; the new capability changes how stores are fed, not where views read data from.
- Stores own `VaultBase` access and payload normalization; composables own lifecycle orchestration around those store methods.
- A single consumer may change query scope over time, and the system should treat that as release-plus-resubscribe rather than mutating an existing query in place.
- Permission enforcement remains authoritative at the backend boundary even when data is delivered through a long-lived subscription channel.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In primary vault and note workflows, users see relevant changes reflected without performing a manual refresh in at least 95% of tested update events.
- **SC-002**: When 5 or more UI consumers request the same data scope, the system maintains a single active underlying subscription for that scope.
- **SC-003**: After the final consumer leaves a subscribed scope, the system releases that scope's background observation within 2 seconds.
- **SC-004**: During route transitions between subscribed views, users do not see unrelated vault or note data flash into the current view in any acceptance test scenario.
- **SC-005**: A new composable consumer can adopt the centralized subscription flow for an existing query type without defining a separate fetch lifecycle outside the shared model.
