---
title: Overview
order: 1
product: agent-svgtools
relationships:
  demonstrates: owl-reconstruction-demo
  references: README
---

`agent-svgtools` gives coding agents a measurable feedback loop for
reconstructing raster artwork as clean, maintainable SVG. It renders candidate
SVGs, compares them with a reference image, localizes visible differences, and
audits the vector structure that scores alone cannot evaluate.

![Agent reconstructing an owl with agent-svgtools](assets/demo.gif)

The recommended workflow is component-first: identify the silhouette and
semantic parts, author deliberate vector geometry, inspect the structure, then
use visual artifacts and scores to guide focused revisions. Automatic tracing
is a disposable diagnostic for an isolated shape when explicitly allowed; it
is not the source of final geometry.

- **Pure Rust executable** — self-contained, with no C runtime dependencies.
- **Machine-first output** — TOON by default, with JSON, JSONL, and optional
  pretty-printed JSON.
- **Structural inspection** — viewBox, authored graphics, path complexity,
  semantic IDs, palette size, and unsafe or external resources.
- **Alpha-aware scoring** — foreground scope excludes shared transparency and
  penalizes alpha disagreement.
- **Visual artifacts** — diff heatmaps, side-by-side panels, onion-skin blends,
  and coordinate grids, written only to explicit paths.

The tool gets an agent close and makes the remaining work observable. A person
or vision-capable agent still decides whether the result is visually finished;
the numeric score is evidence, not acceptance.
