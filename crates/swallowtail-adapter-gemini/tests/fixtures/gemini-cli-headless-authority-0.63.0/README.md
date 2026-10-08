# Gemini CLI 0.63.0 headless authority evidence

This fixture binds the selected Plan Mode authority findings to the exact
0.63.0 source-tree inventory frozen in
gemini-cli-0.63.0/source-tree-inventory.json. The downloaded source archive
was inspected as data only. Its extracted 3,019-file tree, including the
recorded symlink, reproduces the frozen tree manifest.

The noninteractive Plan Mode policy allows exit_plan_mode; that allowed path
switches to YOLO. The separate noninteractive ASK_USER to DENY conversion
does not protect this explicit allow. This contradicts the headless route's
no-automatic-implementation-transition contract. The fixture records a stop;
it does not qualify 0.62.0 or 0.63.0.

Other selected changes are bounded here for follow-up: defensive real-path
checks for read_file, .gemini write confirmation denied in noninteractive
mode, provider-owned 64 KiB stored tool output and context-pressure history
collapse, and unchanged stream framing.
