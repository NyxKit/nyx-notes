# Quickstart: Route Domains

## What's Changing

| File | Change |
|------|--------|
| `frontend/src/router/index.ts` | Add `/vaults/:vault_id` route → `VaultView.vue` |
| `frontend/src/views/HomeView.vue` | Rework: single-vault redirect + multi-vault masonry |
| `frontend/src/views/VaultView.vue` | **New file** — notes masonry + getting-started state |
| `docs/interface/frontend.md` | Update routing table, add VaultView description |

## Implementation Order

1. **Update docs first** (`docs/interface/frontend.md`) — constitution gate.
2. **Add route** in `router/index.ts` — `/vaults/:vault_id` → `VaultView.vue`.
3. **Create `VaultView.vue`** — notes masonry, empty state, new-note action.
4. **Rework `HomeView.vue`** — single-vault redirect, multi-vault masonry, create-vault form.

## Key Patterns

### Single-vault redirect (HomeView)
```ts
onMounted(async () => {
  await loadVaults()
  if (vaults.value.length === 1) {
    router.replace(`/vaults/${vaults.value[0].id}`)
    return
  }
})
```

### VaultView note masonry (reuse HomeView pattern)
```ts
const { vault_id } = useRoute().params
onMounted(async () => {
  await loadVaults()
  const vault = vaults.value.find(v => v.id === vault_id)
  if (vault) setActive(vault)
  await loadList(vault_id as string)
})
```

### Vault card (multi-vault home)
Each vault card `@click` → `router.push('/vaults/' + vault.id)`

## Dev Setup

No new dependencies. Frontend dev server:
```bash
cd frontend && pnpm dev
```
