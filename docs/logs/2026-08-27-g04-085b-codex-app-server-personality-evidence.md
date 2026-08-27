# 2026-08-27 g04.085b Codex App-Server Personality Evidence

Status: complete
Card: 239
Research: 238

## Boundary

Evidence only. The worker may update this file, card 239, Research 238, and new
Codex-local frozen evidence. Shared planning and production code stay unchanged.

## Outcome

Honest empty deliver-now set. Exact tags `0.147.0`–`0.149.1` expose typed
`personality` (`none|friendly|pragmatic`) on thread/turn/settings and return
preference on `ThreadSettings`, but:

- unsupported bundled models still accept the field with non-operational effect
- start/turn responses do not confirm personality
- `ThreadMetadata` does not persist/restore it
- live catalogue can move membership

Frozen corpus:
`crates/swallowtail-adapter-codex/tests/fixtures/evidence/app-server-personality-range.json`.

No production binding. Shared g04.085 / Next Task updates reserved for the
orchestrator after merge.
