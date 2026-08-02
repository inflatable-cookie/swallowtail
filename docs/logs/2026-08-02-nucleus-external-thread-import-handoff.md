# 2026-08-02 Nucleus External Thread Import Handoff

## Outcome

Card 063 and roadmap g03.023 are complete. Swallowtail publishes a bounded
Nucleus adoption handoff without editing the consumer or widening provider
session authority.

The handoff records:

- the exact Codex app-server, Kimi ACP, and OpenCode HTTP prepared entry points
- one browse-select-import-load/replay-resume flow
- consumer-owned local thread mapping and replay persistence
- an in-process-only opaque resume binding until a separate persistence
  contract is approved
- one load-scoped replay-to-live boundary without a false portable message id
- explicit duplicate-import, restart, stale, incomplete, unsupported, and
  reauthorization posture
- deterministic common and adapter fixtures suitable for consumer adoption

Repeated imports are not merged automatically. A raw provider session id never
reconstructs authority. Provider-session management bindings remain separate
and their persistence remains deferred.

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`
- no consumer checkout or live provider operation

## Current State

All queued provider-session discovery/import cards are complete. The sole
roadmap pointer has returned to the g03 evidence gate. Nucleus may implement
the handoff independently; Swallowtail resumes only from new consumer evidence,
material non-deferred provider drift, or explicit operator-selected work.
The final queue audit also repaired g03.022's stale planned status after its
three completed cards.
