# Implementation Plan: Vault Icons & Overview Redesign

**Branch**: `005-vault-icons-overview` | **Date**: 2026-03-23 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/005-vault-icons-overview/spec.md`

---

## Summary

Add an optional icon field to the `Vault` domain type (Rust + TypeScript), expose icon assignment via `POST /api/vaults` and a new `PATCH /api/vaults/:vault_id` endpoint, and redesign the `HomeView` vault cards into 1:1 aspect-ratio squares where the chosen icon renders at large scale at low opacity on the right side as a background accent.

---

## Technical Context

**Language/Version**: TypeScript 5.x / Vue 3.5 (frontend); Rust (backend — stable toolchain)
**Primary Dependencies**: Pinia 3.0, Vue Router 5.0, nyx-kit 2.x, ofetch 1.5 (frontend); Axum, serde/serde_json, notes-core, notes-storage-fs (backend)
**Storage**: Filesystem — `.vault.json` in each vault directory gains an optional `icon` field
**Testing**: Vitest (frontend composables/stores); `#[tokio::test]` with `FsStorage` on `tempdir` (storage); Axum integration tests (server); Playwright E2E
**Target Platform**: Linux server (backend); modern browser SPA (frontend)
**Project Type**: Self-hosted web application (Axum API + Vue SPA)
**Performance Goals**: No new performance concerns — vault list is small (tens of vaults)
**Constraints**: No new npm packages; no new Rust crates
**Scale/Scope**: Small — 3 Rust files, 7 frontend files, 5 doc files

---

## Constitution Check

| Principle | Status | Notes |
|---|---|---|
| I. Docs first | ✅ | All 5 affected docs updated before implementation |
| II. Strict layer boundaries | ✅ | `icon` in `notes-core` domain; validation in server handler; no leakage |
| III. Filesystem is source of truth | ✅ | `icon` added to `.vault.json`; optional field; backward compatible |
| IV. Test coverage | ✅ | Storage test for `update_vault`; server integration test for PATCH; Vitest for store action |
| V. Security | ✅ | PATCH enforces same ownership check as DELETE (`VaultOwner::User(caller_uid)`) |
| VI. Frontend constraints | ✅ | Composition API; nyx-kit only (inline SVG — no icon lib); no semicolons; single quotes; `storeToRefs` for reactive state |

**Gate result**: PASS — no violations.

---

## Project Structure

### Documentation (this feature)

```text
specs/005-vault-icons-overview/
├── plan.md              # This file
├── spec.md              # Feature specification
├── research.md          # Phase 0 — icon delivery, aspect-ratio, PATCH scope decisions
├── data-model.md        # Phase 1 — updated Vault type, VaultUpdate, .vault.json, TS types
├── quickstart.md        # Phase 1 — file map, implementation order, test checklist
├── contracts/
│   └── vault-api.md     # Phase 1 — PATCH /api/vaults/:vault_id + updated POST/GET
└── tasks.md             # Phase 2 output (/speckit.tasks command — NOT created by /speckit.plan)
```

### Source Code

```text
# Backend
crates/
  notes-core/src/
    domain.rs              # Vault.icon, VaultUpdate, VaultIconUpdate, StorageBackend::update_vault
  notes-storage-fs/src/
    lib.rs                 # FsStorage::update_vault, VaultJson serde struct update
  notes-server-axum/src/
    routes/
      vaults.rs            # patch_vault handler, CreateVaultBody.icon, route wiring

# Frontend
app/src/
  types/
    index.ts               # Vault.icon, UpdateVaultRequest, CreateVaultRequest.icon
  api/
    vaults.ts              # updateVault(vaultId, body)
  stores/
    vaults.ts              # update(vaultId, body) action
  assets/
    icons/
      home.svg             # NEW — one file per icon slug (20 total)
      briefcase.svg
      book.svg
      …
  components/
    VaultIcon.vue          # NEW — imports icons via Vite ?raw, renders by slug prop
    VaultIconPicker.vue    # NEW — 5×4 grid picker, emits 'select'
  views/
    HomeView.vue           # vault card redesign: 1:1 square, icon background
    VaultSettingsView.vue  # add icon picker section

# Docs
docs/architecture/
  core-domain.md
  vaults-and-teams.md
  filesystem-storage.md
  backend-api.md
docs/interface/
  frontend.md
```

**Structure Decision**: Single project — shared Rust workspace + Vue frontend in `app/`. No new packages or workspace members needed.

---

## Phase 0: Research

Completed. See [research.md](./research.md).

All decisions resolved:
- Icon delivery: inline SVG in `VaultIcon.vue` — no npm package needed
- Aspect ratio: CSS `aspect-ratio: 1 / 1` with absolute-positioned icon
- PATCH scope: general `UpdateVaultRequest` (name + icon) — single endpoint
- Icon validation: string allowlist in server handler — not a Rust enum
- Backward compat: optional field in `.vault.json` — no migration needed

---

## Phase 1: Design & Contracts

Completed. See:
- [data-model.md](./data-model.md) — Rust types, TS types, `.vault.json` format, validation rules
- [contracts/vault-api.md](./contracts/vault-api.md) — PATCH endpoint + updated POST/GET contracts
- [quickstart.md](./quickstart.md) — file map and implementation order

---

## Implementation Notes

### `VaultIcon.vue`

Accepts a `slug: string | undefined` prop. Each icon is a standalone SVG file in `app/src/assets/icons/{slug}.svg`. The component imports all 20 via Vite's `?raw` suffix (returns SVG markup as a string at build time) and selects the correct one by slug, falling back to `folder` for unknown or absent slugs. Rendered inline with `v-html` so `currentColor` works.

```vue
<VaultIcon slug="briefcase" size="80" class="vault-card__icon" />
```

All icon SVGs share the same conventions: `viewBox="0 0 24 24"`, `fill="none"`, `stroke="currentColor"`, `stroke-width="1.25"`, `stroke-linecap="round"`, `stroke-linejoin="round"`.

### `VaultIconPicker.vue`

Emits `select(slug: string)`. Accepts `modelValue: string | undefined` for controlled selection. Renders a 5-column CSS grid of 40×40px icon buttons. Active icon has `primary-container` background. Uses `VaultIcon` internally.

### HomeView vault card

```css
.home__vault-card {
  aspect-ratio: 1 / 1;
  position: relative;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  justify-content: flex-end;   /* text at bottom */
  align-items: flex-start;
  padding: 1.25rem;
}

.home__vault-card__icon {
  position: absolute;
  right: -0.5rem;              /* bleed slightly beyond edge */
  top: 50%;
  transform: translateY(-50%);
  opacity: 0.12;
  pointer-events: none;
  width: 70%;                  /* 70% of card width */
  height: auto;
}
```

### `update_vault` — FsStorage implementation

1. Resolve vault path by scanning `.vault.json` files (same as existing `delete_vault`)
2. Read existing `.vault.json` into `VaultJson` struct
3. Apply updates: replace `name` if `Some`; set/clear `icon` per `VaultIconUpdate`
4. Re-serialize and write back to `.vault.json`

### `patch_vault` — Axum handler

```
1. Extract caller uid from auth extractor
2. load_vault(vault_id) — 404 if not found
3. Check vault.owner == VaultOwner::User(caller_uid) — 403 otherwise
4. Validate body: at least one field present; icon in VALID_ICONS if provided
5. Build VaultUpdate from body
6. storage.update_vault(vault_id, &update)
7. Return updated vault as JSON 200
```

---

## Complexity Tracking

No violations — no complexity justification needed.
