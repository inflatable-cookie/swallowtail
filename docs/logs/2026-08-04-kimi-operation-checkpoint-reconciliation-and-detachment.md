# 2026-08-04 Kimi Operation Checkpoint, Reconciliation, And Detachment

Roadmap: `../roadmaps/g03/029-kimi-operation-checkpoint-reconciliation-and-detachment.md`
Cards: 073-075

## Changed

- added `ProviderOperationCheckpoint` and its opaque, versioned,
  attachment-bound persisted record
- allowed qualified runtime events to carry the newest exact operation
  checkpoint
- bound optional exact checkpoints into provider-session reconciliation
  agreements
- preserved Kimi subscribe acknowledgement cursor truth
- added prepared exact-turn reconciliation for qualified externally attached
  local servers
- added explicit Kimi active-turn detachment without native abort
- excluded callbacks, owned foreground servers, structured runs, and
  unverified-newer versions
- updated Contracts 048-049, Research 099-101, architecture, corpus, route
  matrix, and consumer guide

## Evidence

Deterministic tests prove:

- checkpoint identity, opacity, corruption, version, size, and integrity rules
- same-route persistence and cross-host attachment rejection
- exact completed and active restart reconciliation
- foreign session, foreign turn, epoch drift, cursor gap, and stale rejection
- idempotent detach, cancellation and terminal rejection
- observer close with no prompt, callback response, archive, or abort
- unchanged cancellation, reattachment, callback, owned-server, and ordinary
  session behavior

Validation:

- `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-adapter-kimi`
  — 291 tests passed; Clippy and package checks passed
- `effigy package:verify-affected swallowtail-core swallowtail-runtime swallowtail-adapter-kimi`
  — all three extracted packages passed
- `effigy qa:docs` — passed
- `cargo fmt --all -- --check` — passed
- `git diff --check` — passed

No authenticated provider work ran.

## Next

Return to the g03 retained-operation evidence gate. Qualify an exact OpenAI
background response/cursor record next, or explicitly select ACP retained
history reconciliation instead.
