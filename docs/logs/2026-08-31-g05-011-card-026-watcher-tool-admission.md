# 2026-08-31 g05.011 Card 026 Watcher Tool Admission

Status: complete; hypothesis rejected
Owner: Tom
Date: 2026-08-31
Card: 026
Contracts: 059-060

## Result

Exact Claude Code `2.1.251` host help still matches the frozen official corpus
at SHA-256 `5ff2e7a0…`. It defines `--tools` as a built-in-set filter and keeps
MCP configuration separate. The proposed suppression mechanism is false.

A deterministic operation-private fixture compares omission, the current
watcher command, and removal of the `--tools Read,Glob,Grep` pair. The current
command initializes the bridge, lists exactly the six reserved watcher tools,
binds the Stop completion gate, rejects an unreserved tool with MCP method-not-
found, joins host work, and releases private settings and MCP files. Removing
the pair changes no MCP visibility and widens built-ins to the default set.
Production argv therefore stays unchanged.

## Alternative Blocker

Watcher opt-in adds `--bare`. Exact help says bare mode never reads OAuth or
keychain credentials and admits Anthropic auth only through
`ANTHROPIC_API_KEY` or `apiKeyHelper` in explicit settings. The consumed card
020 envelope had no API key; the private settings declare only the Stop hook.
That explains failure before MCP initialize more directly than tool admission.

No further live turn is ready. Card 029 must first prove a watcher-only command
that preserves configured authentication without reopening ambient hooks,
skills, MCP servers, or settings.

## Current State

- card 026 complete; no production command or watcher claim changed
- card 029 ready; sole front-door Next Task
- no provider prompt, login, credential read, paid work, or live probe
- Contracts 059-060 and the normal non-watcher command unchanged

## Authority

- [card 026](../roadmaps/g05/batch-cards/026-claude-watcher-tool-admission-evidence-and-repair.md)
- [card 029](../roadmaps/g05/batch-cards/029-claude-watcher-credential-preserving-isolation.md)
- [g05.011](../roadmaps/g05/011-watcher-route-admission-recovery.md)
- [Contract 059](../contracts/059-operation-scoped-process-watchers.md)
- [Contract 060](../contracts/060-operation-scoped-watcher-http-bridge.md)
