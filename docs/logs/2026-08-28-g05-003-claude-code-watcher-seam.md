# 2026-08-28 g05.003 Claude Code Watcher Seam

Status: closed
Owner: Tom
Card: 007
Research: 257

## Result

Research 257 promotes a complete candidate watcher seam for
`claude-code.headless` `2.1.220..=2.1.241`:

- private MCP via `--mcp-config` + `--strict-mcp-config`
- instruction via operation-private `--add-dir` skill root under `--bare`
- same-turn interception via `hooks.Stop` (`decision: block` /
  `additionalContext`, `stop_hook_active`, 8-block cap)
- Claude-native background tasks kept distinct from host watchers

Current production empty strict MCP omission is preserved. No production
command change. Live Stop re-entry not run (boundary). Cards 010-011 are not
blocked by an empty mechanism set; they still need host registry and live
acceptance.

## Evidence

- fixture: `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.241/headless-watcher-seam.json`
- identity test: `tests/claude_code_headless_watcher_seam_identity.rs`
- docs digests and native binary digests recorded in Research 257

## Validation

Required card selectors only; see PR.
