# 2026-08-19 Mistral Vibe Headless 2.24.2 Identity

## Result

Card 274 froze official Mistral Vibe headless identity at GitHub `v2.24.2`
/ PyPI `mistral-vibe==2.24.2` without installing, logging in, or sending
a prompt. The selected wire is `vibe --prompt … --output streaming
--max-turns N --trust --agent plan --workdir DIR`: completed
public-history NDJSON, not `vibe-acp`, not the TUI, and not
`--auto-approve`/`--yolo`. Official docs that programmatic mode defaults
to the auto-approve agent are stale versus tagged source. Named fixtures
live under the future adapter tree. No production claim.

## Next

Implement the Mistral Vibe headless driver core (card 275).
