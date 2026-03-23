# Quickstart: Vault Icons & Overview Redesign

**Branch**: `005-vault-icons-overview`

---

## What changed

1. **`Vault` type** gains an optional `icon` field (string slug) in both Rust and TypeScript
2. **`PATCH /api/vaults/:vault_id`** is a new endpoint for updating vault name/icon
3. **`HomeView`** vault cards are redesigned as 1:1 aspect-ratio squares with icon background
4. **Two new Vue components**: `VaultIcon.vue` and `VaultIconPicker.vue`
5. **`VaultSettingsView`** gains an icon picker section

---

## File map

### Rust

| File | Change |
|---|---|
| `crates/notes-core/src/domain.rs` | Add `icon: Option<String>` to `Vault`; add `VaultUpdate`, `VaultIconUpdate`; add `update_vault` to `StorageBackend` |
| `crates/notes-storage-fs/src/lib.rs` | Implement `update_vault`; read/write `icon` in `.vault.json` serde structs |
| `crates/notes-server-axum/src/routes/vaults.rs` | Add `patch_vault` handler; wire `PATCH /api/vaults/:vault_id`; accept `icon` in `CreateVaultBody` |

### Frontend

| File | Change |
|---|---|
| `frontend/src/types/index.ts` | Add `icon?` to `Vault`, `CreateVaultRequest`; add `UpdateVaultRequest` |
| `frontend/src/api/vaults.ts` | Add `updateVault(vaultId, body)` |
| `frontend/src/stores/vaults.ts` | Add `update(vaultId, body)` action |
| `frontend/src/assets/icons/*.svg` | **New** — 20 individual SVG asset files (one per slug) |
| `frontend/src/components/VaultIcon.vue` | **New** — imports icons via Vite `?raw`, renders by slug prop |
| `frontend/src/components/VaultIconPicker.vue` | **New** — 5×4 icon grid, emits `select` |
| `frontend/src/views/HomeView.vue` | Redesign vault cards: 1:1 square, icon as background |
| `frontend/src/views/VaultSettingsView.vue` | Add icon picker section |

### Docs

| File | Change |
|---|---|
| `docs/architecture/core-domain.md` | Update `Vault` struct; add `VaultUpdate`; add `update_vault` to trait |
| `docs/architecture/vaults-and-teams.md` | Add `icon` to domain type and `.vault.json` example; add PATCH route |
| `docs/architecture/filesystem-storage.md` | Update `.vault.json` format with optional `icon` field |
| `docs/architecture/backend-api.md` | Add `PATCH /api/vaults/:vault_id` route description |
| `docs/interface/frontend.md` | Add `VaultIcon`, `VaultIconPicker` to component list; update `HomeView` and `VaultSettingsView` descriptions |

---

## Implementation order

1. Docs first (all 5 doc files)
2. Rust domain (`notes-core/src/domain.rs`)
3. Rust storage (`notes-storage-fs/src/lib.rs`)
4. Rust server (new handler + route wiring)
5. TypeScript types
6. API client
7. Pinia store
8. `VaultIcon.vue`
9. `VaultIconPicker.vue`
10. `HomeView.vue` — card redesign
11. `VaultSettingsView.vue` — icon picker section

---

## Running locally

```bash
# Backend (from repo root)
cargo run -p notes-server-axum

# Frontend
cd frontend && npm run dev
```

Navigate to `/` to see the redesigned vault overview grid.

---

## Testing checklist

- [ ] Create vault with icon via `POST /api/vaults`
- [ ] Icon persisted in `.vault.json`
- [ ] `GET /api/vaults` returns `icon` field
- [ ] `PATCH /api/vaults/:id` with `{ icon: "book" }` → returns 200 with updated icon
- [ ] `PATCH /api/vaults/:id` with `{ icon: null }` → removes icon
- [ ] `PATCH /api/vaults/:id` with invalid slug → 400
- [ ] `PATCH /api/vaults/:id` on team-owned vault → 403
- [ ] Vault card in `HomeView` renders as square with icon background
- [ ] Cards without icon show no background icon
- [ ] Icon picker in create form selects and deselects correctly
- [ ] Icon picker in vault settings persists on save
