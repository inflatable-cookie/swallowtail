# 2026-08-19 Qoder Headless Driver Core

## Result

Card 279 added package `swallowtail-adapter-qoder` and the smallest
`qoder.headless` driver. Discovery is exact `qoder.package` `1.1.25`
with claim `qoder.headless.package-window-1` and behavior
`qoder.headless.stdio-stream-json-v1`. Spawn is `qodercli --print
--output-format stream-json --permission-mode dont_ask --max-turns 8
--no-session-persistence --cwd <cwd> <prompt>`. Stdin closes without
writing the prompt. `--output-format json` stays the dump-at-end
sibling, not the streaming decoder. `--acp`, SDK stdio, TUI, `--yolo` /
`bypass_permissions` / `accept_edits`, and the `qoder` IDE dispatcher
stay out.

Current source is 37 packages and 44 production routes. The Qoder driver
is realized without a prepared facade or production claim. Immutable
`v0.3.2` stays 30 packages and 36 routes.

`effigy validate:focused swallowtail-adapter-qoder` passed (25 tests,
Clippy). No live install, login, or `--print` prompt.

## Next

Implement the Qoder headless prepared facade (card 280).
