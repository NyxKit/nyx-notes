# CLAUDE.md

Read `AGENTS.md` before doing anything else.

---

## Edit Style

**Plan before editing.**
For any non-trivial change, identify all affected files before touching any of them. State the plan and confirm if the scope is larger than expected.

**Minimize diff size.**
Make the smallest change that achieves the goal. A targeted `Edit` on three lines is better than rewriting a section. Prefer surgical edits over reconstructions.

**Avoid broad rewrites.**
Do not rewrite a file to clean it up unless that is the explicit request. Reformatting, restructuring, or "improving" content beyond the stated task introduces noise and risks losing intent.

**Ask before introducing dependencies.**
Do not add a new Rust crate, npm package, or external service without flagging it first. State what it is, why it is needed, and whether there is a lighter alternative already in scope.

## Active Technologies
- TypeScript 5.x / Vue 3.5 / Vite + Vue Router 5.0, Pinia 3.0, nyx-kit 1.4.4, ofetch 1.5 (001-route-domains)
- Filesystem (Rust backend) — no frontend storage changes (001-route-domains)
- TypeScript 5.x / Vue 3.5 / Vite + nyx-kit 1.4.4 (`NyxButton`, `NyxInput`, `NyxTextarea`, `NyxBadge`, `NyxForm`, `NyxFormField`, `NyxTabs` — all already installed) (002-nyx-kit-primitives)
- N/A — UI-only refactor (002-nyx-kit-primitives)

## Recent Changes
- 001-route-domains: Added TypeScript 5.x / Vue 3.5 / Vite + Vue Router 5.0, Pinia 3.0, nyx-kit 1.4.4, ofetch 1.5
