# Frontend (`frontend/`)

## Purpose

A Vue 3 SPA that lets users browse, create, and edit Markdown notes. It talks to the Axum backend API, authenticates via Firebase, and uses TipTap as the rich Markdown editor.

## Stack

| Concern | Technology |
|---|---|
| Framework | Vue 3 (Composition API) |
| Component library | `@nyxkit/nyx-kit` |
| Editor | TipTap (with Markdown extensions) |
| Auth | Firebase JS SDK |
| HTTP client | `fetch` / `ofetch` |
| Build | Vite |
| Routing | Vue Router |

## Application Layout

```
frontend/
  src/
    components/
      VaultSwitcher.vue      # dropdown: switch between personal and team vaults
      NoteList.vue           # sidebar: list of notes in the active vault
      NoteEditor.vue         # TipTap editor pane
      NoteToolbar.vue        # save, delete, tags, permission selector
      CommentSidebar.vue     # comment thread panel (right of editor)
      CommentThread.vue      # single thread: anchor quote + replies
      CommentComposer.vue    # new comment / reply input
    views/
      HomeView.vue           # redirect to last note or empty state
      NoteView.vue           # editor for a specific note (:vault_id/:id)
      LoginView.vue          # Firebase login
      VaultSettingsView.vue  # rename vault, change permission (team vaults), delete vault
      TeamSettingsView.vue   # manage members, roles, and team vaults
    composables/
      useVaults.ts           # vault list and active vault state
      useNotes.ts            # CRUD operations against the API (vault-scoped)
      useAuth.ts             # Firebase auth state
      useComments.ts         # comment CRUD and anchor resolution
      useTeams.ts            # team management (members, roles, team vaults)
    router/index.ts
    main.ts
```

## Views

### `LoginView`

- Rendered at `/login`
- Firebase sign-in UI (email/password or Google OAuth)
- On success: redirects to `/`

### `HomeView`

- Redirects to the most recently edited note, or shows an empty-state prompt to create the first note

### `NoteView` (`/vaults/:vault_id/notes/:id`)

- Left panel: `VaultSwitcher` at the top + `NoteList` — notes in the active vault, sorted by `updated_at`
- Center panel: `NoteEditor` — TipTap editor for the selected note (read-only if the user has `comment` access)
- Right panel: `CommentSidebar` — comment threads, aligned to their annotated text
- Toolbar: save button, delete button, tags input, category selector, permission selector (note author only)

### `VaultSettingsView` (`/vaults/:vault_id/settings`)

- Rename vault
- For team vaults: change vault permission level (team owner or admin only)
- Delete vault (must be empty; shows note count if not)

### `TeamSettingsView` (`/teams/:team_id/settings`)

- Rename team
- Member list: display name, role, remove button
- Add member by email (looks up Firebase user by email via the API)
- Change member role (owner only can promote/demote admins)
- List of team vaults with a link to each vault's settings

## Editor (`NoteEditor.vue`)

Uses TipTap with the following extensions:

- `StarterKit` (headings, bold, italic, lists, blockquote, code blocks, horizontal rule)
- `Markdown` extension (serialize/deserialize Markdown, so the backend stores `.md` files)
- `Placeholder`
- `CharacterCount` (optional)
- `CommentMark` — custom extension, see [Inline Comments](#inline-comments) below

The editor works in **Markdown storage mode**: the in-memory TipTap document is a rich prosemirror node tree, but the value saved to and loaded from the API is always raw Markdown.

```ts
// composables/useNoteEditor.ts
const editor = useEditor({
  extensions: [StarterKit, Markdown],
  content: markdownToHtml(note.content), // on load
  onUpdate: ({ editor }) => {
    draftContent.value = editor.storage.markdown.getMarkdown()
  },
})
```

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

```ts
// composables/useAuth.ts
import { getAuth, onAuthStateChanged, signInWithPopup } from 'firebase/auth'

const user = ref<FirebaseUser | null>(null)
const idToken = ref<string | null>(null)

onAuthStateChanged(getAuth(), async (firebaseUser) => {
  user.value = firebaseUser
  idToken.value = firebaseUser ? await firebaseUser.getIdToken() : null
})
```

The `idToken` is attached to every API request:

```ts
await fetch(`/api/vaults/${vaultId}/notes/${id}`, {
  headers: { Authorization: `Bearer ${idToken.value}` },
})
```

## Routing

| Path | View | Guard |
|---|---|---|
| `/` | `HomeView` | Auth required |
| `/vaults/:vault_id/notes/:id` | `NoteView` | Auth required |
| `/vaults/:vault_id/settings` | `VaultSettingsView` | Auth required |
| `/teams/:team_id/settings` | `TeamSettingsView` | Auth required |
| `/login` | `LoginView` | Redirect to `/` if already authed |

A navigation guard redirects unauthenticated users to `/login`.

## Component Library (`nyx-kit`)

Use `@nyxkit/nyx-kit` components for all UI primitives:

- Buttons, inputs, tags/chips → nyx-kit components
- Layout (sidebar + main panel) → nyx-kit layout components if available, otherwise custom
- Do **not** introduce a second component library (no Vuetify, no PrimeVue)

## Build & Deployment

The built `dist/` can be:

1. **Served by the Rust server** (zero extra infrastructure): `ServeDir::new("dist")` in Axum
2. **Deployed to Firebase Hosting** with `/api/*` proxied to the Rust server

## Environment Variables

| Variable | Description |
|---|---|
| `VITE_FIREBASE_API_KEY` | Firebase web app config |
| `VITE_FIREBASE_AUTH_DOMAIN` | Firebase web app config |
| `VITE_FIREBASE_PROJECT_ID` | Firebase web app config |
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

### TipTap `CommentMark` Extension

A custom TipTap mark that highlights text associated with a comment thread.

```ts
// extensions/CommentMark.ts
import { Mark } from '@tiptap/core'

export const CommentMark = Mark.create({
  name: 'comment',

  addAttributes() {
    return {
      commentId: { default: null },
      resolved: { default: false },
    }
  },

  parseHTML() {
    return [{ tag: 'span[data-comment-id]' }]
  },

  renderHTML({ HTMLAttributes }) {
    return ['span', {
      'data-comment-id': HTMLAttributes.commentId,
      class: HTMLAttributes.resolved ? 'comment-mark--resolved' : 'comment-mark',
    }, 0]
  },

  // Does NOT serialize to Markdown — marks are applied in-memory only
  addKeyboardShortcuts() {
    return {}
  },
})
```

The mark is **not serialized to Markdown**. It is applied as a ProseMirror decoration each time the note loads, based on `quoted_text` matching.

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

## Non-Goals

- No server-side rendering (SSR) — this is a pure SPA
- No offline/sync (future consideration)
- No real-time collaboration — comments are async, not live-cursor multiplayer
