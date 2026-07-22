# ---
# relationships:
#   demonstrates: README
# ---

Output "/tmp/agent-svgtools-owl-04.gif"

Require jq

Set Shell "bash"
Set Theme "Catppuccin Mocha"
Set FontSize 20
Set Width 1200
Set Height 720
Set Padding 24
Set TypingSpeed 12ms
Set Framerate 30

Type "target/debug/agent-svgtools --format json inspect demo/owl-reconstruction/final.svg | jq '{structure, named: .naming.named_components, colors: .palette.count, vector_only: .safety.vector_only}'" Enter
Sleep 4s

Type "target/debug/agent-svgtools render demo/owl-reconstruction/final.svg /tmp/owl-demo.png --width 1000 --height 1000" Enter
Sleep 4s
