# @wyrd-company/agent-svgtools

npm wrapper for [`agent-svgtools`](https://github.com/wyrd-company/agent-svgtools), a
machine-first CLI that closes the feedback loop for AI agents authoring SVGs
from raster references.

```bash
npx @wyrd-company/agent-svgtools judge logo.png candidate.svg
# or install it
npm install --global @wyrd-company/agent-svgtools
```

On install, a small postinstall script downloads the correct prebuilt binary
for your platform from the matching GitHub Release and verifies its checksum.
No native toolchain is required.

Supported: macOS (arm64/x64), Linux (x64/arm64, musl static), Windows (x64).

See the [main README](https://github.com/wyrd-company/agent-svgtools) and run
`agent-svgtools skill` for the full agent workflow guide.

Licensed MIT.
