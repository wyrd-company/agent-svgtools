# ---
# relationships:
#   demonstrates: README
# ---

Output "/tmp/agent-svgtools-owl-02.gif"

Require jq

Set Shell "bash"
Set Theme "Catppuccin Mocha"
Set FontSize 20
Set Width 1200
Set Height 720
Set Padding 24
Set TypingSpeed 12ms
Set Framerate 30

Type "# revision 1: replace the round dome with a feathered crown" Enter
Type "target/debug/agent-svgtools --format json judge demo/targets/owl.png demo/owl-reconstruction/revision-1.svg --foreground | jq .scores" Enter
Sleep 3s
