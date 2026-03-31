# Frontend (`frontend/`)

## Purpose

A Vue 3 SPA that lets users browse, create, and edit Markdown notes. It talks to the Axum backend API, adapts its auth UI to the server's `AUTH_MODE`, and uses `NyxEditor` from `nyx-kit` for Markdown editing and line-based discussion annotations.

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
      VaultCard.vue          # single vault card (title, slug, description, oversized icon, link)
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
      NoteCard.vue           # single note card (title, distilled description, metadata, link)
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
- Acts as both first-run setup and remote-profile sign-in surface
- First-run setup offers:
  - `Local` → create the singleton local workspace profile and enter the app without login
  - `Server` → choose `Set up a new server` or `Connect to an existing server`
- `Set up a new server` shows guided deployment/setup instructions but does not provision the server directly
- `Connect to an existing server` collects server URL, username, and password, probes `GET /api/auth/mode`, then signs in only when the selected remote server supports `secret_key`
- Remote `oidc` servers are shown as unsupported in this feature's multi-profile flow
- On success: redirects to `/`

### `HomeView`

- If the user has exactly **one vault**: redirects immediately to `/vaults/:vault_id` (replaces history entry)
- If the user has **more than one vault**: renders a `NyxGrid` overview in `grid` mode containing `VaultCard` links; each card navigates to `/vaults/:vault_id` via an internal `RouterLink` anchor that preserves standard browser link affordances
- `VaultCard` aligns title, slug, and optional description to the top-left and places the decorative icon as a large element in the bottom-right, overflowing the card only slightly on both edges
- Provides a "New Vault" inline form (slug + name + optional description) that uses the same card family while preserving form semantics; it calls `useVaultStore().create()` and redirects to the new vault

### `VaultView` (`/vaults/:vault_id`)

- Fetches notes for the vault from `GET /api/vaults/:vault_id/notes`
- **Notes present**: renders a `NyxGrid` overview in `masonry` mode containing `NoteCard` links (sorted by `updated_at` desc); each note card navigates to `/vaults/:vault_id/notes/:id` via an internal `RouterLink` anchor that preserves standard browser link affordances
- `NoteCard` shows the note title, a distilled description generated from the first actual paragraph of saved content, and supporting metadata such as tags and update time
- **No notes**: renders a getting-started prompt with a "New Note" CTA that creates a blank note and navigates to the editor
- Header shows vault name and a persistent "New Note" action button

## Browse Card Family

The frontend uses a shared browse-card family for browse-and-select surfaces only.

- In scope: vault tiles on the home dashboard, the inline create-vault card, and note tiles in the vault notes masonry view
- Out of scope: `VaultSwitcher`, empty-state containers, comment threads, settings panels, modals, and navigation chrome
- `VaultCard` and `NoteCard` are standalone components; do not introduce a shared `BrowseCardSurface` abstraction for this feature
- Both `VaultCard` and `NoteCard` wrap their rendered card content in an internal `RouterLink` anchor so users retain standard link behavior such as open-in-new-tab and copy-link
- `NyxCard` remains the visual shell for both card components, but the anchor is the user-facing interactive surface
- Overview layouts use `NyxGrid`: `HomeView` uses `grid` mode for vault cards and `VaultView` uses `masonry` mode for note cards

### `NoteView` (`/vaults/:vault_id/notes/:id`)

- Center panel: `NoteEditor` — TipTap editor for the selected note (read-only if the user has `comment` access)
- Right panel: `CommentSidebar` — comment threads, aligned to their annotated text
- Toolbar: save button, delete button, tags input, category selector, permission selector (note author only)
- Left sidebar (VaultSwitcher, SidebarNav, NoteList) is owned by `AppLayout`, not this view

### `VaultSettingsView` (`/vaults/:vault_id/settings`)

- Rename vault
- Edit or clear the vault description
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

TipTap, the Markdown extension, annotation rendering, and all editor internals live in nyx-kit. This keeps the notes-core frontend clean of editor implementation details.

```vue
<!-- components/NoteEditor.vue -->
<NyxEditor
  v-model="draftContent"
  :editable="canEdit"
  :annotations="annotations"
  @annotation:create="onAnnotationCreate"
  @annotation:focus="onAnnotationFocus"
  @annotation:blur="onAnnotationBlur"
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

The frontend boots from the active workspace profile, not from one global server configuration.

- A local profile skips login entirely
- A remote profile first calls `GET /api/auth/mode` for that profile's server URL
- If the remote server reports `secret_key`, the client shows the username/password form and stores the resulting token only for that profile
- If the remote server reports `oidc`, the client keeps the profile saved but marks it unsupported for this feature's remote flow
- Switching profiles clears in-memory auth, vault, note, comment, and team state before loading the newly selected profile

```ts
// auth/composables/useAuth.ts
const activeProfile = ref<WorkspaceProfile | null>(null)
const authMode = ref<'local' | 'secret_key' | 'oidc' | null>(null)
const idToken = ref<string | null>(null)

async function bootstrapActiveProfile() {
  if (activeProfile.value?.type === 'local') {
    authMode.value = 'local'
    idToken.value = null
    return
  }

  const { mode } = await fetchAuthModeForProfile(activeProfile.value)
  authMode.value = mode
}
```

The token is attached only to requests made through the currently active remote profile:

```ts
await apiForActiveProfile(`/api/vaults/${vaultId}/notes/${id}`)
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

A navigation guard redirects unauthenticated remote profiles to `/login`. `meta: { requiresAuth: true }` is set on the `AppLayout` parent; child routes inherit it. Local profiles bypass the login guard.

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

## Line-Based Comments

Users can select non-empty text within a rendered line in a note and attach a discussion thread to it. The selected text is the canonical anchor, while the containing line is shown as context in the sidebar. Comments are asynchronous annotations; they do not modify the note body.

### Data Model

```ts
interface CommentAnchor {
  text: string
  prefix: string
  suffix: string
  range_from: number
  range_to: number
  attachment: 'attached' | 'detached'
  line_preview: string
}

interface Comment {
  id: string
  note_id: string
  author_id: string
  author_name: string
  body: string
  anchor: CommentAnchor
  resolved: boolean
  created_at: string
  updated_at: string
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

Comments are anchored by a structured annotation anchor:

- `anchor.text` stores the exact selected text and is the canonical anchor for new comments
- `anchor.prefix` and `anchor.suffix` provide nearby context for reattachment after note edits
- `anchor.range_from` and `anchor.range_to` store the last known rendered document range
- `anchor.line_preview` stores the containing line shown in the sidebar
- `anchor.attachment` indicates whether the comment is currently attached or detached

On load, the frontend maps visible comments into `NyxAnnotation[]` and passes them to `NyxEditor`. Nyx-kit handles highlight rendering, attachment styling, and annotation focus state.

Legacy comments without a reliable structured anchor are retained in storage but hidden from the default line-discussion UI.

### User Interaction Flow

1. **Creating a comment**: User selects text inside the editor → `NyxEditor` emits `annotation:create` with the selection anchor → `CommentComposer` opens → on submit, `POST /api/vaults/:vault_id/notes/:id/comments`
2. **Viewing comments**: `CommentSidebar` renders open and resolved threads using `anchor.line_preview` for context while `NyxEditor` renders matching visible annotations
3. **Replying**: Inside `CommentThread`, a reply input is available for visible unresolved threads
4. **Resolving**: A "Resolve" button on each thread calls `PATCH /api/vaults/:vault_id/notes/:id/comments/:commentId` with `{ resolved: true }` and the annotation styling updates to the resolved state
5. **Detached comments**: If a stored anchor can no longer be matched confidently, the thread remains visible in a detached state with its saved line preview
6. **Legacy comments**: Older note-level comments without a reliable anchor stay stored but do not appear in the default line-discussion experience
7. **Focusing annotations**: Clicking an annotation in the editor opens the comment sidebar automatically if it is closed and highlights the matching thread
8. **Resolved annotation visibility**: Open unresolved annotations render in the editor by default; resolved annotations render only while the Resolved tab is active in the sidebar

### `CommentSidebar.vue`

- Fetches `GET /api/vaults/:vault_id/notes/:id/comments` on note load
- Renders `CommentThread` for each visible comment, ordered by attached note position first and detached threads after that
- Uses `anchor.line_preview` as the sidebar context for the selected-text anchor
- Synchronizes active thread focus with `NyxEditor` annotation focus/blur events
- Opens automatically when an annotation is focused while the sidebar is closed
- Toggle to show/hide resolved threads

### Backend Impact

Comments require new API routes and storage. See [../architecture/backend-api.md](../architecture/backend-api.md) for route definitions. Storage options:

- **Sidecar file** (preferred for v1): `<slug>.comments.json` alongside `<slug>.md` — keeps the note body clean and the filesystem as source of truth while storing structured anchors separately from Markdown
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

**Layer 6 dependency**: requires `NyxEditor` and annotation APIs to be implemented and published in `nyx-kit`. ✅ Satisfied by `nyx-kit` 2.0.6.

**Layer 7 dependency**: requires new backend routes and `FsStorage` sidecar support for `.comments.json` files. ✅ Implemented alongside this layer.

---

## Non-Goals

- No server-side rendering (SSR) — this is a pure SPA
- No offline/sync (future consideration)
- No real-time collaboration — comments are async, not live-cursor multiplayer
