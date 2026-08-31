# 2026-08-31 Claude Watcher Instruction Isolation Mechanism

Status: open; planning required
Owner: Tom
Source: g05.011 card 029 and PR 135 exact-head review

## Observation

Exact Claude Code `2.1.251` flag evidence cannot isolate the current injected
watcher skill from ambient user, project, and local skills while preserving
configured authentication, the private MCP family, and the Stop hook.

- `--bare` removes OAuth/keychain auth and still admits ambient skills
- `--restricted` preserves auth and private composition but admits ambient
  skills, CLAUDE.md, and plugins
- `--safe-mode` and `--disable-slash-commands` close skill discovery only by
  disabling the injected watcher skill mechanism too

Card 029 therefore stopped without a production behavior change. No new live
turn is ready.

## Planning Question

Find an exact operation-private instruction channel that does not share ambient
skill discovery. Contract 059 permits an exact qualified native skill,
developer instruction, MCP, dynamic-tool, or other documented mechanism, but a
future lane must prove the selected channel without treating prompt text as
enforcement or weakening the private bridge and completion gate.

Candidate evidence may compare explicit system-prompt files, private MCP tool
descriptions, or another `2.1.251` mechanism. It must preserve omission,
working-resource confinement, provider-free validation, and the no-live-turn
boundary. This note is not execution authority and does not select a mechanism.

## Next Canonical Home

A new watcher planning milestone/card after an orchestrator readiness review.
Until then g05.011 remains an evidence stop and card 024 owns the sole roadmap
Next Task.
