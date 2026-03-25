# Frontend (`frontend/`)

## Purpose

A Vue 3 SPA that lets users browse, create, and edit Markdown notes. It talks to the Axum backend API, adapts its auth UI to the server's `AUTH_MODE`, and uses TipTap as the rich Markdown editor.

## Stack

| Concern | Technology |
|---|---|
| Framework | Vue 3 (Composition API) |
| Component library | `nyx-kit` |
| Editor | `NyxEditor` from `nyx-kit` (wraps TipTap internally) |
| Auth | Mode-adaptive (none / form / OIDC redirect) |
| HTTP client | `fetch` / `ofetch` |
| Build | Vite |
| Routing | Vue Router |

## Application Layout

`src/` is organised into domain folders. Each domain owns its views, components, stores,
composables, and API module. Cross-domain code lives in `shared/`. Entry points stay at
the `src/` root.

Every importable subdirectory exposes an `index.ts` barrel file. Internal code imports from the
directory path (`@/notes/components`, `@/shared/api`, `@/vaults/views`) rather than directly from
individual files.

```
frontend/src/
  main.ts
  App.vue
  vite-env.d.ts

  vaults/
    index.ts                 # re-exports the vault barrels below
    api/vaults.ts            # vault CRUD API calls
    api/index.ts             # exports vault API functions
    assets/icons/            # 20 solid vault-themed SVGs (book.svg, briefcase.svg, …)
    assets/icons/index.ts    # exports vault icon raw strings
    components/
      VaultCard.vue          # single vault card (icon + name + actions)
      VaultIcon.vue          # renders a vault icon SVG by slug prop; falls back to folder
      VaultIconPicker.vue    # 5×4 grid of 20 icon options; emits select with chosen slug
      VaultSwitcher.vue      # dropdown: switch between personal and team vaults
      index.ts               # exports vault components
    stores/vaults.ts         # useVaultStore — vault list, activeVault, CRUD, $reset()
    stores/index.ts          # exports vault stores
    views/
      HomeView.vue           # multi-vault dashboard; redirects to vault if only one exists
      VaultView.vue          # notes masonry for a single vault; getting-started if empty
      VaultSettingsView.vue  # rename vault, change icon, change permission, delete vault
      index.ts               # exports vault views

  notes/
    index.ts                 # re-exports the notes barrels below
    api/notes.ts             # note CRUD API calls
    api/index.ts             # exports note API functions
    components/
      NoteEditor.vue         # thin wrapper around <NyxEditor> from nyx-kit
      NoteList.vue           # sidebar: list of notes in the active vault
      NoteToolbar.vue        # save, delete, tags, permission selector
      index.ts               # exports note components
    stores/
      notes.ts               # useNotesStore — vault-keyed notes cache, $reset()
      editor.ts              # useEditorStore — editor mode and source view toggle
      index.ts               # exports note stores
    views/
      NoteView.vue           # editor for a specific note (:vault_id/:id)
      index.ts               # exports note views

  comments/
    index.ts                 # re-exports the comments barrels below
    api/comments.ts          # comment CRUD API calls
    api/index.ts             # exports comment API functions
    components/
      CommentComposer.vue    # new comment / reply input
      CommentSidebar.vue     # comment thread panel (right of editor)
      CommentThread.vue      # single thread: anchor quote + replies
      index.ts               # exports comment components
    composables/
      useComments.ts         # comment CRUD and anchor resolution
      index.ts               # exports comment composables

  auth/
    index.ts                 # re-exports auth barrels below
    api/auth.ts              # auth mode discovery and login
    api/index.ts             # exports auth API functions
    composables/
      useAuth.ts             # auth state and token management
      index.ts               # exports auth composables
    views/
      LoginView.vue          # login UI (adapts to auth mode)
      index.ts               # exports auth views

  teams/
    index.ts                 # re-exports the team barrels below
    api/teams.ts             # team CRUD API calls
    api/index.ts             # exports team API functions
    composables/
      useTeams.ts            # team management (members, roles, team vaults)
      index.ts               # exports team composables
    views/
      TeamSettingsView.vue   # manage members, roles, and team vaults
      index.ts               # exports team views

  shared/
    index.ts                 # re-exports shared barrels below
    api/client.ts            # base ofetch HTTP client (used by all domain API modules)
    api/index.ts             # exports shared API utilities
    assets/
      index.ts               # re-exports shared assets and loads global theme CSS
      icons/                 # 20 generic outline SVGs (shared across UI)
      icons/index.ts         # exports shared icon raw strings
      theme.css              # global design tokens
    components/
      AppLayout.vue          # persistent authenticated shell: sidebar + <RouterView />
      SidebarNav.vue         # primary navigation links in the sidebar
      index.ts               # exports shared components
    router/index.ts          # router module and folder barrel for shared routing
    types/index.ts           # TypeScript interfaces mirroring Rust domain types
    utils/time.ts            # date/time formatting utilities
    utils/index.ts           # exports shared utilities
```

### Adding new files

- New file exclusive to one domain → place inside that domain's appropriate subfolder
- New file used by ≥ 2 domains → place in `shared/`
- Supported subdirectory types per domain: `api/`, `assets/`, `classes/`, `components/`,
  `composables/`, `stores/`, `types/`, `utils/`, `views/` — create only those with actual files
- Every importable subdirectory gets an `index.ts` barrel that re-exports all modules in that
  folder, and imports target the folder path rather than a file path

## Views

### `LoginView`

- Rendered at `/login`
- UI adapts to the server's `AUTH_MODE`:
  - `secret_key`: username + password form, submits to `POST /api/auth/login`
  - `oidc`: redirects to the OIDC provider's login page; exchanges the auth code for a token on return
  - `local`: never rendered (login is skipped entirely)
- On success: redirects to `/`

### `HomeView`

- If the user has exactly **one vault**: redirects immediately to `/vaults/:vault_id` (replaces history entry)
- If the user has **more than one vault**: renders a masonry grid of vault cards; each card navigates to `/vaults/:vault_id`
- Provides a "New Vault" inline form (slug + name) that calls `useVaultStore().create()` and redirects to the new vault

### `VaultView` (`/vaults/:vault_id`)

- Fetches notes for the vault from `GET /api/vaults/:vault_id/notes`
- **Notes present**: renders a masonry grid of note cards (sorted by `updated_at` desc); clicking a card navigates to `/vaults/:vault_id/notes/:id`
- **No notes**: renders a getting-started prompt with a "New Note" CTA that creates a blank note and navigates to the editor
- Header shows vault name and a persistent "New Note" action button

### `NoteView` (`/vaults/:vault_id/notes/:id`)

- Center panel: `NoteEditor` — TipTap editor for the selected note (read-only if the user has `comment` access)
- Right panel: `CommentSidebar` — comment threads, aligned to their annotated text
- Toolbar: save button, delete button, tags input, category selector, permission selector (note author only)
- Left sidebar (VaultSwitcher, SidebarNav, NoteList) is owned by `AppLayout`, not this view

### `VaultSettingsView` (`/vaults/:vault_id/settings`)

- Rename vault
- For team vaults: change vault permission level (team owner or admin only)
- Delete vault (must be empty; shows note count if not)

### `TeamSettingsView` (`/teams/:team_id/settings`)

- Rename team
- Member list: display name, role, remove button
- Add member by email (looks up user by email via the API)
- Change member role (owner only can promote/demote admins)
- List of team vaults with a link to each vault's settings

## Editor (`NoteEditor.vue`)

The editor is provided by `NyxEditor` from `nyx-kit`. `NoteEditor.vue` is a thin wrapper that passes props and relays events — it does not configure TipTap directly.

TipTap, the Markdown extension, `CommentMark`, and all editor internals live in nyx-kit. This keeps the notes-core frontend clean of editor implementation details.

```vue
<!-- components/NoteEditor.vue -->
<NyxEditor
  v-model="draftContent"
  :editable="canEdit"
  :comments="comments"
  @comment:create="onCommentCreate"
/>
```

The editor works in **Markdown storage mode**: content passed in and emitted out is always raw Markdown. The rich ProseMirror document tree is internal to NyxEditor.

## Vault Switcher (`VaultSwitcher.vue`)

Rendered at the top of the left panel. Lets the user switch between vaults without leaving the current view.

- Groups vaults: **Personal** (user's own vaults) and **Teams** (one group per team, listing that team's vaults)
- Shows the active vault name with a dropdown arrow
- "New vault" option at the bottom of the personal group
- Navigates to `/vaults/:vault_id/notes` on selection, opening the most recently edited note in the vault

## Note List (`NoteList.vue`)

- Fetches `GET /api/vaults/:vault_id/notes` when the active vault changes
- Displays `NoteMeta[]`: title, category, `updated_at` relative time, tag chips
- Shows a permission badge on each note: no badge for `restricted`, a comment icon for `comment`, a pencil icon for `edit`
- Clicking a note navigates to `/vaults/:vault_id/notes/:id`
- "New Note" button at the top creates a blank note in the current vault
- Filter/search input (client-side, no server round-trip for basic use)

## Auth Flow

On startup, the frontend calls `GET /api/auth/mode` to discover which auth mode the server is running, then renders the appropriate login UI (or skips it for `local` mode).

```ts
// auth/composables/useAuth.ts
const authMode = ref<'local' | 'secret_key' | 'oidc' | null>(null)
const idToken = ref<string | null>(null)

onMounted(async () => {
  const { mode, oidc_issuer } = await fetch('/api/auth/mode').then(r => r.json())
  authMode.value = mode
  if (mode === 'local') return  // no login needed
  if (mode === 'oidc') initOidcClient(oidc_issuer)
  // secret_key: token is set on login form submit
})
```

The token is attached to every API request:

```ts
await fetch(`/api/vaults/${vaultId}/notes/${id}`, {
  headers: { Authorization: `Bearer ${idToken.value}` },
})
```

## Routing

All authenticated routes are nested under the `AppLayout` parent route. `AppLayout` mounts once per session and persists across child route changes — it is never unmounted when navigating between authenticated views.

```
/                          → AppLayout      (meta: { requiresAuth: true })
  /                        → HomeView
  /vaults/:vault_id        → VaultView
  /vaults/:vault_id/notes/:id? → NoteView
  /vaults/:vault_id/settings  → VaultSettingsView
  /teams/:team_id/settings    → TeamSettingsView
/login                     → LoginView
```

| Path | View | Guard |
|---|---|---|
| `/` | `HomeView` (child of `AppLayout`) | Auth required (inherited) |
| `/vaults/:vault_id` | `VaultView` (child of `AppLayout`) | Auth required (inherited) |
| `/vaults/:vault_id/notes/:id` | `NoteView` (child of `AppLayout`) | Auth required (inherited) |
| `/vaults/:vault_id/settings` | `VaultSettingsView` (child of `AppLayout`) | Auth required (inherited) |
| `/teams/:team_id/settings` | `TeamSettingsView` (child of `AppLayout`) | Auth required (inherited) |
| `/login` | `LoginView` | Redirect to `/` if already authed |

A navigation guard redirects unauthenticated users to `/login`. `meta: { requiresAuth: true }` is set on the `AppLayout` parent; child routes inherit it.

## Component Library (`nyx-kit`)

Use `nyx-kit` components for all UI primitives. **No native `<button>`, `<input>`, `<textarea>`, or badge `<span>` elements are permitted in `.vue` files inside `frontend/src/`.** Use the corresponding nyx-kit equivalent in every case.

| Primitive | nyx-kit replacement |
|-----------|---------------------|
| `<button>` | `<NyxButton>` |
| `<input>` | `<NyxInput>` |
| `<textarea>` | `<NyxTextarea>` |
| tag/badge `<span>` | `<NyxBadge>` |
| form container | `<NyxForm>` + `<NyxFormField>` |
| tab switcher | `<NyxTabs>` |

```ts
import { NyxButton, NyxInput, NyxTextarea, NyxBadge, NyxForm, NyxFormField, NyxTabs } from 'nyx-kit/components'
import { NyxVariant, NyxTheme, NyxSize, NyxShape } from 'nyx-kit/types'
```

- Layout (sidebar + main panel) → nyx-kit layout components if available, otherwise custom
- Do **not** introduce a second component library (no Vuetify, no PrimeVue)

## Build & Deployment

The built `dist/` can be:

1. **Served by the Rust server** (zero extra infrastructure): `ServeDir::new("dist")` in Axum
2. **Deployed separately** to Netlify / Cloudflare Pages with `/api/*` proxied to the Rust server

## Environment Variables

| Variable | Description |
|---|---|
| `VITE_API_BASE_URL` | Backend base URL (defaults to `/` for same-origin) |

## Note Permissions

### Permission Levels

| Value | Who can view | Who can comment | Who can edit |
|---|---|---|---|
| `restricted` | Owner only | Owner only | Owner only |
| `comment` | Any authenticated user | Any authenticated user | Owner only |
| `edit` | Any authenticated user | Any authenticated user | Any authenticated user |

The owner can always do everything. Permissions apply to all other authenticated users (no per-user allowlists in v1).

### Permission Selector (owner only)

Rendered in `NoteToolbar` as a segmented control or dropdown. Only visible when the current user is the note owner.

```
[ Restricted ]  [ Comment ]  [ Edit ]
```

On change: calls `PATCH /api/vaults/:vault_id/notes/:id/permission`. No full save required.

### Read-Only Editor State

When the current user has `comment` access (not the owner, note is `comment` permission):
- TipTap editor is initialized with `editable: false`
- Save and delete buttons are hidden
- The permission selector is hidden
- The comment sidebar remains fully functional

```ts
const isOwner = computed(() => note.value?.meta.author_id === currentUser.value?.uid)
const canEdit = computed(() =>
  isOwner.value || note.value?.meta.permission === 'edit'
)

const editor = useEditor({
  editable: canEdit.value,
  // ...
})

watch(canEdit, (val) => editor.value?.setEditable(val))
```

### Shared Notes in the Note List

Notes shared with the current user (not owned) appear in a separate "Shared with me" section in `NoteList`, or are visually distinguished with the sharer's display name below the title.

## Inline Comments

Users can select any text in a note and attach a comment thread to it — similar to Google Docs. Comments are asynchronous annotations; they do not modify the note body.

### Data Model

```ts
interface Comment {
  id: string
  note_id: string
  author_id: string
  author_name: string
  body: string           // plain text (Markdown support optional later)
  quoted_text: string    // the exact text selection the comment is anchored to
  resolved: boolean
  created_at: string     // ISO 8601
  replies: CommentReply[]
}

interface CommentReply {
  id: string
  author_id: string
  author_name: string
  body: string
  created_at: string
}
```

### Anchor Strategy

Comments are anchored by **quoted text** — the exact string the user selected when creating the comment. This approach:

- Survives Markdown serialization/deserialization round-trips (no fragile position offsets)
- Degrades gracefully: if the anchored text is later edited or deleted, the comment becomes "orphaned" and is shown at the top of the comment sidebar with its original quote
- Requires no special Markdown encoding — the note body stays clean

On load, the frontend scans the document for each comment's `quoted_text` and applies a `CommentMark` decoration at the first match.

### `CommentMark` Extension

The `CommentMark` TipTap extension is implemented in `nyx-kit` as part of `NyxEditor`. It highlights text associated with a comment thread using a `span[data-comment-id]` decoration.

The mark is **not serialized to Markdown**. It is applied as an in-memory ProseMirror decoration each time a note loads, based on `quoted_text` matching. The note body on disk stays clean.

### User Interaction Flow

1. **Creating a comment**: User selects text → a "Add comment" tooltip/button appears → `CommentComposer` opens inline → on submit, `POST /api/vaults/:vault_id/notes/:id/comments` → mark applied to matched text
2. **Viewing comments**: Highlighted text shows a colored underline. `CommentSidebar` renders all threads vertically aligned to their marked text (using the mark's DOM position)
3. **Replying**: Inside `CommentThread`, a reply input is always visible at the bottom of the thread
4. **Resolving**: A "Resolve" button on each thread calls `PATCH /api/vaults/:vault_id/notes/:id/comments/:commentId` with `{ resolved: true }` → mark styling changes to muted/strikethrough
5. **Orphaned comments**: If `quoted_text` is not found in the document, the thread renders at the top of the sidebar with a "Text no longer found" indicator and the original quote displayed

### `CommentSidebar.vue`

- Fetches `GET /api/vaults/:vault_id/notes/:id/comments` on note load
- Renders `CommentThread` for each comment, sorted by document position of the anchor (unresolved first, then resolved)
- Vertically aligns each thread to the top of its highlighted text in the editor (uses the DOM rect of the `CommentMark` span)
- Toggle to show/hide resolved threads

### Backend Impact

Comments require new API routes and storage. See [../architecture/backend-api.md](../architecture/backend-api.md) for route definitions. Storage options:

- **Sidecar file** (preferred for v1): `<slug>.comments.json` alongside `<slug>.md` — keeps the note body clean and the filesystem as source of truth
- **Database** (future): migrate to a DB table once multi-user/sharing is needed

### Keyboard Shortcut

| Action | Shortcut |
|---|---|
| Add comment on selection | `Cmd/Ctrl + Alt + M` |

## Implementation Layers

The frontend is built incrementally. Each layer produces reviewable, running code before the next begins.

| # | Layer | Goal | Key files |
|---|---|---|---|
| 1 | Scaffold | Buildable Vite + Vue 3 + TS project, bare `App.vue` | `package.json`, `vite.config.ts`, `main.ts`, `App.vue` |
| 2 | Types + API client | TS interfaces mirroring Rust domain types; `ofetch` client with auth header injection | `shared/types/index.ts`, `shared/api/client.ts` |
| 3 | Auth composable + router | Mode discovery, token management, login view, route guards | `auth/composables/useAuth.ts`, `shared/router/index.ts`, `auth/views/LoginView.vue` |
| 4 | Vault + note stores | CRUD state and operations consumed by views | `vaults/stores/vaults.ts`, `notes/stores/notes.ts` |
| 5 | App shell + NoteView | Layout, vault switcher, note list, first navigable view | `App.vue`, `vaults/components/VaultSwitcher.vue`, `notes/components/NoteList.vue`, `notes/views/NoteView.vue` |
| 6 | Editor | `NoteEditor.vue` wrapping `NyxEditor`, toolbar with permission selector | `notes/components/NoteEditor.vue`, `notes/components/NoteToolbar.vue` |
| 7 | Comments | Comment composable, sidebar, thread, composer | `comments/composables/useComments.ts`, `comments/components/CommentSidebar.vue`, `comments/components/CommentThread.vue`, `comments/components/CommentComposer.vue` |
| 8 | Settings views ✅ | Vault and team management UI | `vaults/views/VaultSettingsView.vue`, `teams/views/TeamSettingsView.vue`, `teams/composables/useTeams.ts` |

**Layer 5 dependency**: requires `nyx-kit` components (`NyxButton`, `NyxInput`, etc.).

**Layer 6 dependency**: requires `NyxEditor` to be implemented and published in `nyx-kit`. ✅ Satisfied by `nyx-kit` 1.3.3.

**Layer 7 dependency**: requires new backend routes and `FsStorage` sidecar support for `.comments.json` files. ✅ Implemented alongside this layer.

---

## Non-Goals

- No server-side rendering (SSR) — this is a pure SPA
- No offline/sync (future consideration)
- No real-time collaboration — comments are async, not live-cursor multiplayer
