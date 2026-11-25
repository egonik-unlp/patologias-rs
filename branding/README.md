# Branding

Sources for the Patologías mark and favicon set. Generated files live in `public/`; edit the sources
here, never the generated files.

## The mark

A microscope field of view (the round disc) holding one squamous cell, the cell a pathologist sees
most in a Pap smear. It's colored the way H&E stains tissue: pink (eosin) cytoplasm, violet
(hematoxylin) nucleus.

| File | Use |
|---|---|
| `mark.svg` | Master mark, 32-unit grid. Also served as `public/favicon.svg` |
| `mark-maskable.svg` | Full-bleed variant for the maskable manifest icon and the Apple touch icon |

The same geometry is drawn in Rust as `BrandMark` in `src/frontend/icons.rs`. **Change both
together.**

## The empty field (404)

`empty-field.svg` is the brand mark's field of view with the cell missing: only a dashed ghost of
where it should be, a few specks and the eyepiece's measuring scale. It's the 404 illustration, drawn
in Rust as `EmptyField` in `src/frontend/icons.rs` (change both together). It uses hema shades only,
no eosin, because it's UI, not the mark.

## Palette

Defined as Tailwind theme colors in `style/tailwind.css` (`@theme`).

| Token | Value | Use |
|---|---|---|
| `hema-50 … hema-950` | violet scale, `hema-900` = `#4B2E83` | The app's single accent: primary buttons, selection, focus |
| `eosin` | `#F4BCCB` | **Only inside the brand mark.** Never in the UI: pink reads close to error red |

Primary buttons use `bg-hema-700 hover:bg-hema-800 active:bg-hema-900` with white text (contrast
7.5:1 or higher). Neutrals stay slate; success/error colors stay green/red.

## Regenerating the favicon set

```bash
./scripts/export-brand-assets.sh
```

Needs Inkscape 1.x and ImageMagick 7. It writes `favicon.ico` (16/32/48), `favicon.svg`,
`apple-touch-icon.png` (180), `icon-192.png`, `icon-512.png` and `icon-maskable-512.png` into
`public/`. `public/site.webmanifest` is written by hand; update it only if names or colors change.

## UI icons

`src/frontend/icons.rs` holds the icon components: 24 px grid, 2 px stroke, `currentColor`, each
taking a `class` (e.g. `<IconHouse class="size-5" />`). All are decorative (`aria-hidden`) and must sit
next to text that names them.

- From **Lucide v0.460.0** (lucide-static, ISC license, notice in `icons.rs`): house, file-plus, files,
  user-plus, log-out, search, printer (`IconPrint`), pencil (`IconEdit`), trash-2 (`IconTrash`),
  copy (`IconCopy`), arrow-left (`IconArrowLeft`).
- Original to this project: `IconSpecimenJar`, `IconCitologia` (squamous cell),
  `IconHistopatologia` (glass slide with a tissue section).

New icons should keep the same grid and stroke so they sit in the same family.
