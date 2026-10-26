---
title: How scoring works
order: 5
product: agent-svgtools
relationships:
  references: agent-svgtools-comparison-engine
---

Every `judge` and `compare` result includes these comparison signals.

- **pixel_match** is mean per-channel agreement from 0 to 100 after
  compositing both images over white. In foreground scope, RGB agreement is
  multiplied by alpha agreement.
- **ssim** is an in-house mean Structural Similarity Index Measure over a
  Gaussian window. In foreground scope, luma SSIM is multiplied by alpha-plane
  SSIM.
- **combined** is the equally weighted mean of `pixel_match` and `ssim`.
- **scope** is `canvas` by default. `--foreground` evaluates only the union of
  pixels that are non-transparent in either image and incorporates alpha
  agreement.
- **regions** are ranked bounding boxes produced by tiling the frame, measuring
  per-block error, and merging contiguous high-error blocks. Coordinates are in
  reference-image pixels.

Foreground scope is usually the right choice for transparent icons because it
does not reward shared empty canvas. Canvas scope remains useful when the full
background is intentionally part of both images.

High pixel match with lower SSIM often points to structural or edge
misalignment; inspect a blend or side-by-side image. Lower pixel match with
higher SSIM often points to palette differences; inspect the reference colors.

Scores do not measure semantic naming, editability, component decomposition,
or whether a visually important local defect remains. Use `inspect`, visual
artifacts, and human or vision-model review alongside the numbers.
