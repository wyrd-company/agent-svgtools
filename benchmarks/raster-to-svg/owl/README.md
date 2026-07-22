---
relationships:
  evaluates: agent-svgtools-skill
  uses: owl-demo-target
  evidence: results
---

# Owl raster-to-SVG benchmark

This capsule preserves six final SVG specimens and normalized measurements from
the owl reconstruction evaluation. It tests whether the agent-facing skill
improves both visual reconstruction and the qualities a score cannot see:
semantic editability, bounded complexity, palette discipline, and vector
safety.

The target is [`demo/targets/owl.png`](../../../demo/targets/owl.png). Its
source page identifies the artwork as public domain. The specimen SVGs in
[`runs/`](runs/) are byte-for-byte copies of the agents' final outputs; their
provenance and relationships are recorded in [`results.json`](results.json)
rather than injected into the evidence files.

## Common prompt

> Recreate the provided owl raster as a clean, maintainable SVG. Use
> agent-svgtools to evaluate and improve the result. Automatic trace may be
> used only as a disposable diagnostic on an isolated crop or binary
> silhouette; do not copy, simplify, or edit traced geometry into the final
> SVG. Save owl.svg and a brief worklog.md listing the commands you ran and
> your final assessment. You may create any other artifacts useful to your
> workflow. Visually verify the finished SVG.

The no-skill condition received the same task prompt and was explicitly told
that `agent-svgtools` was available. It did not receive the contents of
`agent-svgtools skill`. The skill condition received the same prompt plus the
canonical skill.

## Normalized results

Every score and structural count below was recomputed with the same pre-release
build containing alpha-aware foreground scoring and the current `inspect`
command.

| Harness and condition | Runs | Foreground combined | Requirements passed | Named components | Colors |
| --- | ---: | ---: | ---: | ---: | ---: |
| Codex, with skill | 2 | 71.56, 69.76 | 8/8, 8/8 | 35, 37 | 12, 12 |
| Codex, without skill | 2 | 72.12, 77.01 | 4/8, 4/8 | 1, 1 | 29, 19 |
| Pi/Kimi K3, with skill | 1 | 58.81 | not normalized | 29 | 18 |
| Pi/Kimi K3, without skill | 1 | 69.02 | not normalized | 16 | 23 |

The two Codex conditions were visually close. A blind comparison called visual
fidelity a tie and preferred the skill-guided distribution 8.0 to 6.0 because
its SVGs were consistently semantic, compact, and palette-controlled. The
skill did not improve the mean foreground score: 70.66 with the skill versus
74.57 without it.

The single Kimi K3 pair points in the same direction but is not enough for a
distributional claim. The skill-guided run used 52 tool calls, 1.59 million
cumulative tokens, 29 minutes 43 seconds, and $1.28. The no-skill run used 84
tool calls, 4.70 million cumulative tokens, 47 minutes 51 seconds, and $3.32;
it required a neutral 45-minute timebox intervention. The no-skill SVG scored
higher visually, while the skill-guided SVG was more named and used a smaller
palette.

## Interpretation

This evidence supports a bounded claim: the component-first skill materially
improves structural discipline and efficient tool use without reliably
improving visual fidelity. `agent-svgtools` should be presented as a feedback
system that gets an agent close and exposes remaining defects, not as a
guarantee of a finished vector reconstruction. Human direction or a stronger
vision model may still be needed for subtle contours, gradients, proportions,
and asymmetry.

The sample is small, model identity was not exposed by the Codex harness, and
the Kimi pair has one run per condition. Treat this capsule as reproducible
evidence and an eval seed, not a general model leaderboard.

## Reproduce measurements

From the repository root, run the current binary against any preserved SVG:

```bash
cargo run -- --format json judge demo/targets/owl.png \
  benchmarks/raster-to-svg/owl/runs/codex-with-skill-1.svg --foreground

cargo run -- --format json inspect \
  benchmarks/raster-to-svg/owl/runs/codex-with-skill-1.svg
```

The complete scratch workspace, worklogs, grading records, comparison images,
and compressed raw Pi transcripts are archived outside Git at
`/workspaces/references/agent-svgtools/owl-reconstruction-benchmark-2026-07-22/`.
