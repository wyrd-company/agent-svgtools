---
version: alpha
name: agent-svgtools
description: >-
  The visual identity of agent-svgtools: an owl mark whose two eyes are the
  reference and the candidate, on the Wyrd Tools palette (near-black neutrals
  and one blue accent), set in Geist and Geist Mono. Colour tokens without a
  suffix are the dark theme; tokens with `-light` are the light theme.
colors:
  primary: "#6aa5ff"
  primary-light: "#0b4fa8"
  page: "#0b0b0c"
  page-light: "#fafafa"
  text: "#ededef"
  text-light: "#111113"
  text-2: "#a6a6af"
  text-2-light: "#4a4a52"
typography:
  wordmark:
    fontFamily: Geist Mono
    fontSize: 68px
    fontWeight: 700
    lineHeight: 1
    letterSpacing: -0.015em
  tagline:
    fontFamily: Geist
    fontSize: 36px
    fontWeight: 400
    lineHeight: 1.33
rounded:
  tile: 18.3px
  rule: 4px
spacing:
  mark: 96px
  stroke: 6px
  clear-space: 24px
  social-mark: 400px
  social-gutter: 70px
components:
  logo:
    backgroundColor: "{colors.page}"
    textColor: "{colors.text}"
    size: 96px
  logo-light:
    backgroundColor: "{colors.page-light}"
    textColor: "{colors.text-light}"
  favicon:
    backgroundColor: "{colors.page}"
    textColor: "{colors.text}"
    rounded: "{rounded.tile}"
    size: 128px
  favicon-small:
    backgroundColor: "{colors.page}"
    textColor: "{colors.text}"
    rounded: "{rounded.tile}"
    size: 16px
  social-image:
    backgroundColor: "{colors.page}"
    textColor: "{colors.text}"
    typography: "{typography.wordmark}"
    width: 1200px
    height: 630px
  social-image-light:
    backgroundColor: "{colors.page-light}"
    textColor: "{colors.text-light}"
  social-preview:
    backgroundColor: "{colors.page}"
    textColor: "{colors.text}"
    typography: "{typography.wordmark}"
    width: 1280px
    height: 640px
  social-preview-light:
    backgroundColor: "{colors.page-light}"
    textColor: "{colors.text-light}"
  tagline:
    backgroundColor: "{colors.page}"
    textColor: "{colors.text-2}"
    typography: "{typography.tagline}"
  tagline-light:
    backgroundColor: "{colors.page-light}"
    textColor: "{colors.text-2-light}"
  rule:
    backgroundColor: "{colors.primary}"
    rounded: "{rounded.rule}"
    width: 104px
    height: 8px
  rule-light:
    backgroundColor: "{colors.primary-light}"
---

# agent-svgtools design system

The SVG files in [`docs/user/assets/`](docs/user/assets/) are the source of
the identity. The tokens above name their values, and this document says how
to apply them. The palette and type are the ones in the
[wyrd.foo design system](https://wyrd.foo), so the product sits in the
catalog without a seam.

## Overview

agent-svgtools gives an agent the one thing it lacks when it writes SVG: sight.
The identity is an owl, because an owl is the animal that sees in the dark, and
it is drawn the way the tool works: from plain geometry, with nothing hidden.

- **The mark is an owl head.** One continuous outline makes the head and its
  two ear tufts. Two ringed eyes and a beak sit inside it.
- **The two eyes are the two images.** The left eye, in the ink colour, is the
  reference. The right eye, in the accent colour, is the candidate. The tool
  compares them, so the mark shows them side by side.
- **The beak is accent too.** It points at the place where the two disagree.
- **Dark is the default.** Every asset has a dark and a light form with the
  same structure and its own value for each colour.

## Colors

Each colour has a dark value and a light value. A token without a suffix is
the dark value; the `-light` token is the light value of the same role.

- **Primary (#6aa5ff dark, #0b4fa8 light):** The accent. It colours the
  candidate eye, the beak and the rule under the name. Nothing else carries
  an accent.
- **Page (#0b0b0c dark, #fafafa light):** The background of the favicon tile
  and of every social image.
- **Text (#ededef dark, #111113 light):** The ink. It draws the head outline
  and the reference eye, and sets the name.
- **Text 2 (#a6a6af dark, #4a4a52 light):** The tagline.

Every text and mark colour meets a 4.5:1 contrast ratio on the page colour of
its theme. The lowest pair is `primary-light` on `page-light`, at 7.5:1.

The logo uses two colours only: text and primary.

## Typography

Geist Mono sets the product name, as it does on wyrd.foo wherever a product
name is the subject. Geist sets the tagline.

| Token | Where |
| --- | --- |
| `wordmark` | `agent-svgtools` on the social images, in `text`, with a 4px-radius `rule` below it. |
| `tagline` | The line under the rule, in `text-2`, set on two lines. |

The name is always written in lower case, with the hyphen, as one word.

## Layout

The mark is a square with a 96-unit side. Keep a clear space of `clear-space`
(one quarter of the mark's side) around it on every side. Nothing else
enters that space.

On the social images the mark is on the left and the text block is on the
right, both centred on the vertical axis of the image:

- The mark is `social-mark` (400px) square, `social-gutter` (70px) from the
  left edge.
- The name, the rule, and the two tagline lines stand 50px to the right of the
  mark and left-align to one edge.
- A 1280x640 image has the same layout as a 1200x630 image, centred on the
  larger canvas.

## Elevation & Depth

The identity is flat. It has no shadow, gradient, glow or texture. Hierarchy
comes from the stroke weight and from the accent colour on the candidate eye.

## Shapes

The shape language is outline and circle: a `stroke` (6px in a 96-unit mark)
with round joins, and circles for both eyes.

- **Head.** One closed path. The ear tufts are straight lines with round
  joins. The cheeks and the chin are one curve.
- **Eyes.** A ring with a solid pupil. The ring is 5.5 units wide, a little
  lighter than the head outline, so the eyes sit inside the head and do not
  compete with it.
- **Beak.** A solid triangle with rounded corners.
- **Favicon tile.** A square with a corner radius of one seventh of its side
  (`tile`, 18.3px at 128px).
- **Favicon at 16px.** The head is solid and the eyes and the beak are cut out
  of it, because a 6-unit outline does not survive at that size. The right
  pupil stays in the accent colour.

## Components

The assets are the components. Each one has a dark and a light variant, except
the favicons, which are dark.

| Component | File | Size | Use |
| --- | --- | --- | --- |
| `logo-light` | `logo.svg` | square | The mark for a light page. `logo` in `docs.yml`. |
| `logo` | `logo-dark.svg` | square | The mark for a dark page. `logo-dark` in `docs.yml`. |
| `favicon` | `favicon.svg`, `favicon-32.png` | 128 and 32px | The mark on a dark tile. `favicon` in `docs.yml`. |
| `favicon-small` | `favicon-16.svg`, `favicon.ico` | 16px | The solid-head form for the smallest sizes. The `.ico` holds 16, 32 and 48px. |
| `favicon` | `apple-touch-icon.png` | 180px | The mark on a square tile with no rounded corners. The platform rounds it. |
| `social-image`, `social-image-light` | `social-image-dark.svg`, `social-image.svg` and `.png` | 1200x630 | The wyrd.foo social image. `social-image` in `docs.yml` is the light one. |
| `social-preview`, `social-preview-light` | `social-preview-dark.svg`, `social-preview.svg` and `.png` | 1280x640 | The GitHub repository social preview. |

The light files have no suffix and the dark files end in `-dark`. The tokens
use the opposite rule, as wyrd.foo does: the unsuffixed token is dark.

In `README.md` the logo is a `<picture>` element, so the reader sees the form
that matches their colour scheme.

## Do's and Don'ts

- Do draw the mark from the shipped SVG. Do not redraw it.
- Do keep the two eyes in two different colours. The difference is the idea.
- Do use the filled head below 24px, and the outlined head at 24px and above.
- Do keep the clear space around the mark.
- Do write the name in lower case with the hyphen.
- Don't add a third colour, a gradient or a shadow.
- Don't recolour the reference eye with the accent, or the candidate eye with
  the ink.
- Don't put the mark on a busy background. Use `page` or `page-light`.
- Don't stretch, rotate, outline or animate the mark.
- Don't set the name in any face other than Geist Mono.
