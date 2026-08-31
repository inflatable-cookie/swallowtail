# 2026-08-31 g05.011 Card 029 Watcher Isolation Stop

Status: complete; evidence stop
Owner: Tom
Date: 2026-08-31
Card: 029
PR: 135
Merge: `e1313e5f`
Contracts: 059-060

## Result

Exact Claude Code `2.1.251` help, prompt-free parser probes, and deterministic
prepared-command fixtures reject every compared watcher isolation shape. None
preserves configured authentication, the operation-private MCP/Stop/skill
composition, and exclusion of every named ambient authority together.

The first PR head incorrectly selected `--restricted`. Orchestrator review
found that its own evidence still admitted ambient skills and CLAUDE.md. The
revised head re-derived every disposition fail closed, restored the 391
god-file baseline, and took the card's honest-stop path. Production watcher
argv stayed unchanged.

## Current State

- PR 135 merged exact accepted head `6dc98a88` through `e1313e5f`
- all five CI checks passed
- no provider prompt, credential read, live probe, or watcher claim
- the Claude watcher route remains not live-ready
- instruction isolation now requires a mechanism change, preserved in triage
- g05.011 stopped; card 024 is the sole Next Task

## Authority

- [card 029](../roadmaps/g05/batch-cards/029-claude-watcher-credential-preserving-isolation.md)
- [g05.011](../roadmaps/g05/011-watcher-route-admission-recovery.md)
- [mechanism triage](../triage/2026-08-31-claude-watcher-instruction-isolation-mechanism.md)
- [Contract 059](../contracts/059-operation-scoped-process-watchers.md)
- [Contract 060](../contracts/060-operation-scoped-watcher-http-bridge.md)
