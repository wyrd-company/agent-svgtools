#!/usr/bin/env bash
# ---
# relationships:
#   demonstrates: README
# ---

set -euo pipefail

script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
repo_root=$(cd -- "$script_dir/../.." && pwd)
temporary_dir=$(mktemp -d)
font_file=$(fc-match -f '%{file}\n' sans | sed -n '1p')
trap 'rm -rf -- "$temporary_dir"; rm -f -- /tmp/agent-svgtools-owl-0{1,2,3,4}.gif' EXIT

cd "$repo_root"
cargo build --quiet
export PATH="$repo_root/target/debug:$PATH"

for phase in 01-first-pass 02-revision-1 03-revision-2 04-inspect; do
  vhs "$script_dir/$phase.tape"
  gifsicle --unoptimize "/tmp/agent-svgtools-owl-${phase%%-*}.gif" \
    > "$temporary_dir/$phase.gif"
done

reference="$repo_root/demo/targets/owl.png"

make_card() {
  local state=$1
  local title=$2
  local source_svg="$script_dir/$state.svg"
  local comparison="$temporary_dir/$state-comparison.png"
  local card_png="$temporary_dir/$state-card.png"

  agent-svgtools judge "$reference" "$source_svg" --foreground \
    --side-by-side "$comparison" >/dev/null

  magick -size 1200x720 xc:'#1e1e2e' \
    \( "$comparison" -resize 1080x540 \) \
    -gravity center -geometry +0+35 -composite \
    -font "$font_file" \
    -fill '#cdd6f4' -pointsize 32 -gravity north -annotate +0+22 "$title" \
    "$card_png"

  magick "$card_png" "$temporary_dir/$state-card.gif"
  gifsicle --delay=250 --disposal=none "$temporary_dir/$state-card.gif" \
    > "$temporary_dir/$state-card-delayed.gif"
}

make_card first-pass 'First pass · foreground score 66.63'
make_card revision-1 'Revision 1 · feathered crown · score 68.34'
make_card final 'Revision 2 · corrected wings · score 69.59'

agent-svgtools render "$script_dir/final.svg" "$temporary_dir/final-render.png" \
  --width 1000 --height 1000 >/dev/null
magick -size 560x560 xc:'#ffffff' \
  \( "$temporary_dir/final-render.png" -resize 520x520 \) \
  -gravity center -composite "$temporary_dir/final-panel.png"
magick -size 1200x720 xc:'#1e1e2e' \
  "$temporary_dir/final-panel.png" -gravity center -geometry +0+20 -composite \
  -font "$font_file" -fill '#cdd6f4' -pointsize 34 \
  -gravity north -annotate +0+20 'Final SVG' \
  -pointsize 24 -gravity south -annotate +0+16 \
  '37 named components · 12 colors · vector-only' \
  "$temporary_dir/final-outro.png"
magick "$temporary_dir/final-outro.png" "$temporary_dir/final-outro.gif"
gifsicle --delay=400 --disposal=none "$temporary_dir/final-outro.gif" \
  > "$temporary_dir/final-outro-delayed.gif"

gifsicle --loopcount=forever --colors 128 -O3 \
  "$temporary_dir/01-first-pass.gif" \
  "$temporary_dir/first-pass-card-delayed.gif" \
  "$temporary_dir/02-revision-1.gif" \
  "$temporary_dir/revision-1-card-delayed.gif" \
  "$temporary_dir/03-revision-2.gif" \
  "$temporary_dir/final-card-delayed.gif" \
  "$temporary_dir/04-inspect.gif" \
  "$temporary_dir/final-outro-delayed.gif" \
  > "$script_dir/demo.gif"
