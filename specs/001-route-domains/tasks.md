# Tasks: Route Domains

**Input**: Design documents from `/specs/001-route-domains/`
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅, contracts/routing.md ✅

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1, US2)
- No test tasks — not requested in spec

## Path Conventions

- Frontend source: `app/src/`
- Docs: `docs/interface/`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Documentation gate (constitution) + route registration that blocks all story work.

- [x] T001 Update routing table and add `VaultView` description in `docs/interface/frontend.md`
- [x] T002 Register `/vaults/:vault_id` route (→ `VaultView.vue`, `requiresAuth: true`) in `app/src/router/index.ts`

**Checkpoint**: Docs updated and route slot ready — user story implementation can now begin.

---

## Phase 2: User Story 1 — Vault Domain (Priority: P1) 🎯 MVP

**Goal**: Users navigating to `/vaults/:vault_id` see a masonry of their notes, or a getting-started prompt if the vault is empty. They can create a new note from this view.

**Independent Test**: Navigate to `/vaults/:vault_id` with notes → masonry renders. Navigate to `/vaults/:vault_id` with empty vault → getting-started prompt + "New Note" CTA visible.

- [x] T003 [US1] Create `app/src/views/VaultView.vue` with script setup: load vaults, set active vault, load notes for `vault_id` route param using `useVaults` and `useNotes`
- [x] T004 [US1] Add masonry notes grid to `VaultView.vue` (reuse `home__masonry` + `home__note-card` pattern from `HomeView.vue`) — renders when `notes.length > 0`
- [x] T005 [US1] Add getting-started empty state to `VaultView.vue` — renders when `notes.length === 0` and not loading; include "New Note" CTA that calls `useNotes().create()` and pushes to `/vaults/:vault_id/notes/:id`
- [x] T006 [US1] Add header bar to `VaultView.vue` showing vault name and a "New Note" action button; add loading skeleton state while `listLoading` is true
- [x] T007 [US1] Add scoped styles to `VaultView.vue` following the `home__*` BEM pattern (header, body, masonry, note-card, footer) using `var(--nyx-*)` tokens only

**Checkpoint**: `/vaults/:vault_id` is fully functional and independently testable.

---

## Phase 3: User Story 2 — Home Domain (Priority: P2)

**Goal**: `/` redirects to the vault automatically when only one vault exists. When multiple vaults exist, the user sees a masonry of vault cards and can create a new vault.

**Independent Test**: With one vault → landing on `/` pushes to `/vaults/:id` immediately. With multiple vaults → vault cards render; clicking a card navigates to that vault; "New Vault" form creates a vault and redirects.

- [x] T008 [US2] Rework `HomeView.vue` `onMounted`: after `loadVaults()`, if `vaults.value.length === 1` call `router.replace('/vaults/' + vaults.value[0].id)` and return early
- [x] T009 [P] [US2] Add vault masonry section to `HomeView.vue` — renders when `vaults.value.length > 1`; each vault card shows `vault.name`, `vault.slug`, and navigates to `/vaults/:vault_id` on click
- [x] T010 [P] [US2] Add "New Vault" inline form to `HomeView.vue` — toggled by a `showCreateForm` ref; contains `slug` and `name` inputs; on submit calls `useVaults().create()` then `router.push('/vaults/:newVault.id')` on success
- [x] T011 [US2] Add vault card and create-form scoped styles to `HomeView.vue` (vault-card, vault-slug badge, create-form panel) using existing `var(--nyx-*)` tokens; remove the note-masonry logic that is now handled by `VaultView.vue`

**Checkpoint**: All three route domains — Home, Vault, Note — are independently navigable and functional.

---

## Phase 4: Polish & Cross-Cutting Concerns

**Purpose**: Auth guard verification, dead-code removal, and quickstart validation.

- [x] T012 [P] Verify `/vaults/:vault_id` route in `app/src/router/index.ts` carries `meta: { requiresAuth: true }` and is covered by the existing `beforeEach` guard
- [x] T013 [P] Remove any note-masonry dead code from `HomeView.vue` that is now exclusively owned by `VaultView.vue` (e.g. `loadList`, `recentNotes`, note-card styles if fully moved)
- [ ] T014 Manually validate all quickstart.md scenarios: single-vault redirect, multi-vault masonry, vault empty state, note navigation round-trip

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — start immediately
- **User Story 1 (Phase 2)**: Depends on T002 (route registered) — `VaultView.vue` must be importable
- **User Story 2 (Phase 3)**: Depends on US1 completion — `HomeView` redirect target (`/vaults/:vault_id`) must exist
- **Polish (Phase 4)**: Depends on all story phases complete

### User Story Dependencies

- **US1 (Vault domain)**: Unblocked after Phase 1 — no dependency on US2
- **US2 (Home domain)**: Logically follows US1 — redirect destination must exist first

### Within Each User Story

- T003 before T004, T005, T006, T007 (file must exist before edits)
- T008 before T009, T010 (structural rework before additive changes)
- T009 and T010 are [P] — different logical sections, no conflicts

---

## Parallel Opportunities

```bash
# Phase 2 tasks T004–T007 can all begin once T003 (file created) is done:
Task: T004 — masonry grid in VaultView.vue
Task: T005 — empty state in VaultView.vue
Task: T006 — header bar in VaultView.vue
Task: T007 — scoped styles in VaultView.vue

# Phase 3 tasks T009 and T010 can run in parallel after T008:
Task: T009 — vault masonry section in HomeView.vue
Task: T010 — create-vault inline form in HomeView.vue

# Phase 4 tasks T012 and T013 are fully parallel:
Task: T012 — verify auth guard
Task: T013 — remove dead code
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001–T002)
2. Complete Phase 2: User Story 1 (T003–T007)
3. **STOP and VALIDATE**: Navigate to `/vaults/:vault_id` — masonry and empty state work
4. Deliver Vault domain as a standalone increment

### Incremental Delivery

1. Setup → Foundation ready (T001–T002)
2. US1 (T003–T007) → Vault domain live → demo
3. US2 (T008–T011) → Home domain live → demo
4. Polish (T012–T014) → production-ready

---

## Notes

- No new npm packages — all patterns reuse existing `home__*` CSS and `useVaults`/`useNotes` composables
- No semicolons; single quotes throughout all Vue/TS files
- `VaultView.vue` must use Composition API (`<script setup>`) exclusively
- `nyx-kit` components (NyxButton, etc.) preferred over raw `<button>` where available
- Commit after each task or logical group
