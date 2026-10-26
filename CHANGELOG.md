# Changelog

## 0.2.1

### Fixes

- Publish the npm package under the `@wyrd-company` scope. Run it with
  `npx @wyrd-company/agent-svgtools` or install it with
  `npm install --global @wyrd-company/agent-svgtools`.

## 0.2.0

### Breaking

- Building from source now requires Rust 1.88 or later.

### Features

- Add the `inspect` command. It audits an SVG's structure, authored palette,
  component naming, and external-resource safety without rendering it.
- Add the `--foreground` option to `compare` and `judge`. It scores only pixels
  that are not transparent in both images, so shared empty canvas no longer
  raises the score. Reports now include a `score_scope` field with the value
  `canvas` or `foreground`.
- Replace the embedded agent skill with a component-first SVG reconstruction
  workflow. `agent-svgtools skill` prints it.
- Publish the crate to crates.io.

## 0.1.0

### Features

- Initial release. Render an SVG candidate and score it against a raster
  reference with `judge`, using pixel match, SSIM, a combined score, and ranked
  error regions.
- Add the `render`, `trace`, `colors`, and `compare` commands, and the `skill`
  command that prints the agent-facing workflow guide.
- Write diff heatmaps, side-by-side panels, onion-skin blends, and coordinate
  grids for vision models.
- Print results as TOON by default, or as JSON or JSONL.
- Attach prebuilt binaries for macOS, Linux, and Windows to the GitHub release.
