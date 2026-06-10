# 0002 — Implement SSIM in-house

- Status: accepted
- Date: 2026-06-10

## Context

`judge`/`compare` need a structural similarity metric alongside pixel match.
The obvious crate is `dssim`, but it is AGPL-licensed, which is incompatible
with shipping a permissive (MIT/Apache-2.0) tool that others embed and
redistribute. The project also values deterministic, unit-testable scores so
the comparison engine can be regression-tested against known image pairs.

## Decision

Implement the Wang et al. (2004) mean-SSIM index in-tree (`compare::ssim`),
using a separable Gaussian window (sigma 1.5, 11 taps) for local statistics.
Do not take a GPL/AGPL dependency. Pixel match and region detection are
likewise implemented in-house.

## Consequences

- License stays clean (MIT OR Apache-2.0); no copyleft obligations propagate.
- Scores are deterministic and covered by unit tests (identical images → 100,
  black vs white → 0 pixel match, localized diffs → a single bounded region).
- We own the implementation and its performance characteristics (O(n) per blur
  pass), at the cost of maintaining it ourselves.
