# Research: Frontend ESLint Integration

## Decision 1: Config file format — `.mjs` over `.ts`

**Decision**: Use `eslint.config.mjs` (ES module JavaScript), not `eslint.config.ts`.

**Rationale**: ESLint 9's flat config natively supports `.mjs`. Using `.ts` requires `jiti` as an additional dev dependency so ESLint can transpile the file at load time. The nyx-kit shared config is exported as `.mjs`; composing from `.mjs` is direct and dependency-free. Constitution §V prohibits new dependencies without justification — and there is none here when `.mjs` achieves the same result.

**Alternatives considered**:
- `eslint.config.ts` + `jiti`: adds one extra dev dep with no functional gain.
- `eslint.config.js`: equivalent to `.mjs` since `app/package.json` has `"type": "module"`, but `.mjs` is explicit about ES module intent and avoids ambiguity.

---

## Decision 2: Peer dependency versions

**Decision**: Install the following as `devDependencies`, matching nyx-kit's own pinned ranges:

| Package | Version range | Role |
|---------|--------------|------|
| `eslint` | `^9.18.0` | Core linter (flat config era) |
| `eslint-plugin-vue` | `^9.32.0` | Vue 3 specific rules |
| `@vue/eslint-config-typescript` | `^14.3.0` | Provides `defineConfigWithVueTs` + TS-aware rules |
| `eslint-plugin-oxlint` | `^0.15.6` | Shims that disable ESLint rules oxlint handles |

**Rationale**: These are the exact peer deps declared in `nyx-kit/package.json` `peerDependencies`. Using the same ranges guarantees the shared config loads without version conflicts.

**Alternatives considered**:
- Installing `oxlint` CLI as a standalone runner: out of scope per spec Assumption 2; the shim is sufficient.

---

## Decision 3: Project-level style overrides

**Decision**: Add a project-level config block after the nyx-kit shared block that enforces no-semicolons and single-quote style, as required by Constitution §VI.

Rules to add:
- `'@typescript-eslint/semi': ['error', 'never']` — no trailing semicolons
- `'@typescript-eslint/quotes': ['error', 'single', { avoidEscape: true }]` — single quotes, allow double quotes to avoid escaping

**Rationale**: The nyx-kit shared config (`flat/essential` + `recommended` + oxlint shims) enforces correctness and type safety but not code style. Constitution §VI is explicit: no semicolons, single quotes. These rules are auto-fixable by `lint:fix`.

**Caveat**: If `eslint-plugin-oxlint`'s shim disables `@typescript-eslint/semi` or `@typescript-eslint/quotes` (because oxlint handles them), the implementation must verify this at runtime and adjust the override approach if needed (e.g. use the base `semi`/`quotes` rules instead).

**Alternatives considered**:
- Prettier for formatting: introduces another tool and a different config file; out of scope.
- Not enforcing style via ESLint: leaves constitution §VI unenforced by tooling, defeating the purpose of this feature.

---

## Decision 4: Lint script targets

**Decision**: Target `./src` explicitly in both scripts:

```
"lint": "eslint ./src"
"lint:fix": "eslint ./src --fix"
```

**Rationale**: The nyx-kit shared config already ignores `**/dist/**`, `**/dist-ssr/**`, and `**/coverage/**`. Explicitly targeting `./src` scopes lint to the application source and avoids linting config files (`vite.config.ts`, `eslint.config.mjs`) which follow different conventions.

**Alternatives considered**:
- Lint the entire `app/` directory: would include `vite.config.ts` and `eslint.config.mjs`, which may need style exceptions. Narrower scope is safer.

---

## Decision 5: Known rule conflicts to check at implementation time

The `tsconfig.json` already enforces `noUnusedLocals: true` and `noUnusedParameters: true`. ESLint's `@typescript-eslint/no-unused-vars` overlaps with these. This is not a conflict — both can coexist — but may produce duplicate warnings. If noisy, the implementation can disable the ESLint rule (unused-vars is already caught by `tsc`).

The `vue/multi-word-component-names` rule (part of `flat/essential`) flags single-word component names. `App.vue` is conventionally exempt (root component). Views like `HomeView.vue`, `LoginView.vue` are multi-word. If any single-word component files exist, they must be renamed or the rule must be scoped to allow them.
