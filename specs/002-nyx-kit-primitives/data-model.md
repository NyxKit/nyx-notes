# Data Model: nyx-kit Primitives Refactor

**Phase 1 output for `002-nyx-kit-primitives`**

## No new domain entities

This feature introduces no new data types, API calls, or Pinia state. It is a pure UI component swap — props, v-model bindings, and event handlers are preserved; only the element tags change.

---

## Component Replacement Map

The following table is the authoritative mapping from native primitive → nyx-kit component for every occurrence in the codebase.

### Buttons

| File | Current | Replacement | Notes |
|------|---------|-------------|-------|
| `SidebarNav.vue` | `<button>` New Note CTA | `<NyxButton gradient>` | gradient matches existing style |
| `SidebarNav.vue` | `<RouterLink>` nav items | keep as `<RouterLink>` | not a primitive; router navigation |
| `HomeView.vue` | `<button>` New Vault | `<NyxButton>` | header action |
| `HomeView.vue` | `<button>` vault card | `<NyxButton variant="ghost">` | navigation tile |
| `HomeView.vue` | `<button>` Create (form) | `<NyxButton type="submit">` | form submit |
| `HomeView.vue` | `<button>` Cancel | `<NyxButton variant="ghost">` | dismiss form |
| `VaultView.vue` | `<button>` New Note (header) | `<NyxButton>` | header action |
| `VaultView.vue` | `<button>` note card | `<NyxButton variant="ghost">` | navigation tile |
| `VaultView.vue` | `<button>` New Note (empty state CTA) | `<NyxButton gradient>` | primary CTA |
| `NoteView.vue` | sidebar toggle `<button>` | `<NyxButton variant="ghost" shape="square">` | icon-only |
| `NoteView.vue` | source/fav/delete/comments icon btns | `<NyxButton variant="ghost" shape="square">` | icon-only; danger theme for delete |
| `NoteList.vue` | `<button>` New Note | `<NyxButton>` | list header action |
| `NoteList.vue` | note item `<button>` | `<NyxButton variant="ghost">` | list item |
| `CommentComposer.vue` | `<button>` Submit | `<NyxButton type="submit">` | form submit |
| `CommentComposer.vue` | `<button>` Cancel | `<NyxButton variant="ghost">` | dismiss |
| `CommentThread.vue` | `<button>` Resolve | `<NyxButton variant="ghost" size="sm">` | action |
| `CommentThread.vue` | `<button>` Delete | `<NyxButton variant="ghost" theme="danger" size="sm">` | destructive |
| `CommentSidebar.vue` | `<button>` Add comment | `<NyxButton>` | primary action |
| `CommentSidebar.vue` | tab `<button>`s | replaced by `NyxTabs` | see Tabs section |
| `VaultSettingsView.vue` | `<button>` Back | `<NyxButton variant="ghost">` | navigation |
| `VaultSettingsView.vue` | `<button>` Delete confirmation | `<NyxButton theme="danger">` | destructive |
| `TeamSettingsView.vue` | `<button>` Back | `<NyxButton variant="ghost">` | navigation |
| `TeamSettingsView.vue` | `<button>` Remove member | `<NyxButton variant="ghost" theme="danger" size="sm">` | destructive |
| `TeamSettingsView.vue` | `<button>` Add member | `<NyxButton type="submit">` | form submit |
| `TeamSettingsView.vue` | `<button>` Add vault | `<NyxButton type="submit">` | form submit |
| `TeamSettingsView.vue` | `<button>` Cancel | `<NyxButton variant="ghost">` | dismiss |
| `TeamSettingsView.vue` | `<button>` Delete vault/team | `<NyxButton theme="danger">` | destructive |

### Inputs

| File | Current | Replacement | Notes |
|------|---------|-------------|-------|
| `HomeView.vue` | `<input>` name | `<NyxInput>` | vault name field |
| `HomeView.vue` | `<input>` slug | `<NyxInput>` | vault slug field |
| `NoteList.vue` | `<input>` search | `<NyxInput type="search">` | filter input |
| `TeamSettingsView.vue` | `<input>` user ID | `<NyxInput>` | add member |
| `TeamSettingsView.vue` | `<input>` vault name/slug | `<NyxInput>` | add vault form |

### Textareas

| File | Current | Replacement | Notes |
|------|---------|-------------|-------|
| `CommentComposer.vue` | `<textarea>` | `<NyxTextarea>` | comment body |

### Badges / Tags

| File | Current | Replacement | Notes |
|------|---------|-------------|-------|
| `VaultView.vue` | `.vault__note-tag` span | `<NyxBadge theme="primary" variant="soft">` | read-only tag |
| `NoteToolbar.vue` | `.note-toolbar__tag` span | `<NyxBadge theme="primary" variant="soft" hasClose @close="removeTag">` | dismissible tag |
| `NoteList.vue` | tag spans | `<NyxBadge theme="primary" variant="soft">` | read-only tag |

### Tabs

| File | Current | Replacement | Notes |
|------|---------|-------------|-------|
| `CommentSidebar.vue` | hand-rolled tab buttons | `<NyxTabs v-model="activeTab" :tabs="['Open', 'Resolved']">` | client-side only |

### Forms

| File | Current | Replacement | Notes |
|------|---------|-------------|-------|
| `HomeView.vue` | bare `<div>` form card | `<NyxForm @submit="submitCreate">` | vault create |
| `CommentComposer.vue` | bare `<form>` or `<div>` | `<NyxForm @submit="submit">` | comment create |
| `TeamSettingsView.vue` | bare forms | `<NyxForm>` per form section | member + vault forms |

---

## CSS cleanup rule

For each replaced primitive, the corresponding scoped CSS block is removed. Only layout/structural CSS that is not covered by the nyx-kit component's own styles may remain.
