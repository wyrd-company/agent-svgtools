---
title: The reconstruction workflow
order: 3
docs: true
relationships:
  uses: agent-svgtools-skill
  demonstrates: owl-reconstruction-demo
---

Start by installing the complete agent-facing workflow in the working project:

```bash
agent-svgtools skill > SKILL.md
```

Then use a component-first reconstruction loop.

1. Inspect the reference and run `agent-svgtools colors <reference>`.
2. Write down the silhouette, major semantic components, layer order, palette,
   and a complexity budget.
3. Author a complete SVG from deliberate primitives and Bézier curves.
4. Run `agent-svgtools inspect candidate.svg` and correct structural,
   naming, palette, or resource-safety problems.
5. Render and visually inspect the candidate.
6. Run a foreground-scoped judge with comparison artifacts.
7. Revise one named defect, rerun the audit and comparison, and keep the
   revision only when the evidence improves.

```bash
agent-svgtools colors reference.png
agent-svgtools inspect candidate.svg
agent-svgtools render candidate.svg rendered.png
agent-svgtools judge reference.png candidate.svg --foreground \
  --side-by-side side-by-side.png --diff-image diff.png --blend blend.png
```

Foreground scope scores the union of pixels that are non-transparent in either
image and includes alpha agreement. This prevents a large shared transparent
canvas—or an opaque replacement background—from receiving similarity credit.

Use `trace` only when the task explicitly permits it, only on an isolated crop
or binary silhouette, and only as diagnostic evidence. Do not copy, simplify,
or edit traced geometry into the final SVG.

Stop when the major visible defects are resolved, the SVG is structurally
clean and editable, and further score changes no longer represent meaningful
visual improvement. Expect a human or vision-capable agent to direct or finish
the last subtle edits.
