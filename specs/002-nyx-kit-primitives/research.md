# Research: nyx-kit Primitives Refactor

**Phase 0 output for `002-nyx-kit-primitives`**

---

## Decision 1: NyxButton replaces all `<button>` elements

**Decision**: Use `NyxButton` for every button in the codebase. Danger actions use `theme="danger"`. Cancel/secondary actions use `variant="ghost"`. The gradient CTA buttons (currently hardcoded `linear-gradient(135deg, #cbc2e4 0%, #49435f 100%)`) use `gradient` prop.

**Rationale**: NyxButton covers all required variants: `filled` (default CTA), `ghost` (cancel/secondary), and supports `theme="danger"` for destructive actions. The `gradient` prop produces a gradient fill without hardcoded CSS. The `disabled` prop covers async loading states. The `size` prop covers sm/md variants currently achieved via custom CSS modifier classes.

**Alternatives considered**:
- Keep styled `<button>` for card-like tiles (vault cards, note cards): rejected — NyxButton with appropriate variant is correct; the card appearance comes from container CSS, not the button itself.

---

## Decision 2: NyxInput / NyxTextarea replace all text inputs and textareas

**Decision**: `NyxInput` replaces all `<input>` elements. `NyxTextarea` replaces all `<textarea>` elements. Both use `variant="outline"` (default) and accept `v-model` directly.

**Rationale**: The NyxInput prop surface covers all required attributes: `type`, `placeholder`, `autofocus`, `disabled`, `v-model`. NyxTextarea similarly covers `placeholder`, `v-model`, `autofocus`. No custom attributes are required beyond what the components expose.

**Alternatives considered**: None — this is a direct swap.

---

## Decision 3: NyxBadge replaces all hand-rolled tag/badge spans

**Decision**: `NyxBadge` replaces all `.note-tag`, `.vault__note-tag`, and toolbar tag spans. Tags without a close action use bare `NyxBadge`. Tags with a dismiss action (note toolbar) use `hasClose` prop with `@close` handler.

**Rationale**: NyxBadge accepts slot content (the tag label string), supports `theme` for colour, and exposes `hasClose`/`@close` for dismissible tags. This covers both read-only (masonry cards) and interactive (toolbar) tag displays.

**Current tag colours**: Tags use `color: var(--nyx-c-primary)` with a 12% background tint. NyxBadge's `theme="primary"` with `variant="soft"` achieves the same result using design-system tokens.

---

## Decision 4: NyxForm + NyxFormField wrap all forms

**Decision**: All form containers use `NyxForm` (handles `@submit` + `preventDefault`). Labelled fields use `NyxFormField` which provides a scoped `id` to associate label and input. Pattern follows `LoginView.vue` exactly.

**Rationale**: `NyxForm` accepts a `@submit` emit and prevents default browser submission. `NyxFormField` auto-generates an `id` and provides it via slot scope — the input receives `:id="id"` to associate with the label. This is already proven in `LoginView.vue`.

**Affected forms**:
- HomeView: vault creation form (name + slug)
- TeamSettingsView: add-member form, add-vault-to-team form
- CommentComposer: comment body form

---

## Decision 5: NyxTabs replaces the CommentSidebar tab switcher

**Decision**: The open/resolved comment tab toggle in `CommentSidebar.vue` is replaced by `NyxTabs` with `v-model` bound to a local `activeTab` ref.

**Rationale**: `NyxTabs` works fully client-side with `v-model` — no router integration required. The `tabs` prop accepts an array of strings; content is provided via named slots (`#tab-{name}`). The `variant="modern"` matches the app's aesthetic.

**Alternatives considered**:
- Keep hand-rolled tab buttons: rejected — directly violates FR-007 and the constitution.

---

## Decision 6: Note/vault selection cards stay as NyxButton

**Decision**: The note cards in `VaultView` and vault cards in `HomeView` (currently `<button>` with card-like styling) remain `NyxButton`-based rather than converting to `NyxCard`. The card *appearance* is achieved via the surrounding container/layout CSS, not the button itself.

**Rationale**: These elements are navigation actions first. `NyxCard` is a content container, not an interactive control. Using `NyxButton` with `variant="ghost"` or `variant="subtle"` and letting the parent apply `border-radius`/`padding` is semantically correct and keeps click/keyboard behaviour intact.

---

## No NEEDS CLARIFICATION items

All unknowns resolved via direct inspection of the nyx-kit package source. No external research required.

---

## nyx-kit Component API Quick Reference

| Component | Key Props | v-model | Events |
|-----------|-----------|---------|--------|
| `NyxButton` | `theme`, `variant`, `size`, `shape`, `disabled`, `gradient`, `type`, `href` | — | `@click` |
| `NyxInput` | `type`, `theme`, `variant`, `size`, `placeholder`, `disabled`, `autofocus` | `string` | `@click`, `@focus`, `@blur` |
| `NyxTextarea` | `theme`, `variant`, `size`, `placeholder`, `disabled`, `autofocus` | `string` | `@click`, `@focus`, `@blur` |
| `NyxBadge` | `theme`, `variant`, `size`, `disabled`, `hasClose` | — | `@click`, `@close` |
| `NyxForm` | `size` | — | `@submit` |
| `NyxFormField` | `label` | — | — (slot scope: `{ id }`) |
| `NyxTabs` | `tabs[]`, `theme`, `variant`, `size`, `position`, `tabTransition` | `string` | — |

### Enum values used in this refactor

```ts
// NyxTheme
'default' | 'primary' | 'danger'

// NyxVariant
'filled' | 'ghost' | 'soft' | 'subtle' | 'outline'

// NyxSize
'sm' | 'md'
```
