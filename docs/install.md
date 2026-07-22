---
title: Installation
order: 2
docs: true
install: true
relationships:
  references: README
---

Choose a released binary for the fastest install, or build the current source
with Cargo.

## Homebrew

On macOS or Linux:

```bash
brew install wyrd-company/tools/agent-svgtools
```

Or tap once and install by name afterward:

```bash
brew tap wyrd-company/tools
brew install agent-svgtools
```

## Build and install with Cargo

This installs the latest source from the repository and requires Rust 1.88 or
newer:

```bash
cargo install --git https://github.com/wyrd-company/agent-svgtools.git
```

For a local checkout:

```bash
git clone https://github.com/wyrd-company/agent-svgtools.git
cd agent-svgtools
cargo build --release
```

The local binary is written to `target/release/agent-svgtools`.

## Prebuilt release binaries

Each [GitHub release](https://github.com/wyrd-company/agent-svgtools/releases)
provides archives and SHA-256 checksum files for these targets:

| Platform | Target triple | Archive |
| --- | --- | --- |
| macOS (Apple Silicon) | `aarch64-apple-darwin` | `.tar.gz` |
| macOS (Intel) | `x86_64-apple-darwin` | `.tar.gz` |
| Linux x64 | `x86_64-unknown-linux-musl` | `.tar.gz` |
| Linux arm64 | `aarch64-unknown-linux-musl` | `.tar.gz` |
| Windows x64 | `x86_64-pc-windows-msvc` | `.zip` |

The Linux archives use musl static builds. Verify an archive against its
adjacent `.sha256` file before installing it.

## Verify the install

```bash
agent-svgtools --version
agent-svgtools --help
```

Every subcommand also supports `--help`.
