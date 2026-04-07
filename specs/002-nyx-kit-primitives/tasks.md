# Tasks: nyx-kit Primitives Refactor

**Input**: Design documents from `/specs/002-nyx-kit-primitives/`
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅, contracts/ui-primitives.md ✅

**Organization**: Tasks grouped by user story. Each story targets a different primitive class and can be implemented and verified independently.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no shared state)
- **[Story]**: US1 = interactive controls, US2 = badges/tags, US3 = form wrappers
- No test tasks — not requested in spec (manual regression per quickstart.md)

## Path Conventions

- Frontend source: `app/src/`
- Docs: `docs/interface/`

---

## Phase 1: Setup

**Purpose**: Docs gate (constitution Principle I) + establish the reference import pattern.

- [x] T001 Update `docs/interface/frontend.md` component library section to state: no native `<button>`, `<input>`, `<textarea>`, or badge `<span>` permitted; nyx-kit equivalents required

**Checkpoint**: Docs updated — story implementation can begin.

---

## Phase 2: Foundational

**Purpose**: No foundational blockers — nyx-kit is already installed. No new dependencies or configuration needed. Proceed directly to user stories.

---

## Phase 3: User Story 1 — Interactive Controls (Priority: P1) 🎯 MVP

**Goal**: Every `<button>`, `<input>`, and `<textarea>` in the frontend is replaced with `NyxButton`, `NyxInput`, or `NyxTextarea`. Scoped CSS for those primitives is removed.

**Independent Test**: Open every view. Inspect DOM — no native `<button>`, `<input>`, or `<textarea>` outside third-party output. All controls behave identically to before.

- [x] T002 [P] [US1] Replace `<button>` New Note with `NyxButton :gradient="true"` in `app/src/components/SidebarNav.vue`; remove `.sidebar-nav__new-btn` scoped styles
- [x] T003 [P] [US1] Replace `<button>`s in `app/src/components/NoteList.vue`: New Note button → `NyxButton`, note item buttons → `NyxButton variant="ghost"`; remove corresponding scoped styles
- [x] T004 [P] [US1] Replace `<button>` Resolve and Delete in `app/src/components/CommentThread.vue` with `NyxButton variant="ghost" size="sm"` (Delete adds `theme="danger"`); remove scoped button styles
- [x] T005 [P] [US1] Replace add-comment `<button>` in `app/src/components/CommentSidebar.vue` with `NyxButton`; remove scoped button styles
- [x] T006 [P] [US1] Replace `<textarea>` in `app/src/components/CommentComposer.vue` with `NyxTextarea`; replace `<button>` Submit with `NyxButton type="submit"` and Cancel with `NyxButton variant="ghost"`; remove scoped textarea/button styles
- [x] T007 [P] [US1] Replace all icon `<button>`s (sidebar toggle, source view, favorite, delete, comments) in `app/src/views/NoteView.vue` with `NyxButton variant="ghost" shape="square"`; delete button uses `theme="danger"`; remove scoped `.app-shell__icon-btn` styles
- [x] T008 [P] [US1] Replace `<button>`s and `<input>`s in `app/src/views/HomeView.vue`: New Vault → `NyxButton`, vault cards → `NyxButton variant="ghost"`, Create/Cancel → `NyxButton`/`NyxButton variant="ghost"`, name/slug inputs → `NyxInput`; remove corresponding scoped styles
- [x] T009 [P] [US1] Replace `<button>`s in `app/src/views/VaultView.vue`: New Note (header) → `NyxButton`, note cards → `NyxButton variant="ghost"`, New Note (empty CTA) → `NyxButton :gradient="true"`; remove scoped button styles
- [x] T010 [P] [US1] Replace `<button>`s in `app/src/views/VaultSettingsView.vue`: Back → `NyxButton variant="ghost"`, Delete → `NyxButton theme="danger"`; remove scoped button styles
- [x] T011 [P] [US1] Replace `<button>`s and `<input>`s in `app/src/views/TeamSettingsView.vue`: Back → `NyxButton variant="ghost"`, Remove member → `NyxButton variant="ghost" theme="danger" size="sm"`, Add member/vault submit → `NyxButton type="submit"`, Cancel → `NyxButton variant="ghost"`, Delete vault/team → `NyxButton theme="danger"`, all inputs → `NyxInput`; remove scoped button/input styles

**Checkpoint**: Zero native `<button>`, `<input>`, or `<textarea>` elements remain. All controls function identically.

---

## Phase 4: User Story 2 — Badges and Tags (Priority: P2)

**Goal**: All hand-rolled tag/badge `<span>` elements replaced with `NyxBadge`. Scoped tag CSS removed.

**Independent Test**: Open vault notes masonry and note editor toolbar. All tag chips render as `NyxBadge`. No `.note-tag`, `.vault__note-tag`, or equivalent span class exists in the DOM.

- [x] T012 [P] [US2] Replace tag `<span>` elements in `app/src/views/VaultView.vue` with `<NyxBadge theme="primary" variant="soft">`; remove `.vault__note-tag` scoped styles
- [x] T013 [P] [US2] Replace tag `<span>` elements in `app/src/components/NoteList.vue` with `<NyxBadge theme="primary" variant="soft">`; remove corresponding tag scoped styles
- [x] T014 [US2] Replace tag `<span>` elements in `app/src/components/NoteToolbar.vue` with `<NyxBadge theme="primary" variant="soft" :hasClose="true" @close="removeTag(tag)">`; remove `.note-toolbar__tag` scoped styles

**Checkpoint**: Zero hand-rolled tag spans remain. Tags render and dismiss correctly.

---

## Phase 5: User Story 3 — Form Wrappers (Priority: P3)

**Goal**: All form containers use `NyxForm` + `NyxFormField`. Pattern matches `LoginView.vue`.

**Independent Test**: Open vault creation form (HomeView), add-member form (TeamSettingsView), and comment composer. Component tree shows `NyxForm` at form root; labelled inputs use `NyxFormField`.

- [x] T015 [P] [US3] Wrap vault creation form in `app/src/views/HomeView.vue` with `NyxForm @submit="submitCreate"`; wrap name and slug fields with `NyxFormField label="..."` using slot-scope `{ id }` pattern
- [x] T016 [P] [US3] Wrap comment body form in `app/src/components/CommentComposer.vue` with `NyxForm @submit="submit"`; wrap textarea with `NyxFormField`
- [x] T017 [US3] Wrap add-member and add-vault forms in `app/src/views/TeamSettingsView.vue` with `NyxForm`; wrap each input with `NyxFormField label="..."`

**Checkpoint**: All forms use `NyxForm`. Submit handlers fire correctly. No bare form divs remain.

---

## Phase 6: Polish & Cross-Cutting Concerns

- [x] T018 [P] Replace hand-rolled tab buttons in `app/src/components/CommentSidebar.vue` with `NyxTabs v-model="activeTab" :tabs="['Open', 'Resolved']"`; slot comment thread lists under `#tab-Open` and `#tab-Resolved`; remove scoped tab button styles
- [x] T019 [P] Audit all touched files for any remaining scoped CSS that styles replaced primitives; remove dead style blocks
- [ ] T020 Manually verify all quickstart.md scenarios: every view opens without errors, controls interact correctly, v-model bindings update, disabled states respected, tags dismiss

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — start immediately
- **US1 (Phase 3)**: Unblocked after T001 — all 10 tasks [P] and can run simultaneously
- **US2 (Phase 4)**: Unblocked after Phase 1 — independent of US1 (different elements)
- **US3 (Phase 5)**: Requires US1 complete — inputs must be `NyxInput` before wrapping in `NyxFormField`
- **Polish (Phase 6)**: Requires all story phases complete

### Within Each User Story

- US1: T002–T011 are all [P] — entirely different files, no conflicts
- US2: T012–T013 are [P]; T014 depends on nothing but is in the same component area
- US3: T015 and T016 are [P]; T017 after both to validate the pattern

---

## Parallel Opportunities

```bash
# All of US1 can run simultaneously — each task is a different file:
T002 — SidebarNav.vue
T003 — NoteList.vue
T004 — CommentThread.vue
T005 — CommentSidebar.vue
T006 — CommentComposer.vue
T007 — NoteView.vue
T008 — HomeView.vue
T009 — VaultView.vue
T010 — VaultSettingsView.vue
T011 — TeamSettingsView.vue

# US2 can run in parallel with US1 (different elements, same files):
T012 — VaultView.vue (badges only, no conflict with T009 buttons)
T013 — NoteList.vue (badges only, no conflict with T003 buttons)

# US3 T015 and T016 can run in parallel:
T015 — HomeView.vue form wrapper
T016 — CommentComposer.vue form wrapper
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete T001 (docs gate)
2. Complete T002–T011 in parallel (all US1)
3. **STOP and VALIDATE**: Open every view — zero native buttons/inputs
4. Deliver as standalone increment

### Incremental Delivery

1. T001 → docs gate
2. US1 (T002–T011) → controls compliant → validate
3. US2 (T012–T014) → badges compliant → validate
4. US3 (T015–T017) → forms compliant → validate
5. Polish (T018–T020) → tabs + final audit

---

## Notes

- Import pattern for every touched file: `import { NyxButton, ... } from 'nyx-kit/components'` and `import { NyxVariant, NyxTheme, NyxSize } from 'nyx-kit/types'` (only import what's used)
- No semicolons; single quotes throughout
- Reference `LoginView.vue` for the correct `NyxForm` + `NyxFormField` pattern before implementing US3
- When removing scoped CSS, verify the removed class is not used elsewhere in the same file before deleting
