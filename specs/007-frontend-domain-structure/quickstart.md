# Quickstart: Frontend Domain-Based Structure

**Feature**: 007-frontend-domain-structure
**Date**: 2026-03-25

## What Is Being Done

`app/src/` is reorganised from a flat concern-based layout into a domain-based layout. Files move; no new logic is introduced.

## Developer Guide: Where Things Live After the Refactor

### Finding a feature

If you want to change how **vaults** work → open `app/src/vaults/`
If you want to change how **notes** work → open `app/src/notes/`
If you want to change **comments** → open `app/src/comments/`
If you want to change **auth/login** → open `app/src/auth/`
If you want to change **teams** → open `app/src/teams/`
If you want to change **layout, routing, shared types, or utilities** → open `app/src/shared/`

### Domain folder layout (consistent across all domains)

```
domain/
├── api/            # ofetch API module for this domain's endpoints
├── assets/         # domain-owned static assets (only vaults/ has this)
├── components/     # Vue components used only within this domain
├── composables/    # useX.ts composables (only if no Pinia store exists yet)
├── stores/         # Pinia stores for this domain
├── types/          # domain-specific types (only if NOT cross-domain)
├── utils/          # domain-specific utilities
└── views/          # full-page Vue route views
```

Subdirectories that have no files are **omitted** — don't create empty folders.

### Adding a new file

- New component for the notes domain → `app/src/notes/components/MyComponent.vue`
- New API call for vaults → add to `app/src/vaults/api/vaults.ts`
- New utility used by two or more domains → `app/src/shared/utils/myUtil.ts`
- New type used by two or more domains → add to `app/src/shared/types/index.ts`
- New Pinia store for comments → `app/src/comments/stores/comments.ts`

### Import paths

The `@/` alias maps to `app/src/`. Examples:

```ts
// Importing from within the same domain
import { useVaultStore } from '@/vaults/stores/vaults'
import VaultCard from '@/vaults/components/VaultCard.vue'

// Importing shared utilities or types from any domain
import type { Vault, Note } from '@/shared/types'
import { formatDate } from '@/shared/utils/time'
import { client } from '@/shared/api/client'
```

No alias changes were made to `vite.config.ts`.

## Running the App

```bash
cd frontend
pnpm dev      # starts Vite dev server on :1420, proxies /api to localhost:$PORT by default
```

The backend must be running on the configured `PORT` for API calls to work (default `:8080`).

## Verifying the Refactor

After the move, verify:

1. `pnpm build` succeeds with zero TypeScript or import errors
2. `pnpm dev` starts without console errors
3. All routes work: `/`, `/vaults/:id`, `/vaults/:id/notes/:id`, `/login`, `/vaults/:id/settings`, `/teams/:id/settings`
4. Vault icons render correctly (they reference moved SVG assets)
