# Context: add-brand-assets

## Entrevista de intención

<!-- Recorded by the convergent close of /opsx:explore (2026-11-20). Grows by append. -->

### Q&A

| Pregunta | Respuesta |
|---|---|
| Who is the app for, and is there existing branding? | A specific doctor who writes her own reports, same as the Go app in `under-use/patologias`. Its `PRODUCT.md` brief applies: gynecological pathology lab, clinical/calm/precise, "restrained + one accent". The old app used the 🔬 emoji as its logo. |
| Is "Patologías" the final name? | Undecided ("idk"). |
| Which asset tiers are in scope (A browser identity, B brand core, C UI icons, D illustrations, E print)? | Up to C. |
| Literal (microscope, slide) or abstract imagery? | More abstract than literal. |
| Which palette: (i) clinical blue from the Go app, or (ii) H&E (hematoxylin violet as the single UI accent, eosin pink only inside the mark)? | H&E. |
| Which mark (1 field of view, 2 cell sheet, 3 lone squamous cell), after reviewing the concept sheet? | Mark 1, field of view. |
| Should the change add the H&E tokens to `style/tailwind.css`, or leave that to the user's Tailwind lesson 6? | The change adds the colors. Confirmed reading: existing `blue-*` accent classes switch to the new violet scale; slate neutrals and red/green status colors stay. |
| Should the change only deliver icons, or also place them in the UI? | Claude places them (nav and study-type markers), even though it overlaps the user's Tailwind lessons. |
| Is the icon list complete: house, file-plus, files, specimen jar, user-plus, log-out, search, print, edit + custom Citología and Histopatología? | Yes, for now. |
| Inline SVG Rust components or the `leptos_icons` crate? | Inline components. |
| Non-goals: illustrations/empty states, print letterhead, renaming, dark mode, installable/offline web app? | Correct, all out. |
| Commit SVG sources + export script, or only generated files? | Commit sources. |
| Acceptance: tab shows mark 1, all favicon/touch-icon sizes served from `public/`, every icon a `currentColor` component? | OK. |
| Change name `add-brand-assets`? | Yes. |

### Intent brief

**Intención:** Patologías is a pathology report app for one gynecological pathologist (mostly Pap cytology, plus biopsies). It still shows the default Leptos favicon and has no visual identity. Give it an abstract pathology identity that fits the clinical, calm brief carried over from the Go app, so she has a recognizable tool. It replaces the 🔬 emoji the old app used.

**Alcance:**
- **A, browser identity:** mark 1 ("field of view": a violet disc holding one pink squamous cell with a violet nucleus) exported as `favicon.svg`, `favicon.ico` (16/32/48), `apple-touch-icon` 180, icon 192/512 + maskable, and an icon-only `site.webmanifest`. Add the `<link>` and `theme-color` tags to `shell()` in `src/app.rs`.
- **B, brand core:** H&E palette as a Tailwind `@theme` scale in `style/tailwind.css`. Hematoxylin violet (`#4B2E83` as the base) is the single UI accent. Eosin pink (`#F4BCCB`) is used only inside the mark. The existing `blue-*` accent classes switch to the new scale. Slate neutrals and red/green status colors stay. Mark 1 plus "Patologías" as text appears as the sidebar brand.
- **C, UI icons:** inline-SVG Leptos components on a 24px grid with 2px strokes and `currentColor`: house, file-plus, files, specimen jar, user-plus, log-out, search, print, edit (Lucide-derived), plus custom Citología (squamous cell) and Histopatología (slide). Claude places them in the sidebar nav and next to study types in lists.
- SVG sources and an Inkscape/ImageMagick export script are committed so the PNG and ICO files can be regenerated.

**Comportamiento observable:** The browser tab shows mark 1 instead of the Leptos icon. The sidebar shows the brand mark, and each nav link has an icon. Study rows and forms mark Citología vs Histopatología with their icons. Primary buttons are violet instead of blue.

**Non-goals:** illustrations and empty states, print letterhead, renaming the app, dark mode, installable/offline web app behavior (the manifest only declares icons), adding an icon crate dependency, and using pink anywhere in the UI.

**Criterios de aceptación:**
- The tab shows mark 1 on light and dark browser chrome.
- Every favicon and touch-icon size is served from `public/` and referenced in `shell()`.
- Every listed icon exists as a component that takes `currentColor` and a class.
- Running the export script from the committed sources reproduces the files in `public/`.
- The nav and study-type icons are placed in the UI, and the blue accents now use the violet scale.

**Incertidumbres abiertas:**
- The app name is undecided. The mark doesn't depend on a letter, but the sidebar text and the manifest name say "Patologías" for now.
- The icon list is "for now" and may grow.
- Placing the icons and switching colors overlaps with your Tailwind lessons 3 and 6. You chose to have Claude do it here.
- Whether the doctor gets a say has not come up.
- The exact 50–900 steps of the violet scale aren't defined yet; only the base value is.
- Lucide's license (ISC) needs attribution if its paths are copied.

**Reference:** concept sheet (private) at https://claude.ai/artifact/XE6e5uQ3cGkfzCw9G4Nmn1, which holds the mark 1 geometry (32-unit grid) and the custom icon paths.

## Nodos afectados

_Pendiente — correr /opsx:explore add-brand-assets para consolidar el snapshot técnico._

## Archivos relevantes

_Pendiente — correr /opsx:explore add-brand-assets para consolidar el snapshot técnico._

## Contexto knowledge

_Pendiente — correr /opsx:explore add-brand-assets para consolidar el snapshot técnico._

## Riesgos identificados

_Pendiente — correr /opsx:explore add-brand-assets para consolidar el snapshot técnico._
