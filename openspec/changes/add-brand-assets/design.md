# Design: add-brand-assets

> **Graph limitation.** The Graphify MCP failed to connect and there is no `graphify-out/` yet, so this
> design cites no `graph://` nodes. File references below come from reading the code during
> `/opsx:explore` and must be re-checked against the graph once `/opsx:explore add-brand-assets`
> consolidates the snapshot in `context.md`.

## Approach

Three layers, each with one source of truth:

```
 branding/mark.svg ──(scripts/export-brand-assets.sh)──▶ public/ favicon set + manifest
        │                                                        ▲
        │ same geometry                                          │ <link> tags in shell()
        ▼                                                        │
 src/frontend/icons.rs  (BrandMark + 11 stroke icons)      src/app.rs
        ▲                                                        │
        │ fill/stroke via Tailwind classes                       │ class="… hema-*"
 style/tailwind.css  @theme { --color-hema-*, --color-eosin }  ◀─┘
```

### 1. Mark source and raster export (tier A)

- `branding/mark.svg` is the master, 32-unit viewBox, hex colors inline so it stands alone as a file:
  disc `circle r=15.5` in hematoxylin `#4B2E83`; cell path
  `M9 12.2 L15 7.6 L22 9 L25 15.4 L21.6 22.8 L13.6 24.4 L8 19.2 Z` in eosin `#F4BCCB` with a
  2.4-unit same-color stroke and round joins (rounds the corners); nucleus `circle cx=16.6 cy=16
  r=3.1` in violet. Geometry is the one approved on the concept sheet.
- `branding/mark-maskable.svg`: full-bleed `#4B2E83` square with the cell and nucleus scaled to fit
  the inner 80 % safe zone (the disc is dropped; the square is the background).
- `scripts/export-brand-assets.sh` (bash, `set -euo pipefail`, checks that `inkscape` and `magick`
  exist and fails with a clear message otherwise):
  - `inkscape --export-type=png --export-width=N --export-filename=…` for 16, 32, 48, 180, 192, 512
    from `mark.svg`, and 512 from `mark-maskable.svg`;
  - `magick` packs 16/32/48 into `public/favicon.ico`;
  - copies `mark.svg` to `public/favicon.svg`;
  - **(apply)** `apple-touch-icon.png` is rendered from `mark-maskable.svg`, not `mark.svg`: iOS fills
    transparent corners with black, and it applies its own rounded-square mask anyway;
  - **(apply)** the maskable variant keeps the cell at scale 1 (farthest point 10.3 units from center,
    inside the 12.8-unit safe radius), so a circular mask looks like the favicon;
  - intermediate 16/32/48 PNGs go to a temp dir, only the served files land in `public/`.
- `public/site.webmanifest` is hand-written and committed (not generated): `name`/`short_name`
  "Patologías", `icons` (192, 512, 512 maskable), `theme_color` and `background_color`. No `start_url`
  display modes beyond the default, no service worker.
- `shell()` head gains static tags (the shell is plain HTML, so no `leptos_meta` needed):
  `icon` → `/favicon.ico` (sizes="32x32"), `icon` → `/favicon.svg` (type `image/svg+xml`),
  `apple-touch-icon` → `/apple-touch-icon.png`, `manifest` → `/site.webmanifest`,
  `<meta name="theme-color" content="#4B2E83">`.

### 2. Palette (tier B)

`style/tailwind.css` gets an `@theme` block:

- `--color-hema-50 … --color-hema-950`, an oklch scale at hue ≈ 295 whose 900 step is `#4B2E83`
  (oklch ≈ 0.37 0.13 295). Lighter steps raise L and lower C so 50/100 work as selection tints.
- `--color-eosin: #F4BCCB`, used only by `BrandMark`.

Accent swap, one-to-one so the button's state ladder keeps its shape:

| Before | After |
|---|---|
| `bg-blue-500` | `bg-hema-700` |
| `hover:bg-blue-600` | `hover:bg-hema-800` |
| `active:bg-blue-800` | `active:bg-hema-900` |

Sites: the three primary buttons (`CreateReport`, and two forms in `src/users/component.rs`). Slate
neutrals and the red/green status classes are left alone.

### 3. Icons (tier C)

New module `src/frontend/icons.rs` (registered in `src/frontend/mod.rs`):

- A private macro generates one `#[component]` per stroke icon from its path data, all sharing
  `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"
  stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" class=class>`, with
  `#[prop(into, optional)] class: String`.
- Components: `IconHouse`, `IconFilePlus`, `IconFiles`, `IconSpecimenJar`, `IconUserPlus`,
  `IconLogOut`, `IconSearch`, `IconPrint`, `IconEdit`, `IconCitologia`, `IconHistopatologia`.
  Lucide-derived paths are copied from the Lucide source at a pinned version, not redrawn; the custom
  ones (`SpecimenJar`, `Citologia`, `Histopatologia`) use the paths from the concept sheet.
  **(apply)** Lucide pinned at `lucide-static@0.460.0`. `IconSpecimenJar` was redrawn as a lidded jar
  with a fixative line instead of the concept sheet's flask: a flask reads as chemistry, a jar as a
  biopsy container.
- `BrandMark` (same module): the filled mark, using `class="fill-hema-900"` / `fill-eosin` on its
  shapes so it reads the palette from `@theme`, plus the caller's `class` for size.

Placement:

- **Sidebar (`App` in `src/app.rs`):** brand lockup at the top (`BrandMark` + "Patologías"); each nav
  link becomes icon + label in an inline flex row with a small gap; `LogoutUser` gets `IconLogOut`.
- **Study type:** a `StudyTypeIcon { tipo: StudyTypeDto }` component in `src/reports/component.rs`
  matches the type to `IconCitologia` / `IconHistopatologia`. Used in the `ReportTable` "Tipo" cell,
  in `ShowParent` next to "Tipo", and in `ReportHeader` next to the type `<select>` as a reactive
  view of the `tipo` signal (an `<option>` cannot hold SVG).
- `IconSearch`, `IconPrint`, `IconEdit` ship as components but are only placed where a matching
  control already exists; where none exists yet they wait for that control's change.

### 4. Attribution

`branding/README.md`: what the mark depicts, the palette values and their use rule (pink only in the
mark), how to regenerate, and the Lucide ISC license notice. The same notice heads `icons.rs`.

## Archivos a modificar

_From code reading, not from the graph (see limitation above)._

| Archivo | Cambio |
|---|---|
| `branding/mark.svg` | new — master mark |
| `branding/mark-maskable.svg` | new — maskable variant |
| `branding/README.md` | new — palette, usage rule, regeneration, Lucide notice |
| `scripts/export-brand-assets.sh` | new — SVG → PNG/ICO export |
| `public/favicon.ico` | replaced — mark at 16/32/48 |
| `public/favicon.svg`, `apple-touch-icon.png`, `icon-192.png`, `icon-512.png`, `icon-maskable-512.png` | new — generated |
| `public/site.webmanifest` | new — icon-only manifest |
| `style/tailwind.css` | `@theme` hema scale + eosin |
| `src/frontend/icons.rs` | new — `BrandMark` + 11 icon components |
| `src/frontend/mod.rs` | register `icons` |
| `src/app.rs` | head tags in `shell()`; brand lockup and nav icons in `App` |
| `src/reports/component.rs` | `StudyTypeIcon`; use in list, view, type selector; button accent swap |
| `src/users/component.rs` | button accent swap; log-out icon in `LogoutUser` |

## Alternativas consideradas

- **`leptos_icons` + `icondata` crates** — rejected by the user: a new dependency, its Leptos 0.8
  support was unverified, and inline components are styled with plain Tailwind classes.
- **SVG sprite in `public/` with `<use href>`** — rejected: external `<use>` can't be restyled per
  part, and it adds a request; inline SVG costs nothing for 11 small icons.
- **`<img src="/icons/x.svg">`** — rejected: an `<img>` cannot inherit `currentColor`.
- **Clinical blue accent (Go app's palette)** — rejected in favor of H&E violet: still one accent, but
  native to pathology. Recorded in `context.md`.
- **Marks 2 (cell sheet) and 3 (lone cell)** — rejected for the favicon: 2 blurs at 16 px; 3 vanishes
  on dark tab bars without a tile. Mark 3's shape lives on as `IconCitologia`.
- **Generating `site.webmanifest` from the script** — unnecessary; it changes only when names or
  colors change.
- **Putting sources in `public/`** — rejected: cargo-leptos copies `public/` wholesale to the site
  root, so sources there would be served.

## Riesgos

- **Mark geometry lives twice** (`branding/mark.svg` and `BrandMark` in Rust). A later edit to one can
  drift from the other; `branding/README.md` says to change both.
- **User's in-progress work.** `src/app.rs` and `src/users/component.rs` had uncommitted edits during
  planning (the user's Tailwind exercises, e.g. the `LogoutUser` wiring). Apply must edit the working
  tree as it stands and keep the user's classes, touching only what this change needs.
- **Favicon caching.** Browsers cache favicons hard; verification needs a hard reload or a fresh
  profile, or the old Leptos icon will appear to persist.
- **Manifest MIME type.** `file_and_error_handler` serves `public/` files with a guessed content type;
  check `site.webmanifest` comes back as `application/manifest+json` (or at least JSON). If not, it is
  harmless for icons but worth noting.
- **Tailwind scanning.** `style/tailwind.css` uses `source(none)` + `@source "../src/**/*.rs"`, so the
  new `hema-*` / `eosin` classes are picked up only if written literally in `.rs` files — no class
  names built from strings.
- **Inkscape CLI drift.** The script targets the Inkscape 1.x CLI (1.4.3 installed); 0.92 flags differ.
- **Scale tuning.** The 50–800 steps are derived, not chosen by eye; they may need a visual pass on
  the real UI.
