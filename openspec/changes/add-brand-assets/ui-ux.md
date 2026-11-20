# UI/UX: add-brand-assets

**Estado:** Aplica

**Justificación si "No aplica":** —

## Componentes y flujos

No new flows or states: every change is visual and static. Loading, error and empty states of the
affected screens stay as they are.

| Componente | Cambio | Estados |
|---|---|---|
| Browser tab / bookmarks / home-screen shortcut | Shows the field-of-view mark instead of the Leptos icon | — |
| Sidebar (`App`) | Brand lockup at the top; an icon before each nav label | Link hover: icon follows the text color |
| `LogoutUser` | Log-out icon before its label | Unchanged |
| `ReportTable` "Tipo" column | Study-type icon before the type text | — |
| `ShowParent` (single report) | Study-type icon next to "Tipo" | — |
| `ReportHeader` type selector | Study-type icon beside the `<select>`, updates when the choice changes | Citología (default) ↔ Histopatología |
| Primary buttons (login, create user, create report) | Blue → violet | Rest / hover / active keep their three-step ladder |

## Wireframes / mockups

Visual reference: the concept sheet (private) at https://claude.ai/artifact/XE6e5uQ3cGkfzCw9G4Nmn1,
"Mark and icons together" section, H&E palette.

Sidebar:

```
┌──────────────────────┐
│ (◉) Patologías        │  ← BrandMark + app name
│                       │
│ ⌂  Home               │
│ ⊞  New Report         │  icons 18–20 px, inherit link color,
│ ▤  Reports            │  aligned to the label's text center
│ ⚗  Entities           │
│ ⊕  Create User        │
│                       │
│        (flex spacer)  │
│ username / full name  │
│ ⇥  Logout             │
└──────────────────────┘
```

Reports list, type column:

```
 Protocolo   Fecha        Paciente        Tipo
 24-0815     2024-03-02   …               ⬡ Citologia
 24-0816     2024-03-02   …               ▭ Histopatologia
```

New report, type selector:

```
 Tipo
 [ Citopatologia      ▾ ]  ⬡      ← icon swaps to ▭ when Histopatologia is chosen
```

Primary button: violet fill (`hema-700`), white text, darker on hover and press.

## Accesibilidad

| Aspecto | Decisión |
|---|---|
| Roles ARIA | All icons are decorative: `aria-hidden="true"`, no `role`. Every icon sits next to visible text that already names the action or type, so screen readers read the text once. `BrandMark` in the sidebar is also hidden; the "Patologías" text names it. |
| Navegación por teclado | Unchanged. Icons are inside the existing links and buttons, never focusable on their own, so tab order doesn't change. |
| Foco visible | Unchanged: links and buttons keep the browser's default focus outline. Adding a violet focus ring is out of scope for this change. |
| Contraste | White text on `hema-700` and above must reach ≥ 4.5:1 (body-text threshold), checked when the scale is set. Icons inherit their label's color, so they match its contrast. The 16 px mark keeps its own violet disc as background, so it reads on light and dark browser chrome. |
| Lectores de pantalla | Study type is still announced through its text ("Citologia" / "Histopatologia"); the icon adds a visual cue only, never replaces the text. |
| Color como único canal | The two study types differ by icon shape (polygon cell vs. rectangle slide), not only by color, so they stay distinguishable for color-blind users. |
