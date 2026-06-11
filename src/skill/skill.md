# Skill: agent-svgtools

You are authoring an SVG that should reproduce a raster reference image (e.g.
recreating a logo). You cannot see your own output. `agent-svgtools` is the
feedback loop: it renders your SVG, scores it against the reference, tells you
*where* it is wrong, and produces annotated images you can look at.

Work in a loop. Do not guess twice in a row without scoring in between.

## The loop

1. `agent-svgtools colors <reference>` — read the exact hex values you must hit.
2. (optional) `agent-svgtools trace <reference> start.svg` — get a starting-point
   SVG or just to understand the shapes. Treat it as scaffolding, not a final
   answer; it is verbose.
3. Author or revise `candidate.svg`.
4. `agent-svgtools judge <reference> candidate.svg --diff-image diff.png --grid`
   — score it and localize the errors.
5. Read the scores and the `regions` list. Open `diff.png`. Revise the SVG to
   attack the highest-error region first. Go back to step 3.
6. Stop when `combined` clears your target (e.g. 95) or `--threshold` passes.

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
- `combined` — the single number to optimize (0–100).
- `regions` — bounding boxes in reference pixel coordinates, worst first, each
  with a local `error` (0–100). Fix the top region first.
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
