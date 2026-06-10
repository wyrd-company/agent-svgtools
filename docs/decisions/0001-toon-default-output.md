# 0001 — TOON as the default output format

- Status: accepted
- Date: 2026-06-10

## Context

The primary reader of every command's output is an LLM agent running a tight
iterate-and-score loop, where the scoring output is fed back into the model on
each turn. Output volume therefore translates directly into token cost and
context pressure. JSON is universal and well understood by models but is
token-heavy (repeated keys, quotes, braces). TOON (Token-Oriented Object
Notation) encodes the same structure in materially fewer tokens, and some
benchmarks report improved extraction accuracy alongside the token savings.

## Decision

Default to TOON for all machine-readable output. Keep JSON and JSONL as
first-class, explicitly selectable formats (`--format json|jsonl`), and keep
`--pretty` for indented JSON. No format is removed; only the default changes.

## Consequences

- Lower token cost for the common case (the agent loop) with no flag needed.
- Callers needing a universal parser opt in with `--format json` (one flag).
- JSONL remains available for streaming, line-delimited consumption (a header
  record followed by one record per list element, e.g. per diff region).
- We depend on the `toon-format` crate for encoding.
