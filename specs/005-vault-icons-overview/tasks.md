# Tasks: Vault Icons & Overview Redesign

**Input**: Design documents from `/specs/005-vault-icons-overview/`
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅, contracts/ ✅, quickstart.md ✅

**Tests**: Not explicitly requested — no test tasks generated.

**Organization**: Tasks grouped by user story. Docs updated first (constitution: docs are source of truth).

## Format: `[ID] [P?] [Story?] Description`

- **[P]**: Can run in parallel (different files, no inter-task dependencies)
- **[Story]**: User story this task belongs to (US1, US2, US3)

---

## Phase 1: Setup — Documentation

**Purpose**: Update all affected docs before any code changes. Required by the project constitution.

**⚠️ CRITICAL**: All doc updates must precede implementation.

- [x] T001 [P] Update `Vault` struct + add `VaultUpdate` / `VaultIconUpdate` + add `update_vault` to `StorageBackend` trait in `docs/architecture/core-domain.md`
- [x] T002 [P] Add `icon` field to `Vault` type, `.vault.json` examples, and `PATCH /api/vaults/:vault_id` to route table in `docs/architecture/vaults-and-teams.md`
- [x] T003 [P] Document optional `icon` field in `.vault.json` format (personal + team examples) in `docs/architecture/filesystem-storage.md`
- [x] T004 [P] Add `PATCH /api/vaults/:vault_id` row (with personal/team auth rules) to vault routes table in `docs/architecture/backend-api.md`
- [x] T005 [P] Add `VaultIcon.vue`, `VaultIconPicker.vue` to component list; update `HomeView` and `VaultSettingsView` descriptions in `docs/interface/frontend.md`

**Checkpoint**: All 5 doc files consistent — implementation can begin.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Domain model changes and shared frontend primitives that all user stories depend on.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete.

- [x] T006 Add `icon: Option<String>` to `Vault` struct; add `VaultUpdate` and `VaultIconUpdate` types; add `update_vault(&self, vault_id: &str, update: &VaultUpdate) -> Result<(), StorageError>` to `StorageBackend` trait in `crates/notes-core/src/domain.rs`
- [x] T007 Update `VaultJson` serde struct to include optional `icon` field; implement `FsStorage::update_vault` (read `.vault.json`, apply name/icon update, write back); forward `icon` through `FsStorage::create_vault` in `crates/notes-storage-fs/src/lib.rs` — depends on T006
- [x] T008 [P] Add `icon?: string` to `Vault` interface; add `icon?: string` to `CreateVaultRequest`; add `UpdateVaultRequest` interface in `app/src/types/index.ts`
- [x] T009 [P] Create all 20 SVG icon files in `app/src/assets/icons/` — one file per slug: `home.svg`, `book.svg`, `star.svg`, `briefcase.svg`, `code.svg`, `pen.svg`, `heart.svg`, `globe.svg`, `lock.svg`, `rocket.svg`, `lightbulb.svg`, `music.svg`, `camera.svg`, `folder.svg`, `compass.svg`, `flask.svg`, `graduation-cap.svg`, `chart.svg`, `leaf.svg`, `diamond.svg` — all using `viewBox="0 0 24 24"`, `fill="none"`, `stroke="currentColor"`, `stroke-width="1.25"`, `stroke-linecap="round"`, `stroke-linejoin="round"`
- [x] T010 Create `app/src/components/VaultIcon.vue` — accepts `slug: string | undefined` and `size: number` props; imports all 20 SVGs via Vite `?raw` suffix; selects by slug; falls back to `folder` for unknown/absent slugs; renders inline via `v-html`; sets width/height from `size` prop — depends on T008, T009

**Checkpoint**: Domain model updated, SVG assets ready, `VaultIcon` renderable — user story work can begin.

---

## Phase 3: User Story 1 — Icon assignment at vault creation (P1) 🎯 MVP

**Goal**: A user can select one of 20 icons in the inline create form when creating a vault; the icon is stored and returned by the API.

**Independent test**: Create a vault with `icon: "briefcase"` via `POST /api/vaults`; confirm `.vault.json` on disk contains `"icon": "briefcase"`; confirm `GET /api/vaults` returns the vault with `icon: "briefcase"`; confirm vault card in HomeView shows the briefcase icon.

- [x] T011 [P] [US1] Update `CreateVaultBody` struct in `crates/notes-server-axum/src/routes/vaults.rs` to include `icon: Option<String>`; validate icon against `VALID_ICONS` allowlist; pass icon through to `storage.create_vault` — depends on T007
- [x] T012 [P] [US1] Update `createVault(body: CreateVaultRequest)` in `app/src/api/vaults.ts` to include `icon` in the POST request body — depends on T008
- [x] T013 [US1] Update `create(body)` action in `app/src/stores/vaults.ts` to forward `icon` from the request body through to `createVault()` — depends on T012
- [x] T014 [P] [US1] Create `app/src/components/VaultIconPicker.vue` — 5-column CSS grid of 40×40px icon buttons; accepts `modelValue: string | undefined` prop; emits `update:modelValue` with selected slug; active icon uses `primary-container` background; uses `VaultIcon` internally; selection is optional (no icon = valid) — depends on T010
- [x] T015 [US1] Add `VaultIconPicker` to the inline create form card in `app/src/views/HomeView.vue` — add `newIcon` ref; bind to `VaultIconPicker`; pass `newIcon.value` to `createVault`; place picker between name field and form actions — depends on T013, T014

**Checkpoint**: Vault can be created with an icon via the UI and the API; icon persists to disk and is returned by `GET /api/vaults`.

---

## Phase 4: User Story 2 — Vault overview redesign (P2)

**Goal**: Every vault card in `HomeView` is a 1:1 aspect-ratio square with the vault icon rendered large and low-opacity at the right side; vaults without an assigned icon fall back to the `folder` icon.

**Independent test**: Navigate to `/`; all vault cards are square (width === height); each card shows an icon at opacity ~0.12 on the right; vault name and slug are visible at the bottom-left; cards with no assigned icon show the `folder` icon.

- [x] T016 [US2] Redesign vault card in `app/src/views/HomeView.vue` — add `aspect-ratio: 1 / 1` and `position: relative; overflow: hidden` to `.home__vault-card`; move text (name + slug) to bottom-left with `justify-content: flex-end; align-items: flex-start`; add `VaultIcon` absolutely positioned at right side, vertically centred, `width: 70%`, `opacity: 0.12`, `pointer-events: none`; slug `|| 'folder'` as fallback — depends on T010, T015
- [x] T017 [US2] Update `.home__skeleton-card` dimensions in `app/src/views/HomeView.vue` to use `aspect-ratio: 1 / 1` instead of fixed `height: 100px` — depends on T016

**Checkpoint**: Vault overview displays a visually consistent grid of square tiles with icon backgrounds.

---

## Phase 5: User Story 3 — Icon update post-creation via vault settings (P3)

**Goal**: A vault owner (personal) or team owner/admin (team vault) can change or clear a vault's icon after creation via `VaultSettingsView`.

**Independent test**: `PATCH /api/vaults/:id` with `{ "icon": "book" }` returns 200 with updated vault; subsequent `GET /api/vaults` reflects the new icon; `PATCH` with `{ "icon": null }` clears the icon; `PATCH` on a team vault by a non-admin returns 403; icon picker in `VaultSettingsView` reflects current icon and saves on confirm.

- [x] T018 [P] [US3] Add `patch_vault` handler to `crates/notes-server-axum/src/routes/vaults.rs` — parse `UpdateVaultRequest` body (at least one field required, 400 if empty); personal vault: check `VaultOwner::User(caller_uid)`, 403 otherwise; team vault: load team, check `TeamRole::Owner` or `Admin`, 403 otherwise; validate icon slug against `VALID_ICONS`; call `storage.update_vault`; return 200 with updated vault JSON; wire to `PATCH /api/vaults/:vault_id` route — depends on T007
- [x] T019 [P] [US3] Add `updateVault(vaultId: string, body: UpdateVaultRequest)` function in `app/src/api/vaults.ts` — PATCH to `/api/vaults/${vaultId}`, returns updated `Vault` — depends on T008
- [x] T020 [US3] Add `update(vaultId: string, body: UpdateVaultRequest)` action to `useVaultStore` in `app/src/stores/vaults.ts` — calls `updateVault`, replaces matching vault in `vaults` array, updates `activeVault` if it matches — depends on T019
- [x] T021 [US3] Add icon picker section to `app/src/views/VaultSettingsView.vue` (create the view file if it does not yet exist) — show `VaultIconPicker` bound to active vault's current icon; on change call `vaultStore.update(vaultId, { icon })` with `null` for clear; place section above the rename form — depends on T014, T020

**Checkpoint**: Icon can be changed and cleared post-creation; team vault icon respects owner/admin permission; vault settings UI reflects changes immediately.

---

## Phase 6: Polish & Cross-Cutting Concerns

- [x] T022 [P] Walk through `specs/005-vault-icons-overview/quickstart.md` test checklist and verify all acceptance criteria pass end-to-end
- [x] T023 [P] Update `AGENTS.md` Divergence Log if any doc/code discrepancies were found during implementation

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Docs)**: No dependencies — start immediately; all 5 tasks parallel
- **Phase 2 (Foundational)**: Depends on Phase 1 — blocks all user stories
  - T007 depends on T006; T010 depends on T008 + T009; T008 and T009 are parallel
- **Phase 3 (US1)**: Depends on Phase 2 — T011 + T012 + T014 parallel; T013 after T012; T015 after T013 + T014
- **Phase 4 (US2)**: Depends on Phase 3 (HomeView already modified) — T016 then T017 sequentially
- **Phase 5 (US3)**: Depends on Phase 2 — T018 + T019 parallel; T020 after T019; T021 after T014 + T020
- **Phase 6 (Polish)**: Depends on all story phases complete

### User Story Dependencies

- **US1 (P1)**: After Phase 2 — no dependency on US2 or US3
- **US2 (P2)**: After US1 (both modify `HomeView.vue`) — sequential in same file
- **US3 (P3)**: After Phase 2 — independent of US1/US2 (different files); can run in parallel with US1/US2 after Phase 2

### Parallel Opportunities Within Phases

```
Phase 1:  T001 ║ T002 ║ T003 ║ T004 ║ T005
Phase 2:  T006 → T007
          T008 ║ T009 → T010
Phase 3:  T011 ║ T012 → T013 ║ T014 → T015
Phase 5:  T018 ║ T019 → T020 → T021
Phase 6:  T022 ║ T023
```

---

## Implementation Strategy

### MVP First (US1 only — icon creation roundtrip)

1. Complete Phase 1: Docs
2. Complete Phase 2: Foundational
3. Complete Phase 3: US1
4. **STOP and VALIDATE**: Create vault with icon via UI; confirm icon stored and returned
5. Ship if ready

### Incremental Delivery

1. Phase 1 + 2 → Foundation ready
2. Phase 3 (US1) → Icon creation works end-to-end (MVP)
3. Phase 4 (US2) → Overview visually redesigned
4. Phase 5 (US3) → Icon editable post-creation
5. Phase 6 → Polish + validation

---

## Notes

- No test tasks — not requested in spec
- `VaultSettingsView.vue` (T021): check if file exists before creating; if it exists, add the icon picker section only
- All frontend TS/Vue files: no semicolons, single quotes (project convention)
- All Pinia state reads in components must use `storeToRefs` (project convention)
- `PATCH /api/vaults/:vault_id` must handle both `VaultOwner::User` and `VaultOwner::Team` with separate auth checks (clarified in spec)
- Icon slug `null` in JSON body = clear icon; absent field = no change (JSON Merge Patch semantics)
