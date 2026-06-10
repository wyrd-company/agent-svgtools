# 0003 — Render grid/labels via the SVG renderer, not a bitmap font

- Status: accepted
- Date: 2026-06-10

## Context

The comparison artifacts (`--grid`, `--side-by-side`) need text: coordinate
axis labels and panel captions. Two options: (a) embed a bitmap font and blit
glyphs, or (b) author the overlay (lines + `<text>`) as a small SVG and
rasterize it with the project's own renderer.

## Decision

Author overlays as SVG and rasterize them with `render::rasterize`, then
alpha-composite over the artifact. There is a single text/vector code path.

## Consequences

- No hand-maintained glyph tables; labels are crisp at any size where system
  fonts are available (CI runners on all three OSes ship fonts).
- Graceful degradation: where no font is present, gridlines and panels still
  render; only the text is absent — acceptable for the use case.
- Reuses and dogfoods the renderer that is already the heart of the tool.
