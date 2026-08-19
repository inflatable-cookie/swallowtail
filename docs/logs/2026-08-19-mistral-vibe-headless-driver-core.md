# 2026-08-19 Mistral Vibe Headless Driver Core

## Result

Card 275 added package `swallowtail-adapter-mistral-vibe` and the smallest
`mistral-vibe.headless` driver. Discovery is exact `mistral-vibe.release`
`2.24.2` with claim `mistral-vibe.headless.release-window-1` and behavior
`mistral-vibe.headless.stdio-streaming-v1`. Spawn is `vibe --prompt <text>
--output streaming --max-turns 8 --trust --agent plan --workdir <cwd>`.
Stdin closes without writing the prompt. `--output json` stays the
dump-at-end sibling, not the streaming decoder. `vibe-acp`, TUI,
`--continue`/`--resume`, teleport, and `--auto-approve`/`--yolo` stay out.

Current source is 36 packages and 43 production routes. The Vibe driver is
realized without a production claim. Immutable `v0.3.2` stays 30 packages
and 36 routes.

`effigy validate:focused swallowtail-adapter-mistral-vibe` passed (23 tests,
Clippy). No live install, login, or prompt.

## Next

Implement the Mistral Vibe headless prepared facade (card 276).
