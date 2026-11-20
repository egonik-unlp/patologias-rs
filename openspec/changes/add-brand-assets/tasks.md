# Tasks: add-brand-assets

<!-- Tasks de implementación en orden. Ver design.md para el approach técnico. -->

## Implementation

### Tier A · Mark and favicon set (design.md §1)

- [x] 1.1 Create `branding/mark.svg` with the approved field-of-view geometry and inline hex colors
- [x] 1.2 Create `branding/mark-maskable.svg` (full-bleed violet, cell + nucleus inside the 80 % safe zone)
- [x] 1.3 Write `scripts/export-brand-assets.sh` (tool check, Inkscape PNG export, ImageMagick ICO pack, copy `favicon.svg`)
- [x] 1.4 Run the script; replace `public/favicon.ico` and add the generated PNGs and `favicon.svg`
- [x] 1.5 Hand-write `public/site.webmanifest` (icon-only, no service worker)
- [x] 1.6 Add icon, apple-touch-icon, manifest and `theme-color` tags to `shell()` in `src/app.rs`

### Tier B · Palette (design.md §2)

- [x] 2.1 Add the `@theme` block to `style/tailwind.css`: `hema-50…950` (900 = `#4B2E83`) and `eosin`
- [x] 2.2 Check white text on `hema-700`, `-800`, `-900` reaches ≥ 4.5:1; adjust the scale if not
- [x] 2.3 Swap the three primary buttons' `blue-*` classes per the mapping table, keeping the user's other classes intact

### Tier C · Icons (design.md §3)

- [x] 3.1 Create `src/frontend/icons.rs` with the Lucide ISC notice, the shared-SVG macro and the 11 icon components; register it in `src/frontend/mod.rs`
- [x] 3.2 Copy Lucide paths from a pinned Lucide release (record the version in `branding/README.md`); use the concept-sheet paths for the 3 custom icons
- [x] 3.3 Add `BrandMark` using `fill-hema-900` / `fill-eosin`
- [x] 3.4 Sidebar in `App`: brand lockup + icon before each nav label, without overwriting the user's existing sidebar classes
- [x] 3.5 `LogoutUser`: add `IconLogOut`
- [x] 3.6 Add `StudyTypeIcon` in `src/reports/component.rs`; place it in `ReportTable`, `ShowParent` and beside the `ReportHeader` type selector (reactive to `tipo`)

### Docs

- [x] 4.1 Write `branding/README.md`: what the mark depicts, palette + "pink only in the mark" rule, regeneration command, "edit `mark.svg` and `BrandMark` together", Lucide notice and version

## Validation

- [x] `cargo check --features ssr` and `cargo check --features hydrate --target wasm32-unknown-unknown` without errors — hydrate checked with `--lib`; `src/bin/*` are ssr-only and already failed under hydrate before this change
- [x] `cargo clippy` without new warnings in touched files — only `users/component.rs:62`, pre-existing
- [x] `cargo leptos build` succeeds and the generated CSS contains the `hema-*` classes used — verified on the user's running `cargo leptos watch` build
- [x] Re-running `scripts/export-brand-assets.sh` reproduces identical files in `public/` (no unexpected git diff) — byte-identical (md5)
- [x] Every head and manifest icon URL returns its file; note the content type of `site.webmanifest` — all 200; manifest is `application/manifest+json`
- [ ] Manual: tab shows the mark after a hard reload, in light and dark browser chrome
- [x] Manual: sidebar lockup and nav icons render; icons follow link color on hover — icon color = link color on all 5 links (links have no hover color yet)
- [ ] Manual: reports list, report view and type selector show the right study-type icon; selector icon swaps on change
- [x] Manual: no blue remains on primary buttons; red/green messages unchanged; no pink outside the mark
- [x] Browser console: no hydration warnings on pages with icons — checked /create_report and /login
- [x] a11y: icons and sidebar mark are `aria-hidden`; tab order unchanged; screen reader announces only the text labels
- [x] a11y: study types distinguishable in grayscale (shape, not color)

## Post-archive

- [ ] `/documenter distill <dated-change-id>` — **auto-ejecutado por `/opsx:archive`**.
      Solo correr a mano si el auto-trigger falló (re-run con el id completo con fecha,
      ej. `2026-11-20-add-brand-assets`). Sincroniza el use-case (knowledge/modules/<m>/use-cases/) con los deltas del change
      (spec narrativa, decisiones, lore). Humano revisa y acepta edits propuestos.
