# Data Model: Pinia Stores for Vaults and Notes

**Feature**: `004-pinia-stores`
**Phase**: 1 — Design
**Date**: 2026-03-23

---

## New Types

None. All types (`Vault`, `NoteMeta`, `Note`, etc.) remain in `app/src/types/index.ts` unchanged.

---

## New Stores

### `useVaultStore` — `app/src/stores/vaults.ts`

Replaces `useVaults()` composable.

#### State

| Field | Type | Initial | Description |
|-------|------|---------|-------------|
| `vaults` | `Ref<Vault[]>` | `[]` | All loaded vaults |
| `activeVault` | `Ref<Vault \| null>` | `null` | Currently selected vault; null on HomeView |
| `loading` | `Ref<boolean>` | `false` | True while `load()` is in flight |
| `error` | `Ref<string \| null>` | `null` | Last error message |

#### Actions

| Action | Signature | Description |
|--------|-----------|-------------|
| `load()` | `() => Promise<void>` | Fetch all vaults from API; sets `loading` |
| `create()` | `(body: CreateVaultRequest) => Promise<Vault>` | Create vault; appends to `vaults` |
| `remove()` | `(vaultId: string) => Promise<void>` | Delete vault; removes from `vaults`; clears `activeVault` if matched |
| `setActive()` | `(vault: Vault \| null) => void` | Set or clear the active vault |
| `patchPermission()` | `(teamId, vaultId, permission) => Promise<void>` | Update vault permission in-place |
| `addTeamVault()` | `(teamId, body) => Promise<Vault>` | Create team vault; appends to `vaults` |
| `removeTeamVault()` | `(teamId, vaultId) => Promise<void>` | Delete team vault; removes from `vaults` |
| `$reset()` | `() => void` | Restore all state to initial values |

---

### `useNotesStore` — `app/src/stores/notes.ts`

Replaces `useNotes()` composable.

#### State

| Field | Type | Initial | Description |
|-------|------|---------|-------------|
| `notesByVault` | `Ref<Record<string, NoteMeta[]>>` | `{}` | Vault-keyed notes cache |
| `activeNote` | `Ref<Note \| null>` | `null` | Full note content currently open in editor |
| `listLoading` | `Ref<boolean>` | `false` | True while `loadList()` is in flight (shows skeleton in VaultView) |
| `loading` | `Ref<boolean>` | `false` | True while `loadNote()` is in flight |
| `saving` | `Ref<boolean>` | `false` | True while `save()` is in flight |
| `error` | `Ref<string \| null>` | `null` | Last error message |

#### Actions / Functions

| Name | Signature | Description |
|------|-----------|-------------|
| `notesFor()` | `(vaultId: string) => NoteMeta[]` | Returns cached notes for one vault (`[]` if not loaded) |
| `loadAll()` | `(vaultIds: string[]) => Promise<void>` | Fetch all vaults' notes in parallel; does not set `listLoading` |
| `loadList()` | `(vaultId: string) => Promise<void>` | Fetch one vault's notes; sets `listLoading`; writes into `notesByVault[vaultId]` |
| `loadNote()` | `(vaultId, id) => Promise<void>` | Fetch full note; sets `activeNote` |
| `create()` | `(vaultId, body) => Promise<NoteMeta>` | Create note; prepends to `notesByVault[vaultId]` |
| `save()` | `(vaultId, id, body) => Promise<NoteMeta>` | Update note; updates entry in `notesByVault[vaultId]` and `activeNote` |
| `remove()` | `(vaultId, id) => Promise<void>` | Delete note; removes from `notesByVault[vaultId]`; clears `activeNote` if matched |
| `updatePermission()` | `(vaultId, id, permission) => Promise<NoteMeta>` | Patch permission; updates entry in cache and `activeNote` |
| `$reset()` | `() => void` | Restore all state to initial values |

---

## Deleted Files

| File | Replacement |
|------|-------------|
| `app/src/composables/useNotes.ts` | `app/src/stores/notes.ts` |
| `app/src/composables/useVaults.ts` | `app/src/stores/vaults.ts` |

---

## Call Site Migration

All 10 files that import `useNotes` or `useVaults` need two changes:
1. Import path: `@/composables/useX` → `@/stores/x`
2. Function name: `useNotes()` → `useNotesStore()`, `useVaults()` → `useVaultStore()`

| File | Imports to update |
|------|------------------|
| `components/AppLayout.vue` | `useVaults`, `useNotes` |
| `components/NoteList.vue` | `useVaults`, `useNotes` |
| `components/NoteEditor.vue` | `useNotes` |
| `components/VaultSwitcher.vue` | `useVaults`, `useNotes` |
| `components/SidebarNav.vue` | `useVaults`, `useNotes` |
| `views/HomeView.vue` | `useVaults` |
| `views/VaultView.vue` | `useVaults`, `useNotes` |
| `views/NoteView.vue` | `useVaults`, `useNotes` |
| `views/TeamSettingsView.vue` | `useVaults` |
| `views/VaultSettingsView.vue` | `useVaults` |
