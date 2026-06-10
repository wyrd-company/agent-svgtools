# agent-trace

An open source CLI tool that closes the feedback loop for AI coding agents that author SVG files from raster reference images (e.g., recreating a logo as SVG). Agents are good at writing clean, semantic SVG but cannot verify their output visually. This tool gives them rendering, scoring, localized diffs, and vision-model-friendly comparison images.

# Prompt

Build an open source CLI tool in Rust called `agent-trace`.

The tool is machine-first: machine output by default, designed to be consumed by an agent in an iterate-and-improve loop.

## Distribution

- Single static binary, pure Rust, no C dependencies.
- Published to crates.io (`cargo install`), npm (runnable via `npx`), and Homebrew.
- Cross-platform: macOS (arm64/x64), Linux (x64/arm64), Windows (x64).

## Core dependencies

- `resvg` + `tiny-skia` for SVG rasterization.
- `vtracer` (the `visioncortex` library) vendored/embedded for raster→vector tracing.
- An image comparison approach supporting pixel diff and SSIM (implement or use a pure-Rust crate).
- Do NOT vendor potrace or autotrace (C, GPL). If useful, optionally shell out to them when found on PATH, clearly marked as optional.

## Subcommands

### `agent-trace judge <reference-image> <candidate.svg>`
The core command. Rasterizes the SVG at the reference image's resolution and compares.

Output (stdout):
- Overall similarity scores: pixel match %, SSIM, and a combined score.
- Localized diff report: a list of regions (bounding boxes in pixel coordinates) ranked by error magnitude, each with its local error score.
- Dimension/aspect-ratio mismatch warnings.
- Paths to any generated artifact images (see flags).

Output formats:

JSON - most obvious choice. Agents are trained on JSON, but it is token heavy. Consider JSONL
TOON - Token-Oriented Object Notation (TOON) is a compact, human-readable format designed for passing structured data to Large Language Models with significantly reduced token usage. Some benchmarks show increased accuracy + reduced token usage. `cargo add toon-format`
  - Consider if TOON should be considered default.

Flags:
- `--diff-image <path>`: write a diff heatmap image (pixel delta as color overlay).
- `--side-by-side <path>`: write a labeled side-by-side image (reference | rendered).
- `--blend <path>`: write a 50% opacity onion-skin overlay.
- `--grid`: add coordinate gridlines with axis labels to generated images, so vision models can map errors to actual SVG coordinates.
- `--threshold <0-100>`: exit non-zero if combined score is below threshold (CI-friendly).

### `agent-trace render <candidate.svg> <output.png>`
Plain rasterization. Flags for width/height/scale and background color.

### `agent-trace trace <image> [output.svg]`
Wraps the embedded vtracer. Expose vtracer's key tuning options (color mode, filter speckle, corner threshold, etc.). Output goes to file or stdout. Machine readable summary (path count, color count) to via `--json|toon`.

### `agent-trace colors <image>`
Extract the dominant color palette. Machine readable output: hex values, approximate pixel coverage %, and a representative location for each color. Flag `--max-colors <n>`.

### `agent-trace compare <image-a> <image-b>`
Same comparison engine as `judge` but for two raster images (useful when the agent has already rendered something itself).

### `agent-trace skill`
Inspired by Vercel's `agent-browser`. Prints a complete agent-facing skill document (markdown) to stdout describing:
- What the tool is for and the intended workflow loop:
  1. `colors` on the reference to get exact hex values.
  2. Optionally `trace` to get a starting-point SVG or to understand shapes.
  3. Author/revise SVG.
  4. `judge` with `--diff-image --grid` to score and localize errors.
  5. Read the JSON/JSONL|TOON + diff image, revise the SVG, repeat until score target met.
- Full subcommand reference with examples and example machine readable outputs.
- Tips for interpreting scores and diff regions (e.g., "low SSIM but high pixel match usually means structural/edge misalignment").

This document should be written *for an LLM agent as the reader*: imperative, example-heavy, no marketing language. Agents (or users) can run `agent-trace skill > SKILL.md` to install it into a project.

## Help text

All `--help` output is written for agents as the primary audience: terse, complete, with copy-pasteable examples and exact JSON|TOON schemas for outputs. Assume the reader will parse, not skim.

## Output conventions

- JSON/JSONL|TOON to stdout by default for all commands; `--pretty` for human-readable output if JSON.
- Exit codes: 0 success, 1 usage/IO error, 2 threshold failure (judge).
- All generated image artifacts are written to explicit paths given by flags — never implicit temp files — so the agent always knows where to look.

## CI/CD (GitHub Actions)

Create two workflows:

### `.github/workflows/ci.yml`
Runs on pull requests and pushes to the default branch:
- `cargo fmt --check`
- `cargo clippy -- -D warnings`
- `cargo test` on a matrix of macOS, Linux, and Windows.
- Build release binaries on the matrix to catch platform-specific build breaks early.

### `.github/workflows/cd.yml`
Triggers on pushing a SemVer tag. Tags are bare SemVer with **no `v` prefix** (e.g., `1.2.3`, not `v1.2.3`). Trigger pattern should match `[0-9]+.[0-9]+.[0-9]+*` to allow pre-release suffixes.

Steps:
1. **Verify** the tag matches the version in `Cargo.toml`; fail fast if not.
2. **Build** release binaries for all supported targets (macOS arm64/x64, Linux x64/arm64 — musl static, Windows x64). Produce checksummed archives (`.tar.gz` for unix, `.zip` for Windows) and upload as workflow artifacts under an `artifacts/` root.
3. **Create a GitHub Release** for the tag with the archives and checksums attached.
4. **Publish to crates.io** (`cargo publish`, token via `secrets.CARGO_REGISTRY_TOKEN`).
5. **Publish to npm**: an npm package that downloads/wraps the correct platform binary (postinstall or `optionalDependencies` per-platform packages pattern), published with `secrets.NPM_TOKEN`. Must work via `npx` without global install.
6. **Publish to the Homebrew tap** using this job pattern (adapt repository/script paths but keep the structure, env var names, and secret name exactly):

```yaml
      - name: Checkout tap repository
        uses: actions/checkout@v4
        with:
          repository: wyrd-company/homebrew-tools
          path: homebrew-tools
          ssh-key: ${{ secrets.FORMULAE_PUBLISH_KEY }}
      - name: Generate, commit, and push formula
        env:
          ARTIFACT_ROOT: artifacts
          TAG: ${{ steps.version.outputs.tag }}
          VERSION: ${{ steps.version.outputs.version }}
          REPOSITORY: ${{ github.repository }}
          TAP_DIR: homebrew-tools
        run: ./scripts/publish-homebrew-tap.sh
```

   This requires: a `version` step earlier in the job that exposes `tag` and `version` outputs parsed from the pushed tag, built artifacts available under `artifacts/`, and a `scripts/publish-homebrew-tap.sh` script in the repo that generates the formula (with per-platform URLs and sha256s pointing at the GitHub Release assets), commits it to the tap checkout, and pushes.

Publish steps should run only after the build and release steps succeed, so the formula and npm package never point at assets that don't exist.

## Non-goals (v1)

- No structural SVG linting (possible v2).
- No shape/geometry detection (possible v2).
- No GUI, no watch mode, no server.

## Quality bar

- Unit tests for the comparison engine (known image pairs with expected scores).
- Snapshot tests for JSON/JSONL|TOON output schemas.
- An end-to-end test: trace a sample logo, judge it against the original, assert score above a floor.
- README with the same agent-loop documentation as `skill`, plus install instructions for all three channels.