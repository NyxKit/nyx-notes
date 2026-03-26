# Contract: Browse Card Surfaces

## Purpose

Define the user-visible contract for all surfaces included in the shared browse-card family introduced by feature `008-adopt-nyxcard-cards`.

## In-Scope Surfaces

- Vault tiles on the home dashboard
- The inline create-vault surface shown inside the same dashboard grid
- Note tiles in the vault notes masonry view

## Out-of-Scope Surfaces

- Vault switcher container
- Empty-state welcome containers
- Comment thread containers
- Settings panels, modals, and navigation chrome

## Visual Contract

- In-scope surfaces must share the same card-family cues for spacing, corner treatment, tonal layering, and typographic hierarchy.
- The card family must remain aligned with Silent Atelier rules: tonal separation over obvious borders, subdued secondary text, and no generic dashboard styling.
- Surface-specific content such as tags, timestamps, icons, and form controls may vary, but they must still read as members of the same family.

## Interaction Contract

- Vault and note cards must continue to behave as single, obvious activation targets.
- Vault and note cards must wrap their card content in an internal `RouterLink` anchor so users retain standard browser link behavior.
- Keyboard users must be able to reach and trigger the same primary outcome available to pointer users.
- The create-vault card must retain form semantics and must not behave like a single navigation target.
- Hover, focus, and pressed states must communicate interactivity consistently across all browse-and-select cards.

## Content Contract

- Vault cards must show a vault title, slug, and optional description, with the decorative icon positioned as an oversized bottom-right element.
- Note cards must show a note title, an optional distilled description, and may additionally show tags and last-updated context.
- Creation cards must show the existing create-vault inputs and actions, plus a vault description field, without removing required controls.
- Long content must remain readable through graceful truncation or wrapping without overlap.

## Acceptance Contract

- A user can distinguish interactive browse cards from non-interactive containers at a glance.
- A user can open a vault or note from anywhere the card indicates as the primary target.
- Switching between the dashboard and the vault note grid feels visually coherent, not like separate component systems.
- Surfaces outside the browse-card family retain their own documented design treatments.
