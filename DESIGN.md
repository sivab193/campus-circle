---
name: Campus Circle
description: A privacy-first student exchange presented as a tactile campus field guide.
colors:
  paper: "#f3efe4"
  ink: "#17251f"
  muted-ink: "#5d685f"
  coral: "#e65d42"
  field-green: "#2f6b56"
  trail-gold: "#dfb43d"
  rule: "#c9c4b6"
  card: "#fffdf7"
  board-hover: "#24362e"
  browse-paper: "#e8e3d6"
typography:
  display:
    fontFamily: "Newsreader, Georgia, serif"
    fontSize: "clamp(4rem, 8vw, 7.4rem)"
    fontWeight: 700
    lineHeight: 0.82
    letterSpacing: "-0.04em"
  headline:
    fontFamily: "Newsreader, Georgia, serif"
    fontSize: "clamp(2.6rem, 5vw, 5rem)"
    fontWeight: 700
    lineHeight: 0.95
  title:
    fontFamily: "Newsreader, Georgia, serif"
    fontSize: "25px"
    fontWeight: 700
    lineHeight: 1.05
  body:
    fontFamily: "DM Sans, Arial, sans-serif"
    fontSize: "16px"
    fontWeight: 400
    lineHeight: 1.5
  label:
    fontFamily: "DM Sans, Arial, sans-serif"
    fontSize: "11px"
    fontWeight: 700
    lineHeight: 1.2
    letterSpacing: "0.06em"
rounded:
  control: "12px"
  field: "10px"
  card: "14px"
  panel: "16px"
  chip: "20px"
  round: "999px"
spacing:
  xs: "6px"
  sm: "12px"
  md: "20px"
  lg: "40px"
  xl: "80px"
components:
  button-primary:
    backgroundColor: "{colors.ink}"
    textColor: "{colors.card}"
    rounded: "{rounded.control}"
    padding: "0 20px"
    height: "48px"
  button-secondary:
    backgroundColor: "transparent"
    textColor: "{colors.ink}"
    rounded: "{rounded.control}"
    padding: "0 20px"
    height: "48px"
  input:
    backgroundColor: "{colors.card}"
    textColor: "{colors.ink}"
    rounded: "{rounded.field}"
    padding: "12px"
  listing-card:
    backgroundColor: "{colors.card}"
    textColor: "{colors.ink}"
    rounded: "{rounded.card}"
    padding: "19px"
---

# Design System: Campus Circle

## Overview

**Creative North Star: "Pocket Field Guide"**

Campus Circle feels like an uncoated paper guide assembled by students who know the terrain: warm, candid, tactile, and immediately useful. Editorial display type and slightly imperfect paper-note gestures add humanity, while map ink, route numbering, and strict category markers make time-sensitive listings easy to navigate.

Trust is expressed through visible structure. A dark route board organizes the three paths, privacy guidance stays attached to actions, verification is explicit, and safety information is numbered instead of buried in marketing language. The interface should feel crafted, never institutional; it must not imply university endorsement.

**Key Characteristics:**

- Warm uncoated-paper surfaces with dark map-ink structure.
- Editorial Newsreader headlines paired with plainspoken DM Sans utility copy.
- Coral, green, and gold act as stable category and status wayfinding.
- Numbered routes, ranked freshness, index-like tabs, and compact labels accelerate scanning.
- Privacy and consent cues remain visible at decision points.
- Mobile layouts contain every board, card, control, and modal without horizontal spill.

## Colors

The palette resembles printed trail ink on warm paper: restrained neutrals carry most of the page, while three field colors orient people without becoming decoration.

### Primary

- **Map Ink:** The main text, border, active-control, route-board, footer, and high-contrast action color.

### Secondary

- **Signal Coral:** Housing wayfinding, freshness marks, important editorial emphasis, and safety highlights.
- **Field Green:** Marketplace wayfinding, verification states, and quiet trust cues.

### Tertiary

- **Trail Gold:** Ride wayfinding, selection color, and the universal keyboard focus outline.

### Neutral

- **Warm Paper:** The default canvas and control ground; it gives the product its uncoated stock character.
- **Card Stock:** Listing, note, and field surfaces that need separation without cold white.
- **Muted Ink:** Supporting copy and secondary labels.
- **Printed Rule:** Dividers and low-emphasis structure.
- **Browse Paper:** The slightly deeper listing-board section that separates discovery from the hero.

**The Wayfinding Trinity Rule.** Coral means housing, green means marketplace, and gold means rides or focus; preserve these associations and pair color with text or icons.

**The Ink-First Rule.** Most structure is dark ink on paper. Accent colors identify routes, freshness, and state; they do not wash entire screens.

## Typography

**Display Font:** Newsreader (with Georgia and serif fallbacks)  
**Body Font:** DM Sans (with Arial and sans-serif fallbacks)

**Character:** Newsreader supplies the voice of an annotated field journal: confident, compact, and slightly literary. DM Sans keeps filters, safety copy, forms, and actions neutral and fast to read.

### Hierarchy

- **Display** (700, fluid oversized scale, tightly set): Hero statements only; italic coral emphasis may mark the emotional turn.
- **Headline** (700, fluid large scale): Major section and safety headings.
- **Title** (700, compact): Listing names, route names, modal headings, and ranked guidance.
- **Body** (400–500, comfortable reading rhythm): Descriptions, consent language, and explanatory copy; keep measures near 590px in prominent passages.
- **Label** (700, tracked uppercase): Category tags, board headings, tape labels, freshness, and compact utility metadata.

**The Two-Voice Rule.** Use Newsreader for editorial hierarchy and DM Sans for every action, field, filter, and explanatory sentence.

## Layout

The page alternates between broad editorial fields and bounded utility boards. Desktop hero content uses an asymmetric two-column grid with the route board as the dense counterweight; listing discovery uses a three-column grid. The principal horizontal gutters are fluid, with a 24px minimum on larger layouts.

At 1000px, navigation collapses, the hero becomes one column, listings move to two columns, filters stack, and safety guidance becomes linear. At 650px, gutters tighten to 16–18px, listings become single-column, actions fill the available width, and the route board loses its rotation. Narrow-screen containment is mandatory: use `minmax(0, 1fr)`, `min-width: 0`, and width caps where dense content could force overflow.

Spacing is generous between editorial sections and compact inside wayfinding elements. Dense controls cluster around a 12–20px rhythm; major section boundaries use 60–120px breathing room.

**The Route-Before-Grid Rule.** Establish the three category paths before presenting the larger listing grid; downstream screens should preserve equally obvious category wayfinding.

## Elevation & Depth

The system is mostly flat and printed, using ink rules, tonal paper changes, and overlaps for structure. Shadows appear selectively beneath objects that behave like physical pieces placed on the page: the route board, founder note, modal, toast, and a listing card on hover. Small rotations and tape-like labels create tactile depth without glossy effects.

### Shadow Vocabulary

- **Paper Lift** (`0 12px 30px rgba(23,37,31,.12)`): Route board, interactive hover, and toast.
- **Pinned Note** (`0 11px 25px rgba(23,37,31,.1)`): Editorial founder note.
- **Modal Lift** (`0 24px 70px rgba(0,0,0,.35)`): Dialogs over the darkened backdrop.

**The Flat-Until-Handled Rule.** Listing cards rest as bordered paper; lift and a restrained rotation appear only on hover or for deliberately overlaid objects.

## Shapes

Controls and cards use gently rounded rectangular forms, generally 10–16px, while badges, index chips, report controls, route icons, and numbered safety markers are circular or pill-shaped. One irregular brand medallion, subtle board rotation, angled freshness tabs, and taped-note details provide the handmade character. Strong 1px ink borders keep every soft shape legible.

**The Bounded Paper Rule.** Every functional surface needs a clear edge through an ink border, a tonal shift, or a dark filled ground; avoid floating controls with ambiguous hit areas.

## Components

### Buttons

- **Shape:** Gently rounded, bordered, and substantial, with a 48px default minimum height.
- **Primary:** Dark map ink with card-stock text, 20px horizontal padding, and a low hard-edged base shadow.
- **Hover / Focus:** Primary actions lift 2px; all interactive elements use a 3px gold focus outline with a 3px offset.
- **Secondary:** Transparent paper with an ink border; hover fills with card stock.

### Chips

- **Style:** Metadata uses compact paper pills with 5px by 9px padding. Category tabs sit in a bordered paper tray with an 8px internal radius.
- **State:** The active tab reverses to map ink with light text. Selection never relies on color alone.

### Cards / Containers

- **Corner Style:** Listing cards use 14px corners; major dark boards and modals use 16px corners.
- **Background:** Card stock for listings and notes, warm paper for app chrome, and map ink for decisive route/navigation surfaces.
- **Shadow Strategy:** Bordered and flat at rest; listing cards lift only on hover.
- **Border:** A crisp 1px map-ink rule.
- **Internal Padding:** Typically 19–20px for card bodies, with tighter 12–15px category headers.

### Inputs / Fields

- **Style:** Card-stock fill, 1px ink border, 10–12px corners, and 12–14px internal padding.
- **Focus:** The global gold outline is preserved; search fields keep the input itself visually integrated into the bordered container.
- **Error / Disabled:** Errors use a pale coral ground with dark red text; development-only information uses a pale green ground.

### Navigation

The 78px sticky desktop bar uses a paper backdrop, bottom ink rule, compact DM Sans links, and a small dark medallion. On tablet and mobile, links open as a full-width paper panel below the 68px bar. The primary post action stays visible and compresses to its icon at narrow widths.

### Route Board

The signature dark board is a numbered, three-row directory. Each row pairs a two-digit rank, circular category icon, Newsreader route name, plain-language note, and directional arrow. A consent-focused privacy strip closes the board so disclosure expectations are present before browsing.

### Listing Card

Each card exposes category, verification, location, short description, two metadata chips, price, safe-contact action, and report affordance in a fixed scan order. The first-ranked results may receive an angled coral freshness tab; freshness is always stated in text.

## Do's and Don'ts

### Do:

- **Do** preserve the paper, ink, coral, green, and gold roles across new screens.
- **Do** keep category labels, icons, and text alongside color wayfinding.
- **Do** place privacy and consent language beside the action it governs.
- **Do** use numbered steps, ranks, and concise labels for time-sensitive scanning.
- **Do** test at 650px and below for contained boards, forms, cards, and long listing text.
- **Do** keep the student-run, unofficial disclaimer visually present where affiliation could be inferred.

### Don't:

- **Don't** use Purdue logos, official university styling, or language that implies endorsement.
- **Don't** replace the warm paper world with glossy gradients, glass panels, or generic blue SaaS surfaces.
- **Don't** hide contact-consent behavior behind vague trust claims.
- **Don't** rely on accent color alone to communicate category, verification, freshness, or selection.
- **Don't** add decorative accents without a wayfinding, trust, or editorial purpose.
