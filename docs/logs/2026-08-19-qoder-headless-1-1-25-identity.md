# 2026-08-19 Qoder Headless 1.1.25 Identity

## Result

Card 278 froze official Qoder headless identity at npm
`@qoder-ai/qodercli@1.1.25` without installing, logging in, or sending
a prompt. The selected wire is `qodercli --print --output-format
stream-json --permission-mode dont_ask --max-turns N
--no-session-persistence --cwd DIR` plus one prompt operand: stream-json
NDJSON (`system` / `assistant` / `result`), not `--acp`, not SDK stdio,
and not `--yolo` / `bypass_permissions`. Omitting `--permission-mode`
inherits host settings. Named fixtures live under the future adapter
tree. No production claim.

## Next

Implement the Qoder headless driver core (card 279).
