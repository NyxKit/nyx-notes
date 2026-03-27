# Quickstart: Unify Card Surfaces

## 1. Update docs first

- Update `/home/arnedecant/Projects/nyxkit/nyx-notes-core/docs/interface/frontend.md` to define the shared browse-card family and name the in-scope surfaces.
- If reusable visual rules are introduced beyond what `DESIGN.md` already states, update `/home/arnedecant/Projects/nyxkit/nyx-notes-core/DESIGN.md` in the same pass.
- If either doc conflicts with current implementation assumptions, record the divergence before editing code.

## 2. Implement shared browse-card presentation

- Refactor `/home/arnedecant/Projects/nyxkit/nyx-notes-core/frontend/src/vaults/components/VaultCard.vue` so it uses `NyxCard`, wraps internally in a `RouterLink`, and renders title, slug, description, and oversized bottom-right icon.
- Add `/home/arnedecant/Projects/nyxkit/nyx-notes-core/frontend/src/notes/components/NoteCard.vue` as a dedicated note-card component that uses `NyxCard`, wraps internally in a `RouterLink`, and renders title, distilled description, and metadata.
- Refactor `/home/arnedecant/Projects/nyxkit/nyx-notes-core/frontend/src/vaults/views/HomeView.vue` so the inline create-vault surface uses the same card family and supports editing the vault description.
- Refactor `/home/arnedecant/Projects/nyxkit/nyx-notes-core/frontend/src/vaults/views/VaultView.vue` so it uses `NoteCard` instead of inline note-card markup.
- Do not restyle `/home/arnedecant/Projects/nyxkit/nyx-notes-core/frontend/src/vaults/components/VaultSwitcher.vue`, empty-state cards, comment threads, settings/modals, or navigation chrome as part of this feature.

## 3. Preserve interaction semantics

- Keep vault and note cards fully operable with pointer and keyboard.
- Preserve standard anchor behavior so users can open card destinations in new tabs and use the default link context menu.
- Keep the create-vault card as a form-hosting surface, not a misleading single-action tile.
- Distill note descriptions from the first actual Markdown paragraph on every save; do not prompt users to author note descriptions manually.

## 4. Verify visually and behaviorally

- Confirm long vault names, slugs, and note titles remain readable on narrow and wide layouts.
- Confirm loading, populated, empty-adjacent, hover, focus, and pressed states remain consistent.
- Confirm vault cards still open vaults and note cards still open notes with no route regression.

## 5. Run validation

From `/home/arnedecant/Projects/nyxkit/nyx-notes-core/frontend`:

```sh
npm run lint
npm run build
```

Add or update automated coverage for:

- keyboard and pointer activation of vault and note cards
- full-surface activation behavior when clicking inner content
- responsive truncation of long labels
- visual/state rendering for shared browse-card members
