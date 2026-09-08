# 141 Operation Bridge Close Latency

Status: complete; PR 292 merged at `b68a1ccc757eb6adf768fb7ae3d330c5daef1547` (reviewed head `8572ebb556aa7fbc4a83165f35ff70467f1d6f9f`)
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

- [x] registered-tool close no longer pays a fixed timeout; measured before and after
- [x] watcher profile close behaviour and timings unchanged
- [x] all existing teardown, race, and both-profile fixtures pass unchanged
- [x] `TeardownFailed` still fires on a real unresponsive peer, with the lease retained
- [x] the card 139 papercut entry is retired

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

## Result

### Mechanism

`OperationBridgeListener` now records a `try_clone` of every accepted stream
next to its join handle. `close` shuts down the read side of each live
connection's clone before joining, so an idle connection thread wakes on
end-of-stream instead of waiting out `IO_TIMEOUT`; the write side stays open,
so a handler already in flight still delivers its response. The connection
thread retires its own registry entry when its loop returns, so the clone's
descriptor closes with the connection and a peer's end-of-stream lands exactly
where it did before. `IO_TIMEOUT` is unchanged and remains the backstop for a
genuinely unresponsive peer and for a stream whose clone could not be taken.

Self-connect was rejected: `wake_accept` can only end a blocked `accept`, not
the per-connection blocking reads. Carrying a close signal into the loop would
have required restructuring the blocking read. Read-only shutdown of the
accepted streams is the least invasive against the single-listener topology
cards 116 and 125 settled: one listener, one accept loop, joined tasks, and
unchanged watcher-profile behaviour.

### Measurements

- Registered idle keep-alive close, new fixture
  `close_joins_an_idle_keep_alive_connection_without_paying_the_read_timeout`
  (bounds close under two seconds permanently): 5.001s before, 0.21ms after.
- `claude_agent_sdk_driver::registered_tool_route::close_joins_the_registered_listener`
  (card 139's instrument, whole test): 11.87s before, 1.54s after; the flat
  5.00s close portion is gone.
- Watcher in-flight close, `retired_proof_retains_an_in_flight_wait`: 0.11s
  before, 0.11s after; the in-flight response is still delivered.
- Watcher `cross_lease_bearer_fails_and_close_releases_the_listener`: 5.00s
  before, 5.00s after. Its five seconds are live-phase (the client waits for
  end-of-stream after a 401 keep-alive while the connection thread serves its
  read-timeout backstop), not close latency; the unchanged timing is the
  live-phase-semantics-unchanged evidence, not a remaining defect.

### Found and fixed during implementation

The first cut held the wake clone in the registry until `close`, which pinned
the socket open after a connection thread exited and removed the peer's
end-of-stream at the live-phase read timeout;
`cross_lease_bearer_fails_and_close_releases_the_listener` failed at 13.00s
and named it. Connection-thread self-retirement of the registry entry
restored the exact socket lifetime.

### Guarantees

One listener and one accept loop are untouched. Spawn registration, the
closed re-check, and the push now share one lock section, so a concurrent
close either sees the connection in the registry or observes `closed` before
spawning; every spawned connection is joined — this closes the previous small
spawn/push window rather than widening it. Admission freeze before settle and
the `teardown_failed` budget gate sit in the kernel join path ahead of
`close_listener_if_idle` and are unchanged, so `TeardownFailed` still fires on
a real budget overrun with the lease retained. The card 139 papercut entry is
retired. Waking the read required no topology change, so the stop condition
did not trigger.

### Closeout

PR 292 was independently accepted at exact head
`8572ebb556aa7fbc4a83165f35ff70467f1d6f9f` and merged into `main` as
`b68a1ccc757eb6adf768fb7ae3d330c5daef1547`. The accepted review comment is
`5585250665`; it found no required changes. The configured hosted MSRV floor
test job was skipped, while the other required PR checks passed; this is not a
Card 141 validation failure. No named validation failure was deferred. The
unrelated Card 139 nextest leak papercut and the live provider/consumer gates
remain separately owned. The active Next Task pointer remains unchanged.

## Auto-Continuation

No. Stop for exact-head review.
