# Delta Spec: add-brand-assets

**Capability:** brand-identity
**Change:** add-brand-assets

## Purpose

Gives Patologías a consistent visual identity built on the H&E stain: the mark that identifies the app
in the browser, the single accent color of the UI, and the icon set that labels navigation and study
types.

## ADDED Requirements

### Requirement: Browser favicon shows the brand mark
The system SHALL identify itself in browser tabs, bookmarks and home-screen shortcuts with the
"field of view" mark (a hematoxylin-violet disc holding one eosin-pink squamous cell with a violet
nucleus), and SHALL NOT serve the default Leptos icon.

#### Scenario: Tab icon on a light browser
- **GIVEN** the app is running
- **WHEN** a user opens any page in a browser with light tab chrome
- **THEN** the tab shows the field-of-view mark at 16 px

#### Scenario: Tab icon on a dark browser
- **GIVEN** the app is running
- **WHEN** a user opens any page in a browser with dark tab chrome
- **THEN** the mark is still legible, because its violet disc carries its own background

#### Scenario: Legacy favicon request
- **GIVEN** a browser that ignores `<link rel="icon">` and requests `/favicon.ico` directly
- **WHEN** it fetches `/favicon.ico`
- **THEN** it receives the field-of-view mark, not the Leptos icon

### Requirement: Every declared icon asset is served
The system SHALL serve every icon file it declares in the document head and in the web manifest:
`favicon.svg`, `favicon.ico` (16/32/48), `apple-touch-icon.png` (180), `icon-192.png`, `icon-512.png`,
`icon-maskable-512.png` and `site.webmanifest`. The manifest MUST declare only names, icons and
colors, with no service worker or offline behavior.

#### Scenario: Head references resolve
- **GIVEN** any rendered page
- **WHEN** each icon, apple-touch-icon and manifest URL in its head is requested
- **THEN** each request returns the corresponding file

#### Scenario: Manifest icons resolve
- **GIVEN** the web manifest
- **WHEN** each icon listed in it is requested
- **THEN** each request returns a PNG of the declared size, and the maskable icon keeps the whole mark
  inside its safe zone

### Requirement: Raster assets are reproducible from committed sources
The project SHALL keep the SVG sources of the mark and an export script in the repository, and running
that script MUST regenerate every raster icon file served from `public/`.

#### Scenario: Regenerate after editing the mark
- **GIVEN** a developer edits the mark's SVG source
- **WHEN** they run the export script
- **THEN** every PNG and the ICO in `public/` are rewritten from that source

### Requirement: Single violet accent
The UI SHALL use one accent color scale derived from hematoxylin violet (`#4B2E83`) for primary actions,
selection and focus. Eosin pink MUST NOT appear in the UI outside the brand mark. Neutral surfaces and
status colors (success, error) SHALL keep their own colors.

#### Scenario: Primary button
- **GIVEN** a form with a primary submit button (login, create user, create report)
- **WHEN** the user views, hovers and presses it
- **THEN** its rest, hover and active states all use the violet scale, and none is blue

#### Scenario: Status messages keep their meaning
- **GIVEN** an error or success message is shown
- **WHEN** the user reads it
- **THEN** it keeps its red or green color and is not recolored violet or pink

### Requirement: Sidebar brand lockup
The sidebar SHALL show the brand lockup, the field-of-view mark next to the app name "Patologías",
above the navigation links.

#### Scenario: Any page
- **GIVEN** a user on any route
- **WHEN** the sidebar is visible
- **THEN** it shows the mark and "Patologías" at its top

### Requirement: Navigation links carry icons
Each sidebar navigation link SHALL show an icon before its label: Home (house), New Report
(file-plus), Reports (files), Entities (specimen jar), Create User (user-plus). The logout control
SHALL show the log-out icon. Icons MUST inherit the link's text color.

#### Scenario: Icons follow link color
- **GIVEN** the sidebar navigation
- **WHEN** a link's text color changes (for example on hover)
- **THEN** its icon changes color with it

### Requirement: Study types are distinguishable by icon
Wherever the app shows a study's type, it SHALL show the Citología icon (squamous cell) or the
Histopatología icon (glass slide) next to the type, so the two types can be told apart without
reading the text.

#### Scenario: Reports list
- **GIVEN** the reports list contains both cytology and histopathology studies
- **WHEN** the user scans the type column
- **THEN** each row shows the icon of its study type

#### Scenario: Report view
- **GIVEN** a user opens a single report
- **WHEN** the report's type is shown
- **THEN** the matching study-type icon appears with it

#### Scenario: New report type selector
- **GIVEN** a user creating a report
- **WHEN** they choose between Citología and Histopatología
- **THEN** the current choice is shown with its study-type icon

### Requirement: Icons are stylable components
Every icon in the set SHALL be available as a component that draws with the current text color and
accepts a class for size and color, and MUST render identically on the server and in the hydrated
client. The set is: house, file-plus, files, specimen jar, user-plus, log-out, search, print, edit,
Citología and Histopatología.

#### Scenario: Size and color from classes
- **GIVEN** an icon component used with size and text-color classes
- **WHEN** the page renders
- **THEN** the icon takes that size and color

#### Scenario: No hydration mismatch
- **GIVEN** a server-rendered page containing icons
- **WHEN** the client hydrates it
- **THEN** no hydration warning or error is raised for the icons
