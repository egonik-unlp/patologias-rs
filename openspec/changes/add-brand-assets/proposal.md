# Proposal: Brand assets (favicon, H&E palette, UI icons)

**Change ID:** add-brand-assets
**Date:** 2026-11-20
**Author:** Eduardo Gonik
**Estado:** Aprobado — 2026-11-20
<!-- Marca del gate de aprobación. La escribe SOLO /opsx:propose, y solo tras un Approve explícito
     del usuario en el gate: "Aprobado — <fecha>". Toda (re)generación de este archivo la resetea a
     "Pendiente de aprobación" — se aprueba un texto, no un archivo. -->

## Why

Patologías is the report app of one gynecological pathologist (mostly Pap cytology, plus biopsies). It
still ships the default Leptos favicon, has no logo, and its only accent is Tailwind's stock blue on
three buttons. Its predecessor (the Go app in `under-use/patologias`) had a written design brief —
clinical, calm, "restrained + one accent" — but its logo was a 🔬 emoji.

This change gives the app an abstract pathology identity drawn from the field's own visual language:
the H&E stain. The doctor gets a recognizable tool in her browser tabs and a sidebar that reads at a
glance, and the two study types become visually distinct in lists.

## Scope

Three asset tiers, all agreed in the intent interview (see `context.md`):

- **A · Browser identity.** Mark 1, "field of view": a hematoxylin-violet disc holding one
  eosin-pink squamous cell with a violet nucleus, drawn on a 32-unit grid. Exported as
  `favicon.svg`, `favicon.ico` (16/32/48), `apple-touch-icon.png` (180), `icon-192.png`,
  `icon-512.png`, `icon-maskable-512.png`, and an icon-only `site.webmanifest`. `shell()` gets the
  `<link rel="icon">`, `apple-touch-icon`, `manifest` and `theme-color` tags.
- **B · Brand core.** An H&E palette in Tailwind's `@theme`: a violet scale anchored on hematoxylin
  `#4B2E83` as the single UI accent, plus eosin `#F4BCCB` reserved for the mark. The existing
  `blue-*` accent classes move to the violet scale; slate neutrals and red/green status colors stay.
  The sidebar gets a brand lockup: mark 1 + "Patologías" as text.
- **C · UI icons.** Inline-SVG Leptos components on a 24 px grid, 2 px stroke, `currentColor`, taking
  a `class`: house, file-plus, files, specimen jar, user-plus, log-out, search, print, edit
  (Lucide-derived), plus custom Citología (squamous cell) and Histopatología (glass slide). Claude
  places them in the sidebar nav and next to study types wherever the type is shown.
- **Source of truth.** SVG sources and an Inkscape + ImageMagick export script are committed, so every
  PNG/ICO in `public/` can be regenerated.

**Comunidades afectadas:** pending — the Graphify snapshot in `context.md` is not consolidated
(the MCP failed to connect and there is no `graphify-out/` or `knowledge/` yet).

**Archivos principales:** _graph-derived list pending_ (`/opsx:explore add-brand-assets`). Named in the
intent brief itself, not derived from the graph:

- `src/app.rs` — `shell()` head tags; sidebar nav and brand
- `style/tailwind.css` — `@theme` palette
- `public/` — favicon set and manifest

## Impact

- **Visual only.** No server functions, DTOs, routes or database schema change. Hydration is
  unaffected as long as icon components render identically on server and client (pure static SVG).
- **Accent swap touches existing forms.** Every primary button using `blue-*` becomes violet. The
  consumer list must come from the graph once it is available; today it is known only from reading
  the code.
- **Overlap with the user's Tailwind lessons.** The user is learning Tailwind on this codebase
  (lessons 3 flexbox/nav and 6 `@theme`). By explicit choice, Claude does the placement and the palette
  here; the user's in-progress styling in `src/app.rs` and `src/users/component.rs` (uncommitted at
  proposal time) must be preserved, not overwritten.
- **Licensing.** Lucide-derived paths are ISC-licensed; a short attribution must ship with the icon
  sources.
- **New tooling dependency for regeneration only.** Inkscape and ImageMagick are needed to rebuild
  the raster assets, not to build or run the app.

## Non-goals

- Illustrations and empty states (tier D).
- Print letterhead / report header (tier E).
- Renaming the app; "Patologías" stays as a working name.
- Dark mode.
- Installable / offline web app behavior: the manifest only declares icons, no service worker.
- Adding an icon crate (`leptos_icons`, `icondata`) or any new Cargo dependency.
- Using eosin pink anywhere in the UI outside the mark.

## Criterios de aceptación

- The browser tab shows mark 1 instead of the Leptos icon, legibly on both light and dark browser
  chrome.
- Every favicon and touch-icon size is served from `public/` and referenced from `shell()`; requesting
  each declared path returns the file.
- Every listed icon exists as a component that inherits `currentColor` and accepts a class, so its
  color and size are set from Tailwind classes.
- Running the export script from the committed sources reproduces the raster files in `public/`.
- Each sidebar link shows its icon next to its label, and the sidebar shows the brand lockup.
- Wherever a study's type is shown (reports list, report view, new-report type selector), Citología
  and Histopatología are told apart by their icons, not only by text.
- No primary action is blue any more: the former `blue-*` accents use the violet scale; neutrals and
  status colors are unchanged.
- Eosin pink appears only inside the mark.

## Conflictos con decisiones existentes

No `knowledge/decisions.md` exists yet, so no recorded macro decision can be checked.

- **Unrecorded convention, not a conflict:** the user's teaching-mode arrangement (Claude teaches, the
  user writes styling) is bypassed for this change by the user's explicit decision in the interview.
- **Inherited brief:** the change follows the Go app's `PRODUCT.md` ("restrained + one accent") by
  keeping pink out of the UI. The accent hue itself changes from that app's clinical blue to
  hematoxylin violet, by the user's choice.
