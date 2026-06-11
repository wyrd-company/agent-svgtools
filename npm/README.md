# agent-trace (npm)

npm wrapper for [`agent-trace`](https://github.com/wyrd-company/agent-trace), a
machine-first CLI that closes the feedback loop for AI agents authoring SVGs
from raster references.

```bash
npx agent-trace judge logo.png candidate.svg
# or install it
npm i -g agent-trace
```

On install, a small postinstall script downloads the correct prebuilt binary
for your platform from the matching GitHub Release and verifies its checksum.
No native toolchain is required.

Supported: macOS (arm64/x64), Linux (x64/arm64, musl static), Windows (x64).

See the [main README](https://github.com/wyrd-company/agent-trace) and run
`agent-trace skill` for the full agent workflow guide.

Licensed MIT.
