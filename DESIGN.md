---
version: alpha
name: agent-svgtools
description: >-
  The visual identity of agent-svgtools: a full-colour cartoon owl's head,
  drawn by an agent in SVG, set beside text on the Wyrd Tools palette
  (near-black neutrals and one blue accent) in Geist and Geist Mono. Colour
  tokens without a suffix are the dark theme; tokens with `-light` are the
  light theme. Tokens that start with `owl-` belong to the mark and do not
  change with the theme.
colors:
  primary: "#6aa5ff"
  primary-light: "#0b4fa8"
  page: "#0b0b0c"
  page-light: "#fafafa"
  text: "#ededef"
  text-light: "#111113"
  text-2: "#a6a6af"
  text-2-light: "#4a4a52"
  owl-outline: "#1e120e"
  owl-feather: "#7a4e40"
  owl-feather-shade: "#54342a"
  owl-disc: "#f3f1ea"
  owl-iris: "#f2700c"
  owl-pupil: "#0b0706"
  owl-beak: "#eaa823"
  owl-keyline: "#f3e9da"
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
  rule: 4px
spacing:
  mark: 310px
  outline: 6px
  keyline: 4px
  clear-space: 32px
  social-mark: 400px
  social-gutter: 70px
components:
  logo:
    backgroundColor: "{colors.page}"
    textColor: "{colors.owl-keyline}"
    size: 310px
  logo-light:
    backgroundColor: "{colors.page-light}"
    textColor: "{colors.owl-outline}"
  favicon:
    backgroundColor: "{colors.page}"
    textColor: "{colors.owl-keyline}"
    size: 32px
  favicon-on-light:
    backgroundColor: "{colors.page-light}"
    textColor: "{colors.owl-outline}"
    size: 16px
  owl-head:
    backgroundColor: "{colors.owl-feather}"
    textColor: "{colors.owl-keyline}"
  owl-head-shade:
    backgroundColor: "{colors.owl-feather-shade}"
    textColor: "{colors.owl-disc}"
  owl-face:
    backgroundColor: "{colors.owl-disc}"
    textColor: "{colors.owl-pupil}"
  owl-eye:
    backgroundColor: "{colors.owl-iris}"
    textColor: "{colors.owl-pupil}"
  owl-beak:
    backgroundColor: "{colors.owl-beak}"
    textColor: "{colors.owl-outline}"
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

The SVG files in [`docs/user/assets/`](docs/user/assets/) are the identity.
[`demo/logo/`](demo/logo/) is their source: the generator that draws the owl,
the script that builds the assets, and every version of the mark. The tokens
above name the values, and this document says how to apply them. The text
palette and type are the ones in the [wyrd.foo design system](https://wyrd.foo),
so the product sits in the catalog without a seam.

## Overview

agent-svgtools gives an agent the one thing it lacks when it writes SVG: sight.
The identity is an owl, the animal that sees in the dark, and the owl is also
proof of the claim: an agent drew it by hand in SVG and used agent-svgtools to
check every round.

- **The mark is an owl's head.** Two ear tufts, two cream facial discs, large
  orange eyes with a curious tilt, and a yellow beak, inside a dark outline.
- **The mark is revised over time.** Each version is drawn again with the
  current tool and kept in `demo/logo/`, so the logo shows what the tool helps
  an agent do today.
- **The text is quiet.** The name and tagline use the Wyrd neutrals and one
  blue accent, so the owl carries the colour.
- **Dark is the default.** Every asset has a dark and a light form.

## Colors

The `owl-` colours belong to the mark and are the same in both themes. The
others have a dark value and a light value: a token without a suffix is the
dark value, and the `-light` token is the light value of the same role.

- **Owl outline (#1e120e):** The outline of the head, and of the beak in the
  flat mark.
- **Owl feather (#7a4e40) and feather shade (#54342a):** The head and tufts,
  lit from the upper left.
- **Owl disc (#f3f1ea):** The facial discs. They shade to a warm grey at their
  lower outer edge.
- **Owl iris (#f2700c), pupil (#0b0706) and beak (#eaa823):** The face.
- **Owl keyline (#f3e9da):** A cream outline around the head in the dark
  forms, so the dark outline does not disappear on a dark page.
- **Primary (#6aa5ff dark, #0b4fa8 light):** The rule under the name on the
  social images. Nothing else carries the blue.
- **Page (#0b0b0c dark, #fafafa light):** The background of every social image.
- **Text (#ededef dark, #111113 light):** The name.
- **Text 2 (#a6a6af dark, #4a4a52 light):** The tagline.

Every text colour meets a 4.5:1 contrast ratio on the page colour of its theme.
The mark's outline or keyline meets 3:1 against the page it is designed for.

## Typography

Geist Mono sets the product name, as it does on wyrd.foo wherever a product
name is the subject. Geist sets the tagline.

| Token | Where |
| --- | --- |
| `wordmark` | `agent-svgtools` on the social images, in `text`, with a 4px-radius `rule` below it. |
| `tagline` | The line under the rule, in `text-2`, set on two lines. |

The name is always written in lower case, with the hyphen, as one word.

## Layout

The mark sits in a square with a 310-unit side. Keep a clear space of
`clear-space` (about one tenth of the side) around it. Nothing else enters
that space.

On the social images the mark is on the left and the text block is on the
right, both centred on the vertical axis of the image:

- The mark is `social-mark` (400px) square, `social-gutter` (70px) from the
  left edge.
- The name, the rule, and the two tagline lines stand 50px to the right of the
  mark and left-align to one edge.
- A 1280x640 image has the same layout as a 1200x630 image, centred on the
  larger canvas.

## Elevation & Depth

The full mark has depth: one light source from the upper left. The head and
discs use soft gradients, the discs have a soft shade under the brow, the
irises darken under the lid, and each eye and the beak carry one highlight
placed by that light. The text and backgrounds are flat, with no shadow or glow.

The flat mark has no gradients, shades or highlights other than the catchlight
in each eye.

## Shapes

The shape language is soft curves inside a firm outline.

- **Head.** One closed outline with two ear tufts that lean outward. The outline
  is `outline` (6 units) in the full mark and 12 units in the flat mark.
- **Facial discs.** Two cream shapes whose inner edges meet in a narrow brown
  bridge above the beak and open into the forehead.
- **Eyes.** An orange iris, a large pupil, and one catchlight to the upper left.
  The right eye is slightly larger and tilted, which gives the curious look.
- **Beak.** A short shield shape with a rounded top and a point.
- **Keyline.** In the dark forms, a cream band of `keyline` (4 units) outside
  the outline.

## Components

The assets are the components.

| Component | File | Size | Use |
| --- | --- | --- | --- |
| `owl-head`, `owl-head-shade`, `owl-face`, `owl-eye`, `owl-beak` | inside every mark | — | The parts of the owl, each with the colour drawn on top of it. |
| `logo-light` | `logo.svg` | square | The full mark for a light page. `logo` in `docs.yml`. |
| `logo` | `logo-dark.svg` | square | The full mark with keyline for a dark page. `logo-dark` in `docs.yml`. |
| `favicon`, `favicon-on-light` | `favicon.svg`, `favicon-16.svg`, `favicon-32.png`, `favicon.ico` | 32px and below | The flat mark with keyline. The keyline holds on light and dark tabs. `favicon` in `docs.yml`. The `.ico` holds 16, 32 and 48px. |
| `logo` | `apple-touch-icon.png` | 180px | The full mark with keyline on an opaque `page` square. The platform rounds it. |
| `social-image`, `social-image-light` | `social-image-dark.svg`, `social-image.svg` and `.png` | 1200x630 | The wyrd.foo social image. `social-image` in `docs.yml` is the light one. |
| `social-preview`, `social-preview-light` | `social-preview-dark.svg`, `social-preview.svg` and `.png` | 1280x640 | The GitHub repository social preview. |

The light files have no suffix and the dark files end in `-dark`. The tokens
use the opposite rule, as wyrd.foo does: the unsuffixed token is dark.

In `README.md` the logo is a `<picture>` element, so the reader sees the form
that matches their colour scheme.

## Do's and Don'ts

- Do build the assets from `demo/logo/` with `build.sh`. Do not edit the files
  in `docs/user/assets/` by hand.
- Do revise the mark as a new version in `demo/logo/`, and keep the earlier
  versions.
- Do use the full mark at 48px and above, and the flat mark below 48px.
- Do use the keyline forms on dark backgrounds.
- Do keep the clear space around the mark.
- Do write the name in lower case with the hyphen.
- Don't recolour the owl to match a theme. Its colours are the same everywhere.
- Don't use the owl's orange or yellow in text or rules.
- Don't put the mark on a busy background. Use `page` or `page-light`.
- Don't stretch, rotate or crop the mark.
- Don't set the name in any face other than Geist Mono.
