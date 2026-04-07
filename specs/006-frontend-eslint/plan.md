# Implementation Plan: Frontend ESLint Integration

**Branch**: `006-frontend-eslint` | **Date**: 2026-03-24 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/006-frontend-eslint/spec.md`

## Summary

Install ESLint peer dependencies and create `app/eslint.config.mjs` that extends the nyx-kit shared rule set, adds project-level style overrides enforcing the constitution's no-semicolons / single-quote conventions, and registers `lint` and `lint:fix` scripts in `app/package.json`. Run the auto-fixer, then manually resolve any residual violations until the baseline is clean.

## Technical Context

**Language/Version**: TypeScript 5.6 / Vue 3.5 (frontend only)
**Primary Dependencies**: `nyx-kit/eslint` (shared config); `eslint ^9.18`, `eslint-plugin-vue ^9.32`, `@vue/eslint-config-typescript ^14.3`, `eslint-plugin-oxlint ^0.15` (peer deps)
**Storage**: N/A
**Testing**: Manual — `pnpm lint` exit code and output
**Target Platform**: Developer workstation (Node.js 22, run via pnpm scripts)
**Project Type**: Web application — Vue 3 SPA
**Performance Goals**: Lint run completes in under 30 seconds (SC-004)
**Constraints**: No `eslint-disable` suppressions; `build` script untouched; `.mjs` config to avoid `jiti` dependency
**Scale/Scope**: ~35 source files in `app/src/`

## Constitution Check

*GATE: Must pass before implementation begins.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I — Docs as source of truth | PASS | `docs/conventions/README.md` should be updated to mention ESLint as the enforcer of §VI style rules. Minor doc touch. |
| II — Strict layer boundaries | PASS | Tooling change; no backend layers touched. |
| III — Filesystem is content source | PASS | No on-disk note format changes. |
| IV — Test coverage per layer | PASS | No application logic added; lint itself is the validation mechanism. |
| V — Security by design (new deps) | PASS with justification | 4 peer deps added: `eslint`, `eslint-plugin-vue`, `@vue/eslint-config-typescript`, `eslint-plugin-oxlint`. All are standard, well-maintained, and required by `nyx-kit/eslint`'s peer dep list. No Rust crates added. |
| VI — Frontend constraints | PASS | This feature enforces §VI style rules. Config adds `semi: never` and `quotes: single` (base rules; `@typescript-eslint/semi`/`quotes` were removed in typescript-eslint v8). |

**Constitution gates: all PASS.** No Complexity Tracking required.

## Project Structure

### Documentation (this feature)

```text
specs/006-frontend-eslint/
├── plan.md          ✅ this file
├── research.md      ✅ phase 0 output
├── quickstart.md    ✅ phase 1 output
└── tasks.md         (phase 2 — /speckit.tasks)
```

### Source Code (affected files)

```text
app/
├── eslint.config.mjs          # NEW — extends nyx-kit/eslint + project overrides
├── package.json               # MODIFIED — 4 new devDeps + lint/lint:fix scripts
└── src/                       # MODIFIED — code adjustments for lint compliance
    ├── **/*.ts
    └── **/*.vue
```

No other directories or files are touched.

## Phase 0: Research

Complete. See [research.md](./research.md).

**Resolved decisions:**
1. Config format: `eslint.config.mjs` (avoids `jiti` dep)
2. Peer dep versions: exact ranges from nyx-kit's own peer deps
3. Project overrides: `@typescript-eslint/semi: never`, `@typescript-eslint/quotes: single`
4. Script targets: `./src` explicitly
5. Known conflicts: `vue/multi-word-component-names` needs verification for `App.vue`; `no-unused-vars` may overlap with `tsc`

## Phase 1: Design

### eslint.config.mjs structure

```
[nyx-kit shared config (spread)]
+
[project override block]
  files: ['**/*.{ts,vue}']
  rules:
    semi: ['error', 'never']
    quotes: ['error', 'single', { avoidEscape: true }]
```

> **Note**: `@typescript-eslint/semi` and `@typescript-eslint/quotes` were removed in typescript-eslint v8. The base `semi` and `quotes` rules are used instead — they apply correctly to `.ts` and `.vue` files via the configured parser.

### package.json changes

**devDependencies to add:**
- `eslint: ^9.18.0`
- `eslint-plugin-vue: ^9.32.0`
- `@vue/eslint-config-typescript: ^14.3.0`
- `eslint-plugin-oxlint: ^0.15.6`

**scripts to add:**
- `"lint": "eslint ./src"`
- `"lint:fix": "eslint ./src --fix"`

### Code adjustment strategy

1. Run `pnpm lint:fix` first — auto-fixes mechanical violations (quotes, semicolons, simple unused imports).
2. Run `pnpm lint` — inspect residual violations.
3. Resolve each manually: rename components if `vue/multi-word-component-names` fires on single-word files; fix any `@typescript-eslint` correctness violations.
4. Run `pnpm lint` again to confirm zero errors.

### Docs update

Add a line to `docs/conventions/README.md` under the Vue/TS section noting that `pnpm lint` enforces §VI style conventions. Keep it to one sentence — this is a pointer, not a replacement for the constitution.

## Implementation Order

1. Install peer deps (`pnpm add -D ...`)
2. Create `eslint.config.mjs`
3. Add scripts to `package.json`
4. Run `pnpm lint:fix` (auto-fix pass)
5. Run `pnpm lint` (identify residuals)
6. Fix residuals manually, file by file
7. Run `pnpm lint` to confirm clean baseline
8. Update `docs/conventions/README.md`
