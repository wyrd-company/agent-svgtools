# ---
# relationships:
#   demonstrates: README
# ---

Output "/tmp/agent-svgtools-owl-03.gif"

Require jq

Set Shell "bash"
Set Theme "Catppuccin Mocha"
Set FontSize 20
Set Width 1200
Set Height 720
Set Padding 24
Set TypingSpeed 12ms
Set Framerate 30

Type "# revision 2: widen the wings and lower their tips" Enter
Type "target/debug/agent-svgtools --format json judge demo/targets/owl.png demo/owl-reconstruction/final.svg --foreground | jq .scores" Enter
Sleep 3s
