# 2026-08-28 g05.003 Host-Local Watcher Registry

Status: complete
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
joins process readers and scoped monitor tasks. Dropping an unjoined watcher
also requests force cleanup.

## Evidence

- `swallowtail-core`: bounded/redacted `WatcherOperationData` start input.
- `swallowtail-runtime`: host-selected start/completion shape, host-service
  registration, rollback for accepted-but-unbound starts, and unchanged
  turn-scoped registry ownership.
- `swallowtail-host-local`: registered local watcher service, approved launch
  policy, bounded monitoring, process-group cleanup, lifecycle joins, and
  turn cleanup.
- `swallowtail-testkit` and host-local integration fixtures: registration and
  rejection before work, natural completion, bounded output failure, explicit
  wait, idempotent stop, capacity, foreign ids, cancellation/deadline cleanup,
  completion races, and descendant cleanup timing.
- Public API changes are limited to unreleased baselines for core, runtime,
  host-local, and testkit.

## Structural Health

The initial watcher implementation introduced new doctor findings. The host
watcher facade and integration tests were split into focused modules. Final
`effigy doctor --verbose` returned the inherited 381 scan findings: 334
warnings and 47 errors, with no new finding. The existing generated-in-src and
stale graph-index warnings remain; the doctor baseline is unchanged.

## Validation

- `cargo fmt -p swallowtail-core -p swallowtail-runtime -p swallowtail-host-local -p swallowtail-testkit`
- `cargo check -p swallowtail-core -p swallowtail-runtime -p swallowtail-host-local -p swallowtail-testkit`
- `cargo test -p swallowtail-core -p swallowtail-runtime -p swallowtail-testkit --lib`
- `cargo test -p swallowtail-host-local`
- `cargo clippy -p swallowtail-core -p swallowtail-runtime -p swallowtail-host-local -p swallowtail-testkit --all-targets -- -D warnings`
- `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-host-local swallowtail-testkit` — 460 tests passed
- `effigy package:verify-affected swallowtail-core swallowtail-runtime swallowtail-host-local swallowtail-testkit` — passed
- `effigy package:api` — semantic API passed for 40 packages at v0.3.3; immutable v0.3.2 remains 30; v0.3.0 removals remain forbidden
- `git diff --check` — passed before final docs-only reconciliation; rerun at closeout

No provider prompt, adapter wiring, credentials, ambient configuration mutation,
release work, or merge was performed. Card 010 is ready as the sole next lane;
card 011 remains planned.

## Review Handoff

Pull request [#117](https://github.com/inflatable-cookie/swallowtail/pull/117)
was opened against `main` from implementation head `d146afc7`. The closeout-log
update is pushed on the same worker branch. No merge was performed.
