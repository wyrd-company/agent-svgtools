# ---
# relationships:
#   demonstrates: README
# ---

Output "/tmp/agent-svgtools-owl-01.gif"

Require jq

Set Shell "bash"
Set Theme "Catppuccin Mocha"
Set FontSize 20
Set Width 1200
Set Height 720
Set Padding 24
Set TypingSpeed 12ms
Set Framerate 30

Type "target/debug/agent-svgtools skill | sed -n '1,14p'" Enter
Sleep 3s

Ctrl+L
Type "target/debug/agent-svgtools --format json colors demo/targets/owl.png --max-colors 12 | jq -c '[.colors[].hex]'" Enter
Sleep 2s

Type "# complete semantic first pass" Enter
Type "target/debug/agent-svgtools --format json judge demo/targets/owl.png demo/owl-reconstruction/first-pass.svg --foreground | jq .scores" Enter
Sleep 3s
