# agent-trace

An open-source CLI that closes the feedback loop for AI coding agents authoring
SVG files from raster reference images (e.g. recreating a logo as SVG). Agents
write clean, semantic SVG but cannot *see* their output. `agent-trace` gives
them rendering, scoring, localized diffs, and vision-model-friendly comparison
images — all machine-first, so the output drops straight back into an
iterate-and-improve loop.

- **Pure Rust, single static binary**, no C dependencies.
- **Machine-first output**: TOON by default (token-efficient for LLMs), with
  JSON, JSONL, and pretty-printing on tap.
- **Scoring engine**: pixel match %, SSIM, a combined score, and ranked error
  regions with pixel-coordinate bounding boxes.
- **Artifacts**: diff heatmaps, labeled side-by-side panels, onion-skin blends,
  and coordinate grids — built for a vision model to read.

## Install

```bash
# crates.io
cargo install agent-trace

# npm (works via npx, no global install needed)
npx agent-trace --help

# Homebrew
brew install wyrd-company/tools/agent-trace
```

Prebuilt binaries for macOS (arm64/x64), Linux (x64/arm64, musl static), and
Windows (x64) are attached to every [release](https://github.com/wyrd-company/agent-trace/releases).

## The agent loop

1. `agent-trace colors <reference>` — read the exact hex values to target.
2. *(optional)* `agent-trace trace <reference> start.svg` — a starting-point SVG.
3. Author or revise `candidate.svg`.
4. `agent-trace judge <reference> candidate.svg --diff-image diff.png --grid`
   — score it and localize the errors.
5. Read the scores + `regions`, open `diff.png`, attack the worst region, and
   return to step 3. Stop when `combined` clears your target (or `--threshold`
   passes).

Run `agent-trace skill > SKILL.md` to install the full, agent-facing workflow
guide into a project.

## Quick start

```bash
# Score an SVG against a reference image (TOON by default).
agent-trace judge logo.png candidate.svg

# With artifacts and a coordinate grid for a vision model.
agent-trace judge logo.png candidate.svg \
  --diff-image diff.png --side-by-side sbs.png --grid

# CI gate: exit code 2 if the combined score is below 95.
agent-trace judge logo.png candidate.svg --threshold 95
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
| `trace <image> [output.svg]` | Raster→vector starting point via the embedded vtracer. |
| `colors <image>` | Dominant color palette: hex, coverage %, sample location. |
| `skill` | Print the agent-facing skill document (Markdown). |

`judge`/`compare` artifact flags: `--diff-image`, `--side-by-side`, `--blend`,
`--grid`, `--threshold <0-100>`, `--block`, `--max-regions`. Artifacts are only
ever written to the explicit paths you pass — never to implicit temp files.

See `agent-trace <command> --help` for the complete, agent-oriented reference
with exact output schemas.

## Output conventions

- **TOON by default** for all commands; `--format json`, `--format jsonl`, and
  `--pretty` (for JSON) are available. JSONL streams a header record followed by
  one record per list element (e.g. per diff region).
- **Exit codes**: `0` success, `1` usage/IO error, `2` threshold not met
  (`judge`/`compare`).

## How scoring works

- **pixel_match** — mean per-channel agreement (0–100) after compositing both
  images over white.
- **ssim** — an in-house mean-SSIM index (Wang et al. 2004) over a Gaussian
  window, implemented in pure Rust to keep the license permissive and the scores
  deterministic.
- **combined** — a weighted blend of the two; the single number to optimize.
- **regions** — the frame is tiled, per-block error is measured, and contiguous
  high-error blocks are merged into ranked bounding boxes in reference pixel
  coordinates.

Interpreting them: high pixel match + low SSIM usually means structural/edge
misalignment (check `--blend`); low pixel match + high SSIM means the shapes are
right but the colors are off (re-run `colors`).

## Contributing

Development, release, and architecture-decision guidance lives in
[CONTRIBUTING.md](CONTRIBUTING.md).
