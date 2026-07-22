---
title: Command reference
order: 4
docs: true
relationships:
  references: README
---

| Command | Purpose |
| --- | --- |
| `judge <reference> <candidate.svg>` | Render an SVG at the reference resolution and report pixel match, structural similarity (SSIM), a combined score, and ranked error regions. |
| `compare <image-a> <image-b>` | Run the same comparison engine on two raster images. |
| `render <candidate.svg> <output.png>` | Rasterize an SVG, with optional width, height, scale, and background controls. |
| `colors <image>` | Report dominant colors with coverage and sample locations. |
| `inspect <candidate.svg>` | Audit viewBox, authored graphics, paths and commands, groups, reusable definitions, named components, authored colors, and vector safety. |
| `trace <image> [output.svg]` | Produce disposable vtracer diagnostic geometry; it is not semantic final geometry. |
| `skill` | Print the complete agent-facing reconstruction skill as Markdown. |

`judge` and `compare` accept `--foreground`, `--diff-image`,
`--side-by-side`, `--blend`, `--grid`, `--threshold <0-100>`, `--block`, and
`--max-regions`. Artifact files are written only to paths explicitly supplied
by the caller.

## Output conventions

- TOON is the default output format.
- `--format json`, `--format jsonl`, and `--pretty` are available globally and
  on subcommands.
- JSONL emits a header record followed by one record per list item where a
  command has list output.
- Exit code `0` means success, `1` means a usage or I/O failure, and `2` means a
  `judge` or `compare` threshold was not met.

Run `agent-svgtools <command> --help` for all arguments and exact wording.
