---
title: g05 Card 141 operation bridge close latency worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: merged
owner: Tom
created: 2026-09-08
updated: 2026-09-08
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Tom authorized the switch to northstar-queue dispatch and retirement of the Swallowtail coordinator in the Chatterbox conversation on 2026-09-08, with standing direction to keep promoted ready cards moving."
tags: [coordination, handoff, worker, host-local, teardown, latency]
---

## What This Thread Was Doing

Implementing g05 card 141: remove the fixed timeout cost from
registered-tool route close in `swallowtail-host-local`.

## Why It Matters

Card 139 measured every registered-tool close at exactly 5.00 seconds.
`OperationBridgeListener::close` joins an accepted connection thread that
waits out `IO_TIMEOUT` without its read being woken; narrowing that constant
to two seconds moved the close to 2.00 seconds, so the cost is the timeout,
not work. Contract 063 allows a ten-second cleanup budget, so a fixed
five-second wait spends half of it on every close and narrows the margin
before a slow host trips `TeardownFailed`. Every Bovine Desktop session
teardown pays that five seconds today.

## Current State

Card 139 merged at `4b1e369c` and disclosed this rather than fixing
it, because its manifest forbade `swallowtail-host-local`; it is recorded in
`PAPERCUTS.md`. Card 141 closed the defect through PR 292.

- **Queue state:** closed after merge; PR 292 merged into `main` as
  `b68a1ccc757eb6adf768fb7ae3d330c5daef1547`.
- **Reviewed head:** `8572ebb556aa7fbc4a83165f35ff70467f1d6f9f`.
- **Review:** independent exact-head review accepted; comment `5585250665`
  carries the `ready_to_merge` verdict and Northstar identity marker.
- **Validation:** named formatting, focused validation (609/609), affected
  package verification, Northstar QA, and diff checks passed; hosted PR checks
  passed apart from the configured skipped MSRV floor test job.
- **Deferred:** no Card 141 validation failure. The unrelated Card 139
  nextest leak papercut, live provider/consumer acceptance, the Card 132
  real-route gate, release/tag work, and consumer-repository changes remain
  outside this handoff with their existing owners and authority.

## Boundaries

Owned and forbidden paths are the card 141 manifest row and they
bind. Preserve every teardown guarantee exactly: one listener, one accept
loop, joined tasks, admission frozen before settle, `TeardownFailed` on a
genuine timeout with the lease retained and never reported clean, and
unchanged watcher-profile behaviour. Keep `IO_TIMEOUT` as the backstop for a
genuinely unresponsive peer. Do not add a second listener, lease manager, or
loopback runtime; the single-listener topology was settled by cards 116 and
125. No adapter changes beyond named test timings, no contract edits, no live
provider work.

## Important Context

The mechanism to fix is the wake, not the constant: wake the accepted
connection thread's read on close rather than waiting the timeout out. A
socket shutdown signal, a self-connect wake, or extending the existing
`wake_accept` mechanism to accepted connections are all plausible; choose the
least invasive against the single-listener topology and say why in the card
Result. Guardian cleanup closes the registered lease before the provider close
reaches the wire, so a child-teardown change cannot shorten this; card 139
already proved that.

## Suggested Next Move

Reproduce and measure the current close on both the registered-tool
and watcher cases, implement the wake, then measure both again and record
before and after in the card Result. Retire the card 139 papercut entry.
Named validation: `cargo fmt -p swallowtail-host-local -- --check`;
`effigy validate:focused swallowtail-host-local swallowtail-adapter-claude-agent`;
`effigy package:verify-affected swallowtail-host-local swallowtail-adapter-claude-agent`;
`effigy qa:northstar`; `git diff --check`. If waking the read would require
changing the single-listener topology, stop and escalate to Chatterbox
rather than working around it.

## Completion Protocol

Stop at exact-head review. Do not merge, tag, release, or touch any consumer
repository. Report the exact head SHA, the named validation output, and
anything found but deliberately not changed. The card at
`docs/roadmaps/g05/batch-cards/141-operation-bridge-close-latency.md` and its manifest row bind: owned paths,
forbidden paths, acceptance criteria, and stop conditions are as written
there.

## Handoff Closeout

This handoff is merged. The implementation was accepted at exact head
`8572ebb556aa7fbc4a83165f35ff70467f1d6f9f` and published on `main` as
`b68a1ccc757eb6adf768fb7ae3d330c5daef1547`. No named validation failure was
deferred. The active Next Task pointer was preserved; no new planning
direction was introduced.
