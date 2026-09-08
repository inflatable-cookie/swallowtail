# 141 Operation Bridge Close Latency

Status: ready
Owner: Tom
Created: 2026-09-08
Updated: 2026-09-08
Milestone: `../031-ci-latency.md`
Depends on: card 139 merged (`4b1e369c`), which measured and disclosed this; Contract 060 close and join rules; Contract 063 teardown budget

## Defect

Card 139 measured every registered-tool route close at exactly 5.00 seconds.
`OperationBridgeListener::close` joins an accepted connection thread that
waits out `IO_TIMEOUT` without its read being woken; narrowing that constant
to two seconds moved the close to 2.00 seconds, so the cost is the timeout
itself, not work. Guardian cleanup closes the registered lease before the
provider close reaches the wire, so a child-teardown change cannot shorten
it. Card 139's manifest forbade `swallowtail-host-local`, so it was recorded
rather than fixed.

This is a real consumer cost, not a test artefact. Contract 063 allows a
ten-second cleanup budget; spending half of it waiting out a timeout on every
close means a Desktop session teardown pays five seconds it does not owe, and
it narrows the margin before a slow host trips `TeardownFailed`.

## Scope

1. Wake the accepted connection thread's read on close rather than waiting
   for `IO_TIMEOUT`: a shutdown signal on the socket, a self-connect wake, or
   the existing `wake_accept` mechanism extended to accepted connections —
   whichever the reviewer judges least invasive against the single-listener
   topology cards 116 and 125 settled.
2. Preserve every teardown guarantee exactly: one listener, one accept loop,
   joined tasks, admission frozen before settle, `TeardownFailed` on a genuine
   timeout with the lease retained and never reported clean, and the watcher
   profile's existing behaviour unchanged.
3. Measure before and after on the registered-tool close cases and the
   watcher cases, and record both in the card Result. The target is a close
   bounded by the work, not by a constant.
4. Keep `IO_TIMEOUT` as the backstop for a genuinely unresponsive peer.

## Out Of Scope

The Contract 063 cleanup budget itself; adapter code; the live gate; any
change to what close reports.

## Acceptance Criteria

- [ ] registered-tool close no longer pays a fixed timeout; measured before and after
- [ ] watcher profile close behaviour and timings unchanged
- [ ] all existing teardown, race, and both-profile fixtures pass unchanged
- [ ] `TeardownFailed` still fires on a real unresponsive peer, with the lease retained
- [ ] the card 139 papercut entry is retired

## Validation

- `cargo fmt -p swallowtail-host-local -- --check`
- `effigy validate:focused swallowtail-host-local swallowtail-adapter-claude-agent`
- `effigy package:verify-affected swallowtail-host-local swallowtail-adapter-claude-agent`
- `effigy qa:northstar`
- `git diff --check`

## Review Oracle

Invariant: close waits for work, not for a clock, and every teardown
guarantee survives. Smallest counterexample: a close that returns before its
listener thread is joined.

## Auto-Continuation

No. Stop for exact-head review.
