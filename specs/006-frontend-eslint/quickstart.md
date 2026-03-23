# Quickstart: Frontend ESLint

## Prerequisites

Dependencies must be installed before running lint:

```bash
cd frontend
pnpm install
```

## Run lint

```bash
cd frontend
pnpm lint
```

Exits 0 on a clean codebase. Reports violations with file path and line number.

## Auto-fix

```bash
cd frontend
pnpm lint:fix
```

Applies all safe mechanical fixes (semicolons, quotes, unused imports). Reports any residual violations that require manual resolution.

## After fixing manually

Run `pnpm lint` again to confirm the baseline is clean before committing.
