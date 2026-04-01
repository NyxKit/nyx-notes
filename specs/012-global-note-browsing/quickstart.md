# Quickstart: Global Note Browsing

## 1. Update docs first

Update these docs before code changes:

- `/home/arnedecant/Projects/nyxkit/nyx-notes-core/docs/interface/frontend.md`
- `/home/arnedecant/Projects/nyxkit/nyx-notes-core/docs/testing/README.md`
- `/home/arnedecant/Projects/nyxkit/nyx-notes-core/README.md` if user-facing navigation behavior changes are summarized there

Document these changes:

- search is global across accessible profiles and vaults
- favorites are global across accessible profiles and vaults
- note cards in browse grids show server and vault context
- sidebar no longer offers a context-free `New Note` action
- global browse routes live under `/notes/*`

## 2. Implement route and navigation changes

- Add dedicated global browse routes under the authenticated shell
- Redirect any vault-scoped favorites route to the global favorites route
- Remove the sidebar-wide `New Note` CTA
- Point favorites navigation at the global route

## 3. Add notes-domain browse state

- Create a dedicated notes browsing store or composable for:
  - global search query and results
  - persisted favorite references
  - aggregated browse-card models
  - global loading and error states

## 4. Reuse the note card across browse surfaces

- Extend the card input model so it can render server and vault labels
- Use the same card family in:
  - global search
  - global favorites
  - single-vault note grid

## 5. Verify behavior

Run frontend checks after implementation:

- `pnpm --dir /home/arnedecant/Projects/nyxkit/nyx-notes-core/frontend test`
- `pnpm --dir /home/arnedecant/Projects/nyxkit/nyx-notes-core/frontend lint`

Manual verification focus:

- same search query returns the same eligible results regardless of current vault page
- favorites view shows the same result set regardless of current vault page
- note cards in all three browse surfaces show server and vault labels
- users can only create notes from within a concrete vault context
