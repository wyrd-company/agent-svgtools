# Contributing

## Development

```bash
cargo test                                      # unit + integration + e2e tests
cargo fmt --all --check
cargo clippy --all-targets -- -D warnings
cargo run -- judge ref.png candidate.svg        # run locally
```

The crate is organized as focused subsystems (`render`, `compare`, `palette`,
`tracing`, `artifacts`, `output`, `skill`) with one module per CLI subcommand
under `commands/`.

Architectural decisions that matter for a change are recorded as MADR blocks in
the commit message for that change.

## Releasing

Push a bare-SemVer tag (no `v` prefix, e.g. `1.2.3`). The `cd` workflow verifies
the tag matches `Cargo.toml`, builds and checksums binaries for all targets,
creates the GitHub Release, then publishes to crates.io, npm, and the Homebrew
tap.
