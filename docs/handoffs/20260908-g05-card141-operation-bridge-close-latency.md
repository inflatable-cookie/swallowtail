---
title: g05 Card 141 operation bridge close latency worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-09-08
updated: 2026-09-08
base_required: pushed-main
queue_dispatch: northstar-queue
queue_approval: "Tom authorized the switch to northstar-queue dispatch and retirement of the Swallowtail coordinator thread in the Chatterbox conversation on 2026-09-08, and has standing direction to keep promoted ready cards moving."
tags: [coordination, handoff, worker, host-local, teardown, latency]
---

## Objective

Remove the fixed timeout cost from registered-tool route close. Card 139
measured every registered-tool close at exactly 5.00 seconds:
`OperationBridgeListener::close` joins an accepted connection thread that
waits out `IO_TIMEOUT` without its read being woken. Narrowing that constant
to two seconds moved the close to 2.00 seconds, so the cost is the timeout,
not work.

## Current State

Card 139 merged at `4b1e369c` and disclosed this rather than fixing it,
because its manifest forbade `swallowtail-host-local`. It is recorded in
`PAPERCUTS.md`. Contract 063 allows a ten-second cleanup budget, so a fixed
five-second wait spends half of it on every close and narrows the margin
before a slow host trips `TeardownFailed`. A Desktop session teardown pays
that five seconds today.

## Scope

Wake the accepted connection thread's read on close instead of waiting out
`IO_TIMEOUT`: a socket shutdown signal, a self-connect wake, or the existing
`wake_accept` mechanism extended to accepted connections, whichever is least
invasive against the single-listener topology cards 116 and 125 settled.
Preserve every teardown guarantee exactly: one listener, one accept loop,
joined tasks, admission frozen before settle, `TeardownFailed` on a genuine
timeout with the lease retained and never reported clean, and unchanged
watcher-profile behaviour. Keep `IO_TIMEOUT` as the backstop for a genuinely
unresponsive peer.

## Acceptance

Registered-tool close no longer pays a fixed timeout, measured before and
after and recorded in the card Result; watcher close behaviour and timings
unchanged; all existing teardown, race, and both-profile fixtures pass
unchanged; `TeardownFailed` still fires on a real unresponsive peer with the
lease retained; the card 139 papercut entry is retired.

## Stop Conditions

Waking the read would require changing the single-listener topology: stop and
return to Chatterbox rather than adding a second listener or lease manager.

## Validation

- `cargo fmt -p swallowtail-host-local -- --check`
- `effigy validate:focused swallowtail-host-local swallowtail-adapter-claude-agent`
- `effigy package:verify-affected swallowtail-host-local swallowtail-adapter-claude-agent`
- `effigy qa:northstar`
- `git diff --check`

## Completion Protocol

Stop at exact-head review. Do not merge, tag, release, or touch any consumer
repository. Report the exact head SHA, the validation output, and anything you
found but did not change. The card is `docs/roadmaps/g05/batch-cards/141-operation-bridge-close-latency.md`;
its manifest row carries the owned and forbidden paths and they bind.
