---
name: agent-svgtools
description: Reconstruct clean, maintainable SVG artwork from raster references by decomposing images into semantic components, plotting meaningful landmarks, authoring simple primitives and Bezier curves, and iterating with agent-svgtools render/judge artifacts. Use this skill whenever a user asks to convert, redraw, recreate, vectorize, or approximate a PNG, JPEG, WebP, GIF, or other raster image as SVG. Do not treat automatic tracing or a high similarity score as a finished vector.
compatibility: Requires agent-svgtools on PATH and an image-viewing tool.
relationships:
  references: README
---

# Clean Raster-to-SVG Reconstruction

Create a vector drawing, not a raster image inside an SVG wrapper. Balance three
independent qualities:

1. **Visual fidelity:** the rendering clearly matches the reference.
2. **Vector integrity:** geometry is intentional, with no embedded bitmap or
   automatically traced micro-paths.
3. **Maintainability:** components, layering, palette, landmarks, and curves are
   understandable to the next editor.

A similarity score is evidence, not acceptance. A concise SVG can be better than
a 400-path trace even when its global score is lower.

## Workflow

### 1. Inspect the actual image

View the reference with the environment's image tool; do not infer its contents
from the filename. Read its dimensions and dominant palette:

```bash
agent-svgtools colors reference.png --max-colors 12
```

Distinguish design colors from antialiased edge colors. Preserve transparency
unless the background is visibly part of the artwork.

For a flat illustration, declare a palette of at most 12 normalized six-digit
hex colors before drawing. Gradient stops count. Reuse the same literal for the
same semantic color; near-duplicates such as `#ffffff`, `#fefefe`, and `#fdfdfd`
are palette mistakes unless the difference is visible and intentional. Raise
the budget only when the reference visibly requires it and record why.

Decide whether the target supports clean reconstruction. Icons, logos, badges,
and flat illustrations are strong candidates. Ask whether a simplified vector
interpretation is acceptable for photography, texture, or painted effects.

### 2. Decompose before drawing

Make a short back-to-front component map. For each meaningful part, note:

- approximate bounding box
- layer relationship
- fill and stroke
- symmetry or attachment to another part
- simplest plausible element: primitive, polygon, or path

Prefer `rect`, `circle`, `ellipse`, and `polygon` when they fit. Use `path` for
a genuinely curved or irregular contour.

### 3. Plot meaningful landmarks

For each path, identify only points that explain its shape:

- extrema: top, bottom, left, and right
- corners and attachment points
- symmetry-axis crossings
- curvature transitions and inflection points
- points with clearly horizontal or vertical tangents

Do not sample pixels at regular intervals. Connect straight sections with lines
and smooth sections with the fewest Bezier segments that preserve the silhouette.
Place control handles along observed tangent directions. Maintain tangent
continuity across smooth joins.

Measure intentional asymmetry instead of cleaning it away. Record the center
axis and separate left/right bounds for paired components. Mirror with `<use>`
only after verifying that the reference is actually mirrored; otherwise author
the two contours separately. Do not make organic silhouettes rounder, smoother,
or more symmetrical merely because that is easier to express.

### 4. Author a semantic first pass

Match the reference aspect ratio with an explicit `viewBox`. Use a small
intentional palette and named components:

```xml
<g id="fins">...</g>
<path id="body" d="..." fill="#0891b2"/>
<g id="window">...</g>
```

Do not use `<image>`, data URLs, external resources, scripts, canvas, or raster
payloads. Capture component identity, proportion, and layer order before refining
edges.

Name every meaningful visible component or enclosing group. Do not add dummy
IDs to meet a count. Keep normal flat-illustration complexity below roughly 50
authored graphics, 32 authored paths, and 240 path commands unless the reference
has a clear semantic reason to exceed those bounds.

### 5. Close the visual feedback loop

Render and judge the first pass:

```bash
agent-svgtools render candidate.svg rendered.png
agent-svgtools judge reference.png candidate.svg \
  --side-by-side working-side-by-side.png \
  --blend working-blend.png \
  --diff-image working-diff.png \
  --grid \
  --foreground
```

Use `--foreground` for transparent artwork so shared empty canvas cannot inflate
the score. Foreground scoring also compares alpha, so replacing transparency
with an opaque background is penalized. The report states `scope: foreground`.
Use canvas scoring only when the full background is meaningful or when comparing
against an established canvas-scoped threshold.

Inspect every generated artifact. Use them for different questions:

- side-by-side: identity, proportion, palette, and layer order
- blend: positional ghosting and silhouette offsets
- diff and ranked regions: where to look next
- score: whether a focused edit moved the rendering overall

Generating an image without viewing it is not feedback.

### 6. Revise one named component at a time

Choose the largest meaningful visual defect and change only its responsible
component. Re-render and re-judge after that focused revision. Prefer this order:

1. canvas and foreground placement
2. primary silhouette
3. large attached shapes
4. internal negative space and details
5. strokes and small accents

Do not chase thin antialias-only error bands. Revert or rethink an edit that adds
complexity without a clear visual gain.

Use a first pass plus at most three focused revision cycles as a hard default.
A revision is one intentional change set after a judged first pass, even if it
requires several edits to that component. Stop after the third revision unless
the user explicitly asks for further refinement. Stop earlier when component
identity, layering, proportion, and major contours are visually sound. Stop when
successive edits change only tiny edge details, when a score moves without a
visible benefit, or when a revision regresses the foreground score.

## Conditional diagnostics

Do not create masks, ASCII pixel maps, exhaustive row/column scans, per-color
analyses, or crops before the first complete render and judge. They consume time
while encouraging pixel imitation instead of shape reconstruction. Escalate to
one focused crop only when the normal artifacts leave one material defect
ambiguous after a measured revision. Use no more than two diagnostic crops in a
normal task:

```bash
magick reference.png -crop WIDTHxHEIGHT+X+Y reference-region.png
magick rendered.png -crop WIDTHxHEIGHT+X+Y rendered-region.png
agent-svgtools compare reference-region.png rendered-region.png \
  --side-by-side region-side-by-side.png --diff-image region-diff.png \
  --foreground
```

The crop is temporary analysis material; the final SVG keeps the original canvas.

## Automatic trace policy

Do not run `agent-svgtools trace` unless the user or task explicitly permits it.
Tracing approximates raster pixels rather than semantic components and commonly
creates hundreds of paths and sampled edge colors.

When explicitly permitted, tracing is a disposable diagnostic only. Restrict it
to an isolated crop or binary silhouette when practical. Use it to study rough
bounds or contour behavior, then discard it. Never promote, simplify, or repair
traced geometry into the final SVG.

## Completion

Before handing off:

- render at target size and at a larger size
- inspect final side-by-side, blend, and diff images without a coordinate grid
- confirm viewBox, transparency, palette, layer order, and component coverage
- run `agent-svgtools inspect candidate.svg` and use its actual palette,
  structure, naming, and safety counts; do not estimate or self-report them from
  memory
- normalize accidental color duplicates, name every real component, and bring
  complexity within the declared budget before accepting the score
- confirm no embedded raster, external resource, abandoned path, or accidental
  color remains
- report the final foreground score, the `inspect` summary, and material
  remaining differences without presenting any score as the sole acceptance
  criterion

Keep a concise work log when requested: commands, scores, visual observations,
and the reason for each focused revision.
