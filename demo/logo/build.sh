#!/usr/bin/env bash
# Rebuilds the agent-svgtools brand assets from the owl mark.
# Needs node, agent-svgtools, ImageMagick, and the Geist and Geist Mono fonts installed.
set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$(git -C "$here" rev-parse --show-toplevel)"
assets="$root/docs/user/assets"
version="${1:-v1}"
marks="$here/$version"
scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT

render() { agent-svgtools render "$1" "$2" --width "$3" --format json >/dev/null; }

node "$here/gen.mjs" "$marks"
node "$here/assets.mjs" "$marks" "$assets" "$scratch"

for f in social-image social-image-dark; do render "$assets/$f.svg" "$assets/$f.png" 1200; done
for f in social-preview social-preview-dark; do render "$assets/$f.svg" "$assets/$f.png" 1280; done
render "$scratch/apple-touch-icon.svg" "$assets/apple-touch-icon.png" 180
render "$assets/favicon.svg" "$assets/favicon-32.png" 32
for s in 16 32 48; do render "$assets/favicon.svg" "$scratch/favicon-$s.png" "$s"; done
magick "$scratch/favicon-16.png" "$scratch/favicon-32.png" "$scratch/favicon-48.png" "$assets/favicon.ico"

agent-svgtools inspect "$marks/owl.svg"
