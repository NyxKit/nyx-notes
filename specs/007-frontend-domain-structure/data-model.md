# Data Model: File Migration Map

**Feature**: 007-frontend-domain-structure
**Date**: 2026-03-25

This document is the authoritative migration map for the `frontend/src/` restructure. Every file in the current flat layout is mapped to its new domain location.

---

## Migration Table

| Current Path | New Path | Domain | Notes |
|---|---|---|---|
| `src/api/auth.ts` | `src/auth/api/auth.ts` | auth | |
| `src/api/client.ts` | `src/shared/api/client.ts` | shared | Base HTTP client used by all API modules |
| `src/api/comments.ts` | `src/comments/api/comments.ts` | comments | |
| `src/api/notes.ts` | `src/notes/api/notes.ts` | notes | |
| `src/api/teams.ts` | `src/teams/api/teams.ts` | teams | |
| `src/api/vaults.ts` | `src/vaults/api/vaults.ts` | vaults | |
| `src/api/.gitkeep` | deleted | — | Replaced by domain api/ subfolders |
| `src/assets/icons/*.svg` (generic outline) | `src/shared/assets/icons/*.svg` | shared | 20 generic outline icon SVGs |
| `src/assets/icons/vaults/*.svg` | `src/vaults/assets/icons/*.svg` | vaults | 20 solid vault-themed icon SVGs |
| `src/assets/theme.css` | `src/shared/assets/theme.css` | shared | |
| `src/assets/vue.svg` | `src/shared/assets/vue.svg` | shared | |
| `src/components/AppLayout.vue` | `src/shared/components/AppLayout.vue` | shared | App shell layout, used by router |
| `src/components/CommentComposer.vue` | `src/comments/components/CommentComposer.vue` | comments | |
| `src/components/CommentSidebar.vue` | `src/comments/components/CommentSidebar.vue` | comments | |
| `src/components/CommentThread.vue` | `src/comments/components/CommentThread.vue` | comments | |
| `src/components/NoteEditor.vue` | `src/notes/components/NoteEditor.vue` | notes | |
| `src/components/NoteList.vue` | `src/notes/components/NoteList.vue` | notes | |
| `src/components/NoteToolbar.vue` | `src/notes/components/NoteToolbar.vue` | notes | |
| `src/components/SidebarNav.vue` | `src/shared/components/SidebarNav.vue` | shared | Cross-domain nav, rendered by AppLayout |
| `src/components/VaultCard.vue` | `src/vaults/components/VaultCard.vue` | vaults | |
| `src/components/VaultIcon.vue` | `src/vaults/components/VaultIcon.vue` | vaults | |
| `src/components/VaultIconPicker.vue` | `src/vaults/components/VaultIconPicker.vue` | vaults | |
| `src/components/VaultSwitcher.vue` | `src/vaults/components/VaultSwitcher.vue` | vaults | |
| `src/components/.gitkeep` | deleted | — | |
| `src/composables/useAuth.ts` | `src/auth/composables/useAuth.ts` | auth | |
| `src/composables/useComments.ts` | `src/comments/composables/useComments.ts` | comments | |
| `src/composables/useTeams.ts` | `src/teams/composables/useTeams.ts` | teams | |
| `src/composables/.gitkeep` | deleted | — | |
| `src/router/index.ts` | `src/shared/router/index.ts` | shared | References all domain views |
| `src/router/.gitkeep` | deleted | — | |
| `src/stores/editor.ts` | `src/notes/stores/editor.ts` | notes | Editor state is note-domain only |
| `src/stores/notes.ts` | `src/notes/stores/notes.ts` | notes | |
| `src/stores/vaults.ts` | `src/vaults/stores/vaults.ts` | vaults | |
| `src/types/index.ts` | `src/shared/types/index.ts` | shared | All cross-domain TypeScript types |
| `src/types/.gitkeep` | deleted | — | |
| `src/utils/time.ts` | `src/shared/utils/time.ts` | shared | Generic date/time utility |
| `src/views/HomeView.vue` | `src/vaults/views/HomeView.vue` | vaults | Home = vault overview screen |
| `src/views/LoginView.vue` | `src/auth/views/LoginView.vue` | auth | |
| `src/views/NoteView.vue` | `src/notes/views/NoteView.vue` | notes | |
| `src/views/TeamSettingsView.vue` | `src/teams/views/TeamSettingsView.vue` | teams | |
| `src/views/VaultSettingsView.vue` | `src/vaults/views/VaultSettingsView.vue` | vaults | |
| `src/views/VaultView.vue` | `src/vaults/views/VaultView.vue` | vaults | |
| `src/views/.gitkeep` | deleted | — | |
| `src/App.vue` | `src/App.vue` | root | Unchanged — entry point |
| `src/main.ts` | `src/main.ts` | root | Import path for router/theme.css must update |
| `src/vite-env.d.ts` | `src/vite-env.d.ts` | root | Unchanged |

---

## Import Changes Required

Files that need internal imports updated after the move:

### `src/main.ts`
| Old Import | New Import |
|---|---|
| `@/router` | `@/shared/router` |
| `@/assets/theme.css` | `@/shared/assets/theme.css` |

### `src/shared/router/index.ts` (was `src/router/index.ts`)
| Old Import | New Import |
|---|---|
| `@/composables/useAuth` | `@/auth/composables/useAuth` |
| `@/components/AppLayout.vue` | `@/shared/components/AppLayout.vue` |
| `@/views/HomeView.vue` | `@/vaults/views/HomeView.vue` |
| `@/views/VaultView.vue` | `@/vaults/views/VaultView.vue` |
| `@/views/NoteView.vue` | `@/notes/views/NoteView.vue` |
| `@/views/VaultSettingsView.vue` | `@/vaults/views/VaultSettingsView.vue` |
| `@/views/TeamSettingsView.vue` | `@/teams/views/TeamSettingsView.vue` |
| `@/views/LoginView.vue` | `@/auth/views/LoginView.vue` |

### API modules — `client.ts` import
Each domain API module imports `client.ts`. The import path must be updated:
| Old Import | New Import |
|---|---|
| `@/api/client` | `@/shared/api/client` |

Affected files: `auth/api/auth.ts`, `notes/api/notes.ts`, `vaults/api/vaults.ts`, `comments/api/comments.ts`, `teams/api/teams.ts`

### `types/index.ts` imports in domain files
Any file importing `@/types` must update to `@/shared/types`.

### `utils/time.ts` imports
Any file importing `@/utils/time` must update to `@/shared/utils/time`.

### `VaultIcon.vue` / `VaultIconPicker.vue` — asset paths
Dynamic import paths that reference `assets/icons/vaults/` must update to `@/vaults/assets/icons/` or equivalent relative paths.

---

## Domain Inventory (final)

| Domain | views | components | stores | composables | api | assets |
|--------|-------|------------|--------|-------------|-----|--------|
| `vaults/` | HomeView, VaultView, VaultSettingsView | VaultCard, VaultIcon, VaultIconPicker, VaultSwitcher | vaults | — | vaults | icons/vaults/ |
| `notes/` | NoteView | NoteEditor, NoteList, NoteToolbar | notes, editor | — | notes | — |
| `comments/` | — | CommentComposer, CommentSidebar, CommentThread | — | useComments | comments | — |
| `auth/` | LoginView | — | — | useAuth | auth | — |
| `teams/` | TeamSettingsView | — | — | useTeams | teams | — |
| `shared/` | — | AppLayout, SidebarNav | — | — | client | icons/, theme.css, vue.svg |

**Root** (`src/`): `main.ts`, `App.vue`, `vite-env.d.ts`
**Shared non-domain**: `shared/router/index.ts`, `shared/types/index.ts`, `shared/utils/time.ts`
