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

## Brand

[`DESIGN.md`](DESIGN.md) is the design system for the logo, favicons and social
images in `docs/user/assets/`. Their source is [`demo/logo/`](demo/logo/):
`gen.mjs` draws the owl and `assets.mjs` builds the assets from it. Do not edit
the files in `docs/user/assets/` by hand. Rebuild them, PNGs and `favicon.ico`
included:

```bash
demo/logo/build.sh v1
```

To revise the owl, edit `gen.mjs` and build the next version, for example
`demo/logo/build.sh v2`; see [`demo/logo/README.md`](demo/logo/README.md).

The build needs node, agent-svgtools and ImageMagick. The social images set
their text in Geist and Geist Mono, so install both before you build. Validate a
change to `DESIGN.md` with `npx @google/design.md lint DESIGN.md`.

## Releasing

[Intentional](https://wyrd.foo/docs/intentional/) is the only mechanism for
versions and releases. Do not edit the version in `Cargo.toml` or
`npm/package.json`, edit `CHANGELOG.md`, or create release tags by hand. The
configuration lives in `.intentional/config.yml`: the code unit
`agent-svgtools`, projected onto `Cargo.toml`, `npm/package.json` and
`docs/user/docs.yml` and tagged with bare SemVer (no `v` prefix, e.g. `1.2.3`),
and the docs unit `agent-svgtools-docs` (see [Public docs](#public-docs)).

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
4. **Apply.** `intentional apply` writes the changelog, bumps the manifests and
   `docs/user/docs.yml`, and consumes the intents. Run `cargo check` so
   `Cargo.lock` follows the new version, then commit it all as the only change
   in the release pull request.
5. **Tag.** After the release commit is on `main`,
   `intentional tag --plan release-plan.json` creates the annotated tag. Push it.

Pushing the tag runs the `cd` workflow. It verifies the tag matches
`Cargo.toml`, builds and checksums binaries for all targets, creates the GitHub
Release, then publishes to crates.io, npm, and the Homebrew tap.

## Public docs

The public docs are `docs/user/`: `docs.yml` and the Markdown pages, each with
`product: agent-svgtools` in its front matter. Everything under `docs/user/` is
published to [wyrd.foo](https://wyrd.foo/tools/agent-svgtools/); other `docs/`
folders are private. Validate a change from a wyrd.foo checkout:

```bash
node publisher/cli.ts validate <path to this checkout> --owner wyrd-company/agent-svgtools
```

Docs release on their own through the `agent-svgtools-docs` release unit
(`docs/user`, tagged `agent-svgtools-docs@{version}`), so a docs-only change adds
an intent for that unit:

```bash
intentional add --release-unit agent-svgtools-docs:patch \
  --message "Describe the docs change."
```

Intentional writes `version` and `docs-version` into `docs/user/docs.yml`; do
not edit them. The `Publish docs` workflow runs after the `cd` workflow
succeeds for a release tag, on a push of an `agent-svgtools-docs@*` tag, and by
manual dispatch. A plain push to `main` does not publish.
