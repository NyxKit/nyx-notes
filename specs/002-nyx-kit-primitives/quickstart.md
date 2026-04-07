# Quickstart: nyx-kit Primitives Refactor

## What's Changing

| File | Change |
|------|--------|
| `app/src/components/SidebarNav.vue` | `<button>` → `NyxButton gradient` |
| `app/src/components/NoteList.vue` | `<button>`s + tag spans → `NyxButton`, `NyxBadge` |
| `app/src/components/NoteToolbar.vue` | tag spans → `NyxBadge hasClose` |
| `app/src/components/CommentComposer.vue` | `<textarea>`, `<button>`s, form → `NyxTextarea`, `NyxButton`, `NyxForm` |
| `app/src/components/CommentThread.vue` | `<button>`s → `NyxButton` (ghost/danger) |
| `app/src/components/CommentSidebar.vue` | tab buttons → `NyxTabs`; add-comment button → `NyxButton` |
| `app/src/views/HomeView.vue` | `<button>`s, `<input>`s, form div → `NyxButton`, `NyxInput`, `NyxForm` |
| `app/src/views/VaultView.vue` | `<button>`s, tag spans → `NyxButton`, `NyxBadge` |
| `app/src/views/NoteView.vue` | icon `<button>`s → `NyxButton variant="ghost" shape="square"` |
| `app/src/views/VaultSettingsView.vue` | `<button>`s → `NyxButton` (ghost, danger) |
| `app/src/views/TeamSettingsView.vue` | `<button>`s, `<input>`s, forms → `NyxButton`, `NyxInput`, `NyxForm` |
| `docs/interface/frontend.md` | Update component library section |

No changes to: `LoginView.vue` (already correct), `VaultSwitcher.vue` (already uses `NyxSelect`), `NoteEditor.vue`, `App.vue`.

## Implementation Order

1. Components first (used across multiple views): `SidebarNav`, `NoteList`, `NoteToolbar`, `CommentComposer`, `CommentThread`, `CommentSidebar`
2. Views: `HomeView`, `VaultView`, `NoteView`, `VaultSettingsView`, `TeamSettingsView`
3. CSS cleanup: remove scoped styles for replaced primitives in all touched files
4. Docs update: `docs/interface/frontend.md`

## Key Patterns

### Button (CTA gradient)
```vue
<NyxButton :gradient="true" @click="action">Label</NyxButton>
```

### Button (icon-only)
```vue
<NyxButton variant="ghost" shape="square" @click="toggle">
  <svg>...</svg>
</NyxButton>
```

### Button (danger)
```vue
<NyxButton theme="danger" @click="destroy">Delete</NyxButton>
```

### Input with form
```vue
<NyxForm @submit="submit">
  <NyxFormField label="Vault name">
    <template #default="{ id }">
      <NyxInput :id="id" v-model="name" placeholder="My vault" />
    </template>
  </NyxFormField>
</NyxForm>
```

### Badge (read-only tag)
```vue
<NyxBadge theme="primary" variant="soft">{{ tag }}</NyxBadge>
```

### Badge (dismissible)
```vue
<NyxBadge theme="primary" variant="soft" :hasClose="true" @close="removeTag(tag)">
  {{ tag }}
</NyxBadge>
```

### Tabs (comment sidebar)
```vue
<NyxTabs v-model="activeTab" :tabs="['Open', 'Resolved']">
  <template #tab-Open><!-- open threads --></template>
  <template #tab-Resolved><!-- resolved threads --></template>
</NyxTabs>
```

## Dev Setup

No new dependencies — nyx-kit is already installed.

```bash
cd frontend && pnpm dev
```

## Verification

After each file, open the app and confirm:
- Controls render and behave identically
- No console errors about unknown props
- `v-model` updates work as before
- Disabled states are respected
