# Research: Unify Card Surfaces

## Decision 1: Limit the shared card family to the clarified in-scope surfaces

**Decision**: The shared card family applies only to `/home/arnedecant/Projects/nyxkit/nyx-notes/app/src/vaults/components/VaultCard.vue`, the inline create-vault card in `/home/arnedecant/Projects/nyxkit/nyx-notes/app/src/vaults/views/HomeView.vue`, and note cards in `/home/arnedecant/Projects/nyxkit/nyx-notes/app/src/vaults/views/VaultView.vue`.

**Rationale**: These three surfaces were explicitly clarified in the spec, are browse-and-select peers, and can share one card family without flattening distinct interface patterns.

**Alternatives considered**:
- Include the vault switcher glass surface: rejected because `DESIGN.md` gives it its own glassmorphic navigation treatment.
- Include comment thread cards: rejected because they follow the recessed threaded-comment pattern rather than browse-and-select behavior.
- Include the empty-state welcome card: rejected because it is a call-to-action container, not a peer selection tile.

---

## Decision 2: Use NyxCard as the shared visual shell and keep link-native behavior

**Decision**: `NyxCard` defines the shared card surface, while `VaultCard` and `NoteCard` wrap their content in an internal `RouterLink` anchor so users retain standard link affordances.

**Rationale**: `NyxCard` is a presentational container whose root is an `<article>` and whose emitted click only fires on `@click.self`. Wrapping the rendered card in a `RouterLink` preserves browser-native anchor behavior while still allowing `NyxCard` to provide the visual shell.

**Alternatives considered**:
- Keep all browse cards as `NyxButton` only: rejected because the new feature explicitly asks to adopt `NyxCard` for `VaultCard` and related card surfaces.
- Make `NyxCard` itself fully clickable for navigation: rejected because its API and emitted event do not provide strong button/link semantics by default.

---

## Decision 3: Resolve the prior NyxButton-vs-NyxCard precedent by separating surface from activation

**Decision**: The earlier decision in `/home/arnedecant/Projects/nyxkit/nyx-notes/specs/002-nyx-kit-primitives/research.md` remains valid for semantics, but this feature supersedes it for presentation. The implementation should preserve accessible activation semantics while moving the visual surface onto `NyxCard`.

**Rationale**: The repo already documented a concern that browse cards are navigation actions first. That concern remains true, so the safe path is to separate visual card framing from activation behavior instead of treating this feature as permission to make presentational containers carry all interaction semantics.

**Alternatives considered**:
- Ignore the older spec and replace the root with a clickable `NyxCard`: rejected because it would silently throw away the earlier accessibility reasoning.
- Ignore the new feature request and keep the old approach everywhere: rejected because it would fail the current feature intent.

---

## Decision 4: Keep card components standalone within their domains

**Decision**: `VaultCard` remains in `/home/arnedecant/Projects/nyxkit/nyx-notes/app/src/vaults/components/`, and a dedicated `NoteCard` should live in `/home/arnedecant/Projects/nyxkit/nyx-notes/app/src/notes/components/`.

**Rationale**: The user explicitly does not want a shared `BrowseCardSurface` abstraction. Keeping each card component in its own domain preserves domain boundaries while still allowing both to follow the same documented design language.

**Alternatives considered**:
- Create a shared `BrowseCardSurface` component: rejected by product direction for this feature.
- Add a second component library dependency: rejected by constitution and unnecessary because `nyx-kit` already provides `NyxCard`.

---

## Decision 5: Update docs and design guidance before code changes

**Decision**: Implementation begins with docs updates in `/home/arnedecant/Projects/nyxkit/nyx-notes/docs/interface/frontend.md` and, if needed for reusable visual rules, `/home/arnedecant/Projects/nyxkit/nyx-notes/DESIGN.md`.

**Rationale**: The constitution and `AGENTS.md` make docs the authoritative source of truth. This feature changes a frontend design decision and shared UI behavior, so code must not move ahead of the docs.

**Alternatives considered**:
- Update code first and reconcile docs later: rejected because it violates repo rules.
- Limit updates to component code comments: rejected because component comments are not the source of truth for product behavior.

---

## Decision 6: Feature execution must add explicit frontend interaction coverage

**Decision**: Execution tasks must include automated coverage for card activation and state rendering, plus browser-level verification of the affected browse flows.

**Rationale**: This feature changes high-traffic interactive surfaces. The constitution requires frontend unit and E2E coverage, and the current repository has no existing frontend test files or test harness configuration. The feature therefore needs to budget for test setup or targeted harness expansion instead of shipping on visual inspection alone.

**Alternatives considered**:
- Rely only on lint and build: rejected because they do not validate keyboard activation, click-target behavior, or responsive card states.
- Skip automated tests because the repo currently lacks them: rejected because that would repeat an existing gap instead of closing it.

---

## Decision 7: Explicitly exclude other card-like containers from this feature

**Decision**: The VaultSwitcher, empty-state cards, comment threads, settings/modals, and navigation chrome remain out of scope and must not be restyled as part of this feature.

**Rationale**: The clarified spec now explicitly excludes these surfaces, and several of them already have distinct documented treatments in `DESIGN.md` or serve materially different interaction roles.

**Alternatives considered**:
- Expand the feature to all card-like containers: rejected because it would materially widen scope and blur multiple interaction patterns.
- Leave exclusions implicit: rejected because it would increase implementation and review ambiguity.

---

## Decision 8: Store optional descriptions for vaults and distilled descriptions for notes

**Decision**: Vaults gain an optional user-authored `description` field, while notes gain an optional distilled `description` field recomputed from the first actual Markdown paragraph on every save.

**Rationale**: Vault cards need richer summary content the user can control directly. Note cards also need summary content, but the user explicitly does not want note-description entry in the UI, so note descriptions must be derived from saved content instead.

**Alternatives considered**:
- Prompt users for a note description manually: rejected by product direction.
- Derive note descriptions on the client at render time only: rejected because the requirement says the description is distilled on every save.
