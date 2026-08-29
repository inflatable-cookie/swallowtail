# 2026-08-28 g05.003 Host-Local Watcher Registry

Status: blocked
Owner: Tom
Card: 009
Contract: 059

## Result

The host-local watcher registry now provides the Contract 059 enforcement layer
behind the provider-neutral core. The pre-1.0 start seam was corrected from
caller-supplied summaries to bounded, redacted `WatcherOperationData` that is
interpreted under host policy. Only the host selects progress or terminal
summaries.

`LocalHostServices` registers the watcher service without starting work.
Approved operation data is resolved through host-owned policy; rejected starts,
capacity failures, closed turns, foreign ids, and stale lifecycle operations
fail closed before unrelated work is touched. Status, explicit wait,
model/operator stop, cancellation, deadline cleanup, and first-terminal-wins
races share one turn-scoped registry.

Local watcher processes drain bounded output without exposing raw content.
Local process launch now creates an owned process group. Graceful stop closes
stdin and requests group termination; forced cleanup uses exact platform
process-tree commands, closes descendants that retain inherited pipes, and
joins process readers and scoped monitor tasks. Escaped descendants, failed
tree termination, or failed reader joins are surfaced as failed process and
watcher cleanup; they are never promoted to clean `ProcessExit` or joined
truth. A new closed-pipe fixture proves an escaped descendant remains live
independently of output drain; the host detects it and fails closed without
signalling its unowned PID. Dropping an unjoined watcher also requests force
cleanup.

## Evidence

- `swallowtail-core`: bounded/redacted `WatcherOperationData` start input.
- `swallowtail-runtime`: host-selected start/completion shape, host-service
  registration, rollback for accepted-but-unbound starts, and unchanged
  turn-scoped registry ownership.
- `swallowtail-host-local`: registered local watcher service, approved launch
  policy, bounded monitoring, owned process-group cleanup, escaped-descendant
  detection, cancellable reader joins, lifecycle retirement, live wait
  cancellation/deadline outcomes, multi-wait notification, and turn cleanup.
- `swallowtail-testkit` and host-local integration fixtures: registration and
  rejection before work, natural completion, bounded output failure, explicit
  wait, idempotent stop, capacity, foreign ids, cancellation/deadline cleanup,
  completion races, escaped-descendant failure, foreign-group safety, bounded
  retirement, and descendant cleanup timing.
- Public API changes are recorded only in unreleased baselines; no released
  baseline was changed.

## Contract Blocker

The host-local boundary can retain and signal its owned process group, but an
escaped descendant can call `setsid`, close the supervised pipes, and remain
alive after the root exits. Numeric-PID signalling would violate
foreign-process safety. The implementation returns
`swallowtail.local_process.descendant_escape_detected` and leaves cleanup
failed/non-joined. Contract 059 currently requires that no watcher outlive its
turn and that every failure path stop and join owned work. This evidence needs
an operator contract decision before card 009 can be complete or card 010 can
be ready.

## Structural Health

The initial watcher implementation introduced new doctor findings. The host
watcher facade and integration tests were split into focused modules. Final
`effigy doctor` returned the inherited 381 scan findings: 334
warnings and 47 errors, with no new finding. The existing generated-in-src and
stale graph-index warnings remain; the doctor baseline is unchanged.

## Validation

- `cargo fmt -p swallowtail-core -p swallowtail-runtime -p swallowtail-host-local -p swallowtail-testkit`
- `cargo check -p swallowtail-core -p swallowtail-runtime -p swallowtail-host-local -p swallowtail-testkit`
- `cargo test -p swallowtail-core -p swallowtail-runtime -p swallowtail-testkit --lib`
- `cargo test -p swallowtail-host-local`
- `cargo clippy -p swallowtail-core -p swallowtail-runtime -p swallowtail-host-local -p swallowtail-testkit --all-targets -- -D warnings`
- `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-host-local swallowtail-testkit` — 473 tests passed
- `effigy package:verify-affected swallowtail-core swallowtail-runtime swallowtail-host-local swallowtail-testkit` — passed
- `effigy package:api` — semantic API passed for 40 packages at v0.3.3; immutable v0.3.2 remains 30; v0.3.0 removals remain forbidden
- `cargo clippy -p swallowtail-core -p swallowtail-runtime -p swallowtail-host-local -p swallowtail-testkit --all-targets -- -D warnings` — passed
- `effigy qa:northstar`, docs indexes, and roadmap next-action checks — passed
- Northstar Rust everyday closeout — compiler, lint, and docs evidence passed;
  package tests exited zero with one parser warning; focused Effigy validation
  is the accepting test evidence
- `git diff --check` — passed

No provider prompt, adapter wiring, credentials, ambient configuration mutation,
release work, or merge was performed. Card 010 is gated on the Contract 059
decision recorded above; card 011 remains planned.

## Review Fixes

PR #117 review findings are implemented in the follow-up batch. Local process
supervision retains a live host-owned group owner through tree cleanup, uses
safe process-group signalling, observes descendants, makes reader
cancellation joinable, and reports tree or reader cleanup failure instead of
clean exit. The watcher wait now observes live cancellation/deadline futures;
its task and watcher signals wake every supported waiter, including when a
task panics. Turn retirement releases joined entries, rejects stale controls,
bounds tombstones, and namespaces reused-turn identities. Monitor-task spawn
failure rolls back the registry and process, with deterministic retry and
finalization coverage. The escaped-descendant contract remains unresolved, so
the generation index no longer points at card 010 as ready.

## Review Handoff

Pull request [#117](https://github.com/inflatable-cookie/swallowtail/pull/117)
was opened against `main` from implementation head `d146afc7`. Review fixes were
applied in a follow-up batch on the same worker branch. No merge was performed.
