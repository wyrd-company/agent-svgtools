<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="docs/user/assets/logo-dark.svg">
    <img src="docs/user/assets/logo.svg" alt="agent-svgtools logo" width="160">
  </picture>
</p>

# agent-svgtools

An open-source CLI that closes the feedback loop for AI coding agents authoring
SVG files from raster reference images (e.g. recreating a logo as SVG). Agents
write clean, semantic SVG but cannot *see* their output. `agent-svgtools` gives
them rendering, scoring, localized diffs, and vision-model-friendly comparison
images — all machine-first, so the output drops straight back into an
iterate-and-improve loop.

- **Pure Rust, single static binary**, no C dependencies.
- **Machine-first output**: TOON by default (token-efficient for LLMs), with
  JSON, JSONL, and pretty-printing on tap.
- **Scoring engine**: pixel match %, SSIM, a combined score, optional
  foreground-only scope for transparent artwork, and ranked error regions with
  pixel-coordinate bounding boxes.
- **Artifacts**: diff heatmaps, labeled side-by-side panels, onion-skin blends,
  and coordinate grids — built for a vision model to read.

## Install

```bash
# Homebrew
brew install wyrd-company/tools/agent-svgtools

# Cargo, from crates.io
cargo install agent-svgtools

# npx, without installing
npx @wyrd-company/agent-svgtools --help

# Cargo, the latest source
cargo install --git https://github.com/wyrd-company/agent-svgtools.git
```

Prebuilt binaries for macOS (arm64/x64), Linux (x64/arm64, musl static), and
Windows (x64) are attached to every
[release](https://github.com/wyrd-company/agent-svgtools/releases).

## The agent loop

1. `agent-svgtools colors <reference>` — read the exact hex values to target.
2. Decompose the reference into semantic components and author `candidate.svg`.
   Use `trace` only as a disposable diagnostic when the task explicitly permits
   it; do not promote traced geometry into the final SVG.
3. Run `agent-svgtools inspect candidate.svg` to audit palette, naming,
   complexity, and vector safety.
4. Run `agent-svgtools judge <reference> candidate.svg --foreground` to score
   transparent artwork without rewarding shared empty canvas, adding artifact
   flags when visual localization is useful.
5. Read the scores + `regions`, open `diff.png`, attack the worst region, and
   return to step 3. Stop when the major visible defects are resolved and the
   `inspect` budget is clean; the score is evidence rather than acceptance.

Run `agent-svgtools skill > SKILL.md` to install the full, agent-facing workflow
guide into a project.

## Demo

[![Agent reconstructing an owl with agent-svgtools](demo/owl-reconstruction/demo.gif)](demo/owl-reconstruction/README.md)

The [owl reconstruction walkthrough](demo/owl-reconstruction/README.md)
preserves the SVG from each real agent revision, recomputes its foreground
score during playback, and finishes with a structural audit of the final SVG.
The [owl benchmark capsule](benchmarks/raster-to-svg/owl/README.md) preserves
the skill and no-skill specimens, normalized measurements, and conclusions.
The logo above is one too: an agent drew it in SVG from the same owl and
checked every round with agent-svgtools. [`demo/logo/`](demo/logo/README.md)
keeps each version and how it was made.

## Quick start

```bash
# Score an SVG against a reference image (TOON by default).
agent-svgtools judge logo.png candidate.svg

# With artifacts and a coordinate grid for a vision model.
agent-svgtools judge logo.png candidate.svg \
  --diff-image diff.png --side-by-side sbs.png --grid --foreground

# CI gate: exit code 2 if the combined score is below 95.
agent-svgtools judge logo.png candidate.svg --threshold 95
```

Example `judge` output:

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
  scope: foreground
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

## Commands

| Command | Purpose |
| --- | --- |
| `judge <reference-image> <candidate.svg>` | Render the SVG at the reference resolution and score it (pixel match, SSIM, combined, regions). |
| `compare <image-a> <image-b>` | The same engine for two raster images. |
| `render <candidate.svg> <output.png>` | Plain rasterization (`--width`/`--height`/`--scale`/`--background`). |
| `trace <image> [output.svg]` | Disposable raster-to-vector diagnostic via embedded vtracer; output is not semantic final geometry. |
| `colors <image>` | Dominant color palette: hex, coverage %, sample location. |
| `inspect <candidate.svg>` | Audit viewBox, authored graphics, paths/commands, named components, authored colors, and external or embedded resources. |
| `skill` | Print the agent-facing skill document (Markdown). |

`inspect` reads unprefixed SVG attributes only from the no-namespace attribute.
Reference resolution prefers `href` and falls back to XLink `href` only when
`href` is absent. Other namespaces do not supply SVG attribute values. The
safety audit is conservative and ignores namespaces: it reports `on*`
attributes and non-fragment `href` values in any namespace, and scans every
attribute value for external URLs.

`judge`/`compare` artifact flags: `--diff-image`, `--side-by-side`, `--blend`,
`--grid`, `--foreground`, `--threshold <0-100>`, `--block`, `--max-regions`.
Artifacts are only ever written to the explicit paths you pass — never to
implicit temp files.

See `agent-svgtools <command> --help` for the complete, agent-oriented reference
with exact output schemas.

## Output conventions

- **TOON by default** for all commands; `--format json`, `--format jsonl`, and
  `--pretty` (for JSON) are available. JSONL streams a header record followed by
  one record per list element (e.g. per diff region).
- **Exit codes**: `0` success, `1` usage/IO error, `2` threshold not met
  (`judge`/`compare`).

## How scoring works

- **pixel_match** — mean per-channel agreement (0–100) after compositing both
  images over white. Foreground scope multiplies this by alpha agreement.
- **ssim** — an in-house mean-SSIM index (Wang et al. 2004) over a Gaussian
  window, implemented in pure Rust to keep the license permissive and the scores
  deterministic. Foreground scope multiplies luma SSIM by alpha-plane SSIM.
- **combined** — a weighted blend of the two; the single number to optimize.
- **scope** — `canvas` by default. Pass `--foreground` to score only the union
  of pixels that are non-transparent in either image and include alpha
  agreement, preventing shared empty canvas from inflating similarity or an
  opaque replacement background from receiving credit.
- **regions** — the frame is tiled, per-block error is measured, and contiguous
  high-error blocks are merged into ranked bounding boxes in reference pixel
  coordinates.

Interpreting them: high pixel match + low SSIM usually means structural/edge
misalignment (check `--blend`); low pixel match + high SSIM means the shapes are
right but the colors are off (re-run `colors`).
