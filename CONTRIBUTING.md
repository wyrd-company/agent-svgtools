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

[Intentional](https://wyrd.foo/docs/intentional/) is the only mechanism for
versions and releases. Do not edit the version in `Cargo.toml` or
`npm/package.json`, edit `CHANGELOG.md`, or create release tags by hand. The
configuration lives in `.intentional/config.yml`: one release unit,
`agent-svgtools`, projected onto `Cargo.toml` and `npm/package.json`, tagged
with bare SemVer (no `v` prefix, e.g. `1.2.3`).

1. **Intent.** Every pull request with a user-facing change adds an intent:

   ```bash
   intentional add --release-unit agent-svgtools:patch \
     --message "Describe the user-visible outcome."
   ```

   Use `major`, `minor`, or `patch`; before 1.0 the configured compatibility
   mapping applies. Commit the intent with the change.
2. **Status.** `intentional status` shows pending intents and the next version;
   `intentional check` validates configuration, intents, and tag history.
3. **Plan.** `intentional plan > release-plan.json` saves the digest-bound plan.
4. **Apply.** `intentional apply` writes the changelog, bumps both manifests, and
   consumes the intents. Run `cargo check` so `Cargo.lock` follows the new
   version, then commit it all as the only change in the release pull request.
5. **Tag.** After the release commit is on `main`,
   `intentional tag --plan release-plan.json` creates the annotated tag. Push it.

Pushing the tag runs the `cd` workflow. It verifies the tag matches
`Cargo.toml`, builds and checksums binaries for all targets, creates the GitHub
Release, then publishes to crates.io, npm, and the Homebrew tap.
