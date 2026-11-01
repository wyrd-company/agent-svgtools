---
agent-svgtools: minor
---

Default TOON output follows TOON spec v4.4. Empty arrays print as `key: []` instead of `key[0]:`, and strings that start with `#`, such as hex colours, are quoted. A decoder for an earlier TOON spec cannot read the new output.
