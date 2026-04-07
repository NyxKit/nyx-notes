# Research: Vault Icons & Overview Redesign

**Branch**: `005-vault-icons-overview`
**Phase**: 0 — resolved before Phase 1 design

---

## Decision Log

### Icon delivery strategy

**Decision**: Individual SVG files in `app/src/assets/icons/{slug}.svg`, imported into `VaultIcon.vue` as raw strings via Vite's `?raw` suffix and rendered inline with `v-html`.

**Rationale**: No icon library dependency — the 20 SVG files are project assets, not node_modules. Storing each icon as its own file (`home.svg`, `briefcase.svg`, etc.) keeps them version-controlled, editable without touching Vue source, and cleanly separated from component logic. Vite's `?raw` import returns the SVG markup as a string at build time (zero runtime cost); rendering via `v-html` preserves `currentColor` so the icon inherits CSS stroke/fill colour from its parent — consistent with the project's existing inline SVG icon style.

**Alternatives considered**:
- **Lucide Vue** — ~300KB dep, overkill for 20 icons
- **SVG paths hardcoded inside `VaultIcon.vue`** — mixes asset content with component logic; harder to replace an individual icon
- **SVG sprite sheet** — adds a build step; individual files are simpler and sufficient at this scale
- **`<img src="…svg">` import** — loses `currentColor`; icon colour cannot be controlled via CSS
- **Font icons** — conflicts with project's stroke-based icon style; requires web font

---

### Icon set size (20 vs. more/fewer)

**Decision**: Exactly 20, as specified by the user.

**Rationale**: The user explicitly capped the count at 20. This avoids choice paralysis, keeps the picker compact (a 5×4 or 4×5 grid), and prevents the `.vault.json` from accumulating arbitrary user-provided strings on the backend.

---

### Aspect-ratio approach for vault cards

**Decision**: CSS `aspect-ratio: 1 / 1` on the card container.

**Rationale**: Natively supported in all modern browsers (Chrome 88+, Firefox 89+, Safari 15+). No JS required. The masonry `columns` layout already in `HomeView` handles variable-height cards gracefully — switching to fixed 1:1 ratio squares works fine within `columns`.

**Alternatives considered**:
- Padding-top hack (`padding-top: 100%; height: 0`) — obsolete; `aspect-ratio` is the correct modern API
- Fixed pixel height — breaks at different card widths

---

### Icon position: absolute vs. flexbox

**Decision**: Absolute positioning for the icon within the card, with the card set to `position: relative; overflow: hidden`.

**Rationale**: The icon functions as a decorative background element at very large size (~70% of card height). Using absolute positioning lets the icon overflow the natural flow without pushing the text, while `overflow: hidden` clips it cleanly at the card boundary. This matches the "background filler" intent exactly.

**Alternatives considered**:
- CSS Grid with overlapping areas — more complex, same visual result
- Negative z-index — works but is fragile with stacking contexts from NyxButton

---

### PATCH endpoint scope (icon only vs. general vault update)

**Decision**: `PATCH /api/vaults/:vault_id` accepts `UpdateVaultRequest { name?: string, icon?: string | null }` — a general partial-update endpoint.

**Rationale**: Implementing icon-only (`PATCH /api/vaults/:vault_id/icon`) vs. name-only vs. combined is a false distinction. A single PATCH endpoint with optional fields is idiomatic REST, more future-proof, and avoids two nearly-identical handlers. The server merges only the provided fields; absent fields are unchanged.

`null` as the icon value explicitly clears the icon (removes the field from `.vault.json`). This follows the JSON Merge Patch (RFC 7396) convention.

**Alternatives considered**:
- Separate `PATCH /icon` sub-resource — more endpoints for no gain
- Including icon in the CREATE body only — doesn't address post-create updates

---

### Icon slug validation: enum vs. allowlist string check

**Decision**: String allowlist check in the server handler (not a Rust enum).

**Rationale**: Defining a Rust enum for 20 icon slugs would require serde `rename` annotations for every hyphenated slug (e.g. `graduation-cap`), add serde-derived boilerplate to `notes-core`, and couple the domain to a purely presentational concept. A simple `VALID_ICONS: &[&str]` allowlist in the server handler is cleaner and keeps `notes-core` unaware of icon names. Unknown values stored on disk (e.g. from a future migration) are forwarded as-is; the frontend falls back gracefully.

---

### `VaultIconPicker` layout

**Decision**: 5-column grid of icon buttons, 4 rows = 20 icons.

**Rationale**: 5×4 fits compactly in both the inline create form card and the `VaultSettingsView` form. Each icon button is a small square (~40px) with hover highlight using `surface-container-highest` background, matching the "Ghost Border" + subtle hover pattern from DESIGN.md.

---

### Backward compatibility of `.vault.json`

**Decision**: `icon` is an optional field; existing vaults without it are treated as `icon: None` / `icon: undefined`.

**Rationale**: The filesystem-storage doc states unknown frontmatter keys must be preserved on round-trip. The same principle applies to `.vault.json`: the storage layer reads `icon` if present and ignores its absence. No migration script is needed because adding an optional field is non-breaking.

**Note**: This is still a schema change to `.vault.json` and must be documented in `filesystem-storage.md` as the authoritative spec.
