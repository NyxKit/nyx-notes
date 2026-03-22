# Contract: UI Primitive Standard

**Feature**: 002-nyx-kit-primitives
**Type**: Internal UI contract — enforces the constitution's nyx-kit rule

## Rule

No `.vue` file in `frontend/src/` may contain a native `<button>`, `<input>`, `<textarea>`, or hand-rolled badge `<span>` in its template. All interactive controls and display primitives MUST use the corresponding nyx-kit component.

## Permitted Exceptions

| Element | When permitted |
|---------|---------------|
| `<RouterLink>` | Navigation links — not a primitive |
| `<a>` | External links only (opens new tab), where `NyxButton href` is not suitable |
| `<span>`, `<div>`, `<p>` etc. | Layout and text containers — not interactive |
| Third-party slot output | Content rendered by nyx-kit internals (e.g. `NyxEditor`) |

## nyx-kit Import Convention

```ts
// Components
import { NyxButton, NyxInput, NyxTextarea, NyxBadge, NyxForm, NyxFormField, NyxTabs } from 'nyx-kit/components'

// Types/enums (only import what you use)
import { NyxVariant, NyxTheme, NyxSize, NyxShape } from 'nyx-kit/types'
```

## Mapping Reference

| Primitive | nyx-kit replacement |
|-----------|-------------------|
| `<button>` | `<NyxButton>` |
| `<input>` | `<NyxInput>` |
| `<textarea>` | `<NyxTextarea>` |
| tag/badge `<span>` | `<NyxBadge>` |
| `<form>` or form container `<div>` | `<NyxForm>` |
| labelled field wrapper | `<NyxFormField>` |
| tab switcher | `<NyxTabs>` |
