# agent-svgtools

Rust CLI and library that renders, scores, and inspects SVGs against raster
references for AI agents. `src/main.rs` is a thin entry point; functionality
lives in the library (`src/lib.rs`), with one module per subcommand under
`src/commands/`.

## Where things are

- Agent skill: `.agents/skills/agent-svgtools/SKILL.md`. The binary embeds it
  (`agent-svgtools skill`), so a change to it ships with the next release.
- Contributor workflow, release steps, and public docs: `CONTRIBUTING.md`.
- Public docs: `docs/user/`, published to wyrd.foo.

## Checks

There is no Taskfile. CI and the lefthook pre-push hook run:

```bash
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features --workspace
```

## Releases

Intentional manages versions, `CHANGELOG.md`, and tags. A pull request with a
user-facing change adds an intent with `intentional add`. Do not edit
`CHANGELOG.md` or manifest versions by hand.

## TOON output policy

agent-svgtools output is ephemeral: agents read it in the moment, and nothing
stores or exchanges it. So the CLI tracks the current TOON spec and the latest
`toon-format` release. A TOON format change from a dependency update is
accepted. The same change updates the TOON examples (generated from real CLI
output) and keeps `tests/toon_roundtrip.rs` passing.
