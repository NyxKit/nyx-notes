# Design System Strategy: The Focused Editorial

## 1. Overview & Creative North Star
The Creative North Star for this design system is **"The Silent Atelier."** 

In a world of noisy interfaces, this system treats the digital screen as a high-end physical workspace—a dark, quiet room where only the most essential tools are visible. We are breaking the "template" look by moving away from rigid grids and boxy containers. Instead, we utilize **intentional asymmetry** and **tonal depth**. The UI does not sit *on* the screen; it emerges *from* it. By prioritizing "dark space" over lines, we create a signature experience where the writer’s thoughts are the only high-contrast elements.

## 2. Colors & Surface Philosophy
The palette is rooted in deep obsidian tones and a low-saturation, modern purple (`primary: #cbc2e4`). This is not a "black mode" app; it is a layered, monochromatic environment designed to reduce ocular strain.

*   **The "No-Line" Rule:** To achieve a premium editorial feel, **1px solid borders are strictly prohibited** for sectioning. Do not use lines to separate a sidebar from a canvas. Boundaries must be defined solely through background shifts. For example, the writing canvas resides on `surface` (#0e0e10), while the sliding sidebar sits on `surface-container-low` (#131316).
*   **Surface Hierarchy & Nesting:** Treat the UI as stacked sheets of fine, dark paper. 
    *   **Base Layer:** `surface` (#0e0e10).
    *   **Navigation/Sidebars:** `surface-container-low` (#131316).
    *   **Floating Menus/Popovers:** `surface-container-highest` (#25252b).
*   **The "Glass & Gradient" Rule:** For floating action buttons or the vault switcher, use Glassmorphism. Apply `surface-variant` (#25252b) at 60% opacity with a `backdrop-filter: blur(20px)`. This creates a "frosted" look that feels integrated rather than pasted on.
*   **Signature Textures:** Use a subtle linear gradient for primary CTAs: `primary` (#cbc2e4) to `primary-container` (#49435f) at a 135-degree angle. This adds "visual soul" and depth that flat colors lack.

### Project Logo

The canonical mark is [`app/public/nyx-notes.svg`](app/public/nyx-notes.svg): a capital
N forming the sides of a connected, rounded note outline, with text strokes in the
open areas beside its diagonal. Keep the N visually dominant. Draw the N above
the separately colored top and bottom outline strokes, so the letter's rounded
ends and joins define the visible connections.
Align text strokes on four shared horizontal rows within the N, spaced 53⅓ viewBox
units vertically from y = 80 to 240. Include unbroken rows aligned with the top and
bottom of its stems, inset where they approach the diagonal. All endpoints facing
the diagonal sit 56 viewBox units horizontally from its centerline so strokes
remain separated at small sizes. Omit fragments that cannot fit with that clearance.
Use matching 20-unit strokes for the N and outer outline, and 14-unit text strokes.
Inset the text to x = 72–184 and connect the outer outline tips to the N at y = 80
and 240. Use this SVG as the application favicon.

Use exactly three solid purple shades: `primary` (`#cbc2e4`) for the N, the rounded
RGB midpoint (`#8a83a2`) for the outline, and `primary-container` (`#49435f`) for
the text strokes. Keep the background transparent and favor the dark app surface
when displaying it. See [logo usage](docs/interface/frontend.md#project-logo).

## 3. Typography: The Editorial Voice
We use a dual-font approach to separate the "Act of Organizing" from the "Act of Writing."

*   **Interface (The Framework):** We use **Manrope** and **Inter**. These are clinical, modern, and precise.
    *   Headings and large UI text use **Manrope** for a structural, architectural feel.
    *   Small labels and micro-text use **Inter** for maximum legibility.
*   **The Writing Canvas (The Soul):** We use **Newsreader**. This is a high-end serif that mimics the experience of a luxury literary journal.
    *   Body text in the editor: 1rem / 16px.
    *   Document title: 1.375rem — soft and elegant without feeling loud.
*   **Hierarchy Logic:** Contrast is achieved through weight and scale, not color. Interface labels should use `on-surface-variant` (#abaab1) to stay unobtrusive, while the text being written uses `on-surface` (#e6e4ec).

## 4. Elevation & Depth
Traditional drop shadows are too "software-like." We use **Tonal Layering**.

*   **The Layering Principle:** Depth is achieved by placing a `surface-container-lowest` (#000000) element inside a `surface-container-high` (#1f1f24) parent. This "recessed" look is perfect for threaded comment cards.
*   **Ambient Shadows:** If an element must float (e.g., a context menu), use a shadow with a blur of `24px`, spread of `-4px`, and an opacity of 6% using the `on-background` color. This mimics natural light.
*   **The "Ghost Border":** If accessibility requires a stroke (e.g., a focused input), use `outline-variant` (#47474d) at **20% opacity**. It should be felt, not seen.

## 5. Component Guidelines

### Full-Width Editor
*   **Padding:** Use 5.5–7rem of lateral margin to create a focused column.
*   **Caret:** Use `primary` (#cbc2e4) with a slight outer glow to guide the eye.

### Sliding Sidebars
*   **Background:** `surface-container-low` (#131316).
*   **Transition:** Use a `300ms cubic-bezier(0.4, 0, 0.2, 1)` for a "sliding silk" feel.
*   **Divider Replacement:** Use ~1.4rem of empty space to separate the navigation list from the footer.

### Subtle Tag Pills
*   **Style:** No background. Use a "Ghost Border" (`outline-variant` at 20%) and small label text (0.75rem).
*   **Active State:** Background `primary-container` (#49435f), text `on-primary-container` (#d5cbee). Fully rounded (`border-radius: 50%`).

### Vault Switcher
*   **Style:** A glassmorphic card using `surface-container-high` at 70% opacity.
*   **Interaction:** On hover, the background shifts to `surface-bright` (#2b2c32).

### Browse Card Family
*   **In Scope:** Only vault dashboard cards, the inline create-vault card, and vault note masonry cards belong to the shared browse-card family.
*   **Component Strategy:** Keep `VaultCard` and `NoteCard` as standalone components. Do not introduce a shared `BrowseCardSurface` abstraction.
*   **Visual Shell:** Use `NyxCard` as the visible shell, but wrap the rendered card content in an internal anchor so the default browser link context menu remains available.
*   **Overview Layouts:** Use `NyxGrid` for browse overviews. `HomeView` uses `grid` mode for vault cards, while `VaultView` uses `masonry` mode for note cards.
*   **Surface Treatment:** Use tonal layering (`surface-container-low` to `surface-container-high`) instead of visible dividers. Hover and focus should strengthen the surface shift, not introduce a hard border.
*   **Typography:** Card titles use **Manrope** with structural weight; supporting metadata stays subdued in **Inter**.
*   **Vault Card Layout:** Title, slug, and description sit in the top-left. The decorative icon is oversized, anchored in the bottom-right, and allowed to overflow the card by roughly 10% on the bottom and right edges.
*   **Note Card Content:** Show the title and a subdued description distilled from the first actual paragraph of saved note content, followed by supporting metadata such as tags or update time.
*   **Exclusions:** Do not restyle the vault switcher, empty-state prompts, comment threads, settings containers, modals, or navigation chrome to match the browse-card family.

### Threaded Comment Cards
*   **Structure:** Avoid lines. Use 1rem of indentation and a slight background shift to `surface-container-lowest` (#000000) for each nested reply.
*   **Connectors:** If needed, use a 1px vertical track of `surface-variant` (#25252b) at 30% opacity.

### Buttons & Inputs
*   **Primary Button:** `--nyx-radius-sm` (0.25rem). Use the Signature Texture gradient.
*   **Icon Toggles:** No background or border in default state. Use `on-surface-variant` (#abaab1). On hover, shift to (#bdb4d5) at 10% opacity background.
*   **Checkboxes:** Custom squares with a small radius (`--nyx-radius-xs`). Checked state uses `primary` background with light icon.

## 6. Do's and Don'ts

### Do
*   **Use Vertical White Space:** Use 2–3rem of vertical space to separate thoughts and sections instead of dividers.
*   **Prioritize Typography:** Let the `Newsreader` font do the heavy lifting for the "High-End Editorial" feel.
*   **Embrace the Dark:** Ensure `surface` (#0e0e10) remains the dominant color to keep the environment distraction-free.

### Don't
*   **Don't use pure white:** Never use `#ffffff` for text. Use `#e6e4ec to prevent light bleed/glare.
*   **Don't use 100% opaque borders:** They break the "Silent Atelier" illusion and feel like a generic dashboard.
*   **Don't use standard shadows:** Avoid "muddy" black shadows. Stick to the Ambient Shadow spec to maintain the dark-mode depth.
