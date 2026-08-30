# 2026-08-30 g05.003 Claude Code Watcher Binding

Status: complete
Owner: Tom
Card: 010
Contract: 059, 060
Research: 257, 260, 261

## Result

Claude Code headless opts into the existing Contract 060 bridge only on exact
`2.1.251`. Omission keeps `--mcp-config {"mcpServers":{}}` plus
`--strict-mcp-config` and does not open a listener, lease files, or change
argv. Other versions reject watcher opt-in before those effects.

Opt-in opens the host-owned HTTP/MCP lease, then materializes operation-private
MCP, Stop-hook settings, and a bounded watcher skill through a temporary
working-resource filesystem. Endpoint and bearer stay in that lease, not in
argv, ambient environment, or default formatting. Model MCP calls and operator
controls share one `WatcherHostService`. Deterministic fake-provider fixtures
start a host watcher through the reserved MCP family, observe raw completion-gate
text with `decision: "block"` while work remains, omit that decision when idle,
wait/stop, then admit terminal. Cancel, deadline, and provider-failure paths
freeze, join, and release private files. Temporary MCP/settings/skill material
is 0700/0600 on Unix even when the configured temp root is shared. No watcher
support claim, capability matrix, or live Claude turn landed.

Card 011 remains planned. Live same-turn proof still needs explicit operator
authorization for provider access, credentials, and any paid work.

## Evidence

- `swallowtail-adapter-claude-agent`: exact version gate, private composition,
  Stop continuation, joined cleanup, pending-once host-future proofs
- `swallowtail-host-local`: nested working-resource writes, Stop-hook decision
  mapping on completion-gate text, private temp permissions
- Unreleased public-api baseline updated for the opt-in profile methods
- Focused structured-run fixtures; no live provider probe

## Next

Orchestrator reassessment of card 011. Do not start it from this closeout.
