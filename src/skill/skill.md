# Skill: agent-svgtools

You are authoring an SVG that should reproduce a raster reference image (e.g.
recreating a logo). You cannot see your own output. `agent-svgtools` is the
feedback loop: it renders your SVG, scores it against the reference, tells you
*where* it is wrong, and produces annotated images you can look at.

The goal is a clean, maintainable vector drawing that visually matches the
reference. Do not optimize for pixel score at the cost of unusable SVG. A high
score with jagged edges, stray paths, distorted shapes, or thousands of tiny
objects is a failed result.

Work systematically. Analyze first, decompose the image into meaningful parts,
author semantic SVG shapes, then judge and repair one component at a time. Do
not guess twice in a row without scoring in between.

## When this works best

This workflow is strongest for logos, icons, badges, marks, flat illustrations,
and simple product graphics. It is weak for photos, gradients, textures,
watercolor, shadows, and highly detailed artwork. For those, produce a simplified
vector interpretation unless the user explicitly asks for high-fidelity tracing.

Good targets:
- limited palette, usually 2-8 meaningful colors
- clear foreground/background separation
- shapes that can be described as circles, rectangles, polygons, curves, text,
  strokes, or layered filled regions
- enough resolution to see the real edges

Bad targets:
- photographic gradients or fine texture
- small antialiased text that should really be recreated from a font
- screenshots with compression artifacts
- output where the only way to match is embedding or approximating a bitmap

## The loop

1. Inspect the reference dimensions and palette:
   `agent-svgtools colors <reference> --max-colors 12`.
2. Decide whether the image is suitable for clean vector reconstruction. If it
   is not, explain that a simplified SVG is the realistic target.
3. Make a component plan before writing SVG: background, main silhouette,
   internal shapes, strokes, text, highlights, shadows. Record approximate
   bounding boxes from the image/grid when useful.
4. Optionally run `agent-svgtools trace <reference> trace.svg` to understand
   shape boundaries, but treat the trace as scaffolding. Do not accept it as the
   final SVG unless it is simple, readable, and visually clean.
5. Author `candidate.svg` from semantic SVG elements and paths. Prefer grouped,
   named components over one large opaque pile of traced paths.
6. Judge with visual artifacts:
   `agent-svgtools judge <reference> candidate.svg --side-by-side sbs.png --diff-image diff.png --blend blend.png --grid`.
7. Read both the numbers and the images. Fix the worst meaningful component,
   not merely the largest score delta. Re-run `judge` after each focused edit.
8. Stop when the SVG is visually acceptable, structurally clean, and the score
   supports that conclusion. A typical clean logo target is combined 95-98; 100
   is usually unrealistic unless you have the original vector or reproduce
   renderer quirks.

## Systematic conversion workflow

### 1. Prepare the reference

Use the highest-resolution source available. If a white/solid background is not
part of the mark, ignore it mentally when judging quality. If companion tools
such as ImageMagick are available, use them to create temporary work images:

```bash
# remove a near-white background for analysis
magick logo.png -fuzz 5% -transparent white logo-foreground.png

# trim surrounding whitespace
magick logo-foreground.png -trim logo-trimmed.png

# crop a problem component using a judge region
magick logo.png -crop 124x186+217+279 component.png
```

These prepared images are analysis aids. Keep the final SVG clean; do not embed
the raster unless explicitly asked.

### 2. Decompose before drawing

Create a short plan from the image:

```text
canvas: 500x500
background: white, not part of logo
components:
  body: teal curved rocket body, x 135-320, y 40-360
  fins: red left/right/base polygons
  window: red stroked circle with white fill
  flame: yellow fill with black outline, three lobes
palette:
  teal #0088aa
  red #fe0000
  yellow #fefe00
  black #010100
```

Use `colors` for exact fills, but interpret the palette. Antialiasing creates
near-duplicate colors; do not turn every sampled color into a separate SVG fill.

### 3. Author semantic SVG

Prefer simple SVG elements where they fit:
- `rect`, `circle`, `ellipse`, `line`, `polyline`, and `polygon` for geometric
  parts
- `path` for true curves and irregular silhouettes
- `g id="..."` for meaningful components
- reusable `fill`, `stroke`, `stroke-width`, `stroke-linejoin`, and
  `stroke-linecap` choices

Keep paths readable. A few good Bezier curves are better than hundreds of traced
micro-paths. If text is present, recreate it with an available font or convert
only the final text to paths when the visual shape matters more than editability.

### 4. Use tracing cautiously

Tracing is a starting point, not the product. It is useful for:
- identifying approximate boundaries
- estimating whether the reference is vector-friendly
- bootstrapping simple silhouettes
- comparing what an automatic trace loses

It is risky when it produces many colors/paths. Treat these as warning signs:
- path count above a few dozen for a simple logo
- color count far above the actual brand palette
- ragged edges, speckles, tiny slivers, or duplicated shadows
- a high score that still looks bad in `--side-by-side`

When the trace is noisy, use it as a reference and hand-author a cleaner SVG
instead of editing hundreds of generated paths.

### 5. Judge locally and visually

The global score can be misleading when the background is large. Always open the
side-by-side, diff, or blend artifact. A score above 98 can still be unusable if
the foreground has visible defects.

Use the report this way:
- `regions` points to where to look first.
- `--side-by-side --grid` shows whether component geometry is right.
- `--blend` shows misalignment and ghosting.
- `--diff-image` shows local color/edge disagreement.
- `colors` tells you whether a mismatch is palette or shape.

Repair one component at a time. For example: fix the rocket body silhouette,
then fins, then window, then flame. After each edit, rerun `judge` and compare
the new artifacts against the previous ones.

### 6. Preserve SVG quality

Before declaring success, inspect the SVG as code:
- named groups correspond to visual components
- no unexpected embedded image
- no large pile of tiny anonymous paths unless unavoidable
- palette is intentional and small
- dimensions/viewBox match the reference aspect ratio
- strokes and joins look deliberate
- the side-by-side artifact has no obvious visible aberrations

If the numeric score improves but the SVG becomes less maintainable or visually
worse, revert that edit and choose a semantic repair instead.

## Output format

Default output is TOON (compact, token-efficient). Add `--format json` for
standard JSON, `--format jsonl` for newline-delimited records, or `--pretty`
for indented JSON. All examples below show TOON unless noted.

Exit codes: `0` success, `1` usage/IO error, `2` threshold not met (`judge`).

## Commands

### judge — score an SVG against a reference

```
agent-svgtools judge logo.png candidate.svg
agent-svgtools judge logo.png candidate.svg --diff-image diff.png --grid
agent-svgtools judge logo.png candidate.svg --threshold 95   # exit 2 if below
```

Renders `candidate.svg` at the reference's pixel size, then scores. Example
output:

```toon
reference:
  path: logo.png
  width: 512
  height: 512
candidate: candidate.svg
rendered:
  width: 512
  height: 512
scores:
  pixel_match: 96.4
  ssim: 88.1
  combined: 92.25
warnings[0]:
regions[2]{x,y,width,height,error}:
  192,64,128,96,41.2
  0,416,96,96,33.7
artifacts:
  diff_image: diff.png
```

Reading it:
- `pixel_match` — per-channel color agreement (0–100).
- `ssim` — structural similarity (0–100). Sensitive to edges/shape.
- `combined` — a useful summary score (0–100), not the only success criterion.
- `regions` — bounding boxes in reference pixel coordinates, worst first, each
  with a local `error` (0–100). Inspect these visually before editing.
- `warnings` — dimension/aspect-ratio mismatches.

Artifact flags (all write to the explicit path you give — nothing implicit):
- `--diff-image <path>` — red heatmap of per-pixel error over a faded reference.
- `--side-by-side <path>` — labeled `reference | rendered` panels.
- `--blend <path>` — 50% onion-skin overlay; ghosting = misalignment.
- `--grid` — overlay coordinate gridlines + axis labels on the generated images
  so you can map a region box to actual SVG coordinates.
- `--block <px>` / `--max-regions <n>` — tune region granularity/count.

### compare — score two raster images

Same engine as `judge`, but both inputs are rasters (use when you have already
rendered something yourself). `image-b` is resized to `image-a` if needed.

```
agent-svgtools compare reference.png mine.png --diff-image diff.png
```

### render — rasterize an SVG to PNG

```
agent-svgtools render candidate.svg out.png --width 512
agent-svgtools render candidate.svg out.png --scale 2 --background white
```

Flags: `--width`, `--height`, `--scale`, `--background <color>`. With both
width and height the SVG is stretched to fill; with one, aspect is preserved.

### colors — extract the reference palette

```
agent-svgtools colors logo.png --max-colors 6
```

```toon
colors[3]{hex,rgb,coverage,location}:
  "#1d4ed8",[29,78,216],52.3,{x:120,y:96}
  "#ffffff",[255,255,255],38.0,{x:8,y:8}
  "#f59e0b",[245,158,11],9.7,{x:300,y:240}
```

Use `hex` values directly in `fill`/`stroke`. `coverage` is the share of the
image; `location` is a sample pixel of that color.

### trace — raster → vector starting point

```
agent-svgtools trace logo.png start.svg
agent-svgtools trace logo.png start.svg --coloring color --filter-speckle 4 --curve spline
```

Writes the SVG to the output path (or stdout if omitted) and prints a summary
(`path_count`, `color_count`). Key tuning flags: `--coloring color|binary`,
`--layer stacked|cutout`, `--curve pixel|polygon|spline`, `--filter-speckle`,
`--color-precision`, `--corner-threshold`, `--length-threshold`.

Prefer lower color precision and higher speckle filtering when you want a
simpler scaffold, but verify the visual result. If tuning collapses the mark or
creates a single blob, discard that trace.

### skill — print this document

```
agent-svgtools skill > SKILL.md
```

## Interpreting scores

- **High pixel match, low SSIM** — colors are close but structure/edges are
  misaligned or the wrong shapes. Look at `--blend`: ghosting reveals offsets.
- **Low pixel match, high SSIM** — shapes are right but colors/contrast are off.
  Re-run `colors` and correct your fills.
- **One dominant region** — a single element is wrong (missing shape, wrong
  position). Fix it before fine-tuning.
- **Many small regions along edges** — anti-aliasing/stroke-width differences;
  usually low priority unless your target score is very high.
- **A dimension/aspect warning** — your SVG `viewBox`/aspect does not match the
  reference; fix that first, since everything else is measured after stretching.
- **High combined score, visible defects** — the score is being diluted by easy
  pixels such as background. Trust the artifacts and fix the visible foreground
  defect.
