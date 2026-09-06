# 104 Post-Tag Determinism Sweep

Status: ready
Owner: Tom
Created: 2026-09-06
Updated: 2026-09-06
Milestone: `../031-ci-latency.md`
Depends on: card 094 (merged; its deferred post-tag list), card 103 (merged; the OpenCode surface is done)

## Goal

Retire the remaining timing and leak surfaces card 094 deferred until after
the `v0.4.1` tag, so hosted shard reruns stop costing release lanes. Card 103
already settled the OpenCode surface; this card owns the rest.

## Surfaces

1. `swallowtail-adapter-kimi-platform/tests/direct_driver.rs`
   `in_flight_deadline_times_out_and_releases_after_connection_join`: hosted
   shard 2/3 deadline timing flake. Replace the wall-clock bound with a
   manual or test clock, as card 103 did for OpenCode.
2. `swallowtail-host-local/tests/local_process/descendant_tree.rs`: both
   descendant-tree cases, timing flake. Same treatment; observe process
   state through a readiness signal rather than a sleep bound.
3. `swallowtail-host-local::watcher_service::lifecycle::watcher_stop_and_join_retires_owned_identities`:
   the one leaky test from the status-level leak run. Find the retained
   identity and make stop-and-join deterministic.
4. `swallowtail-host-local/tests/local_process/` fixture test (card 100's
   follow-up note): bring it under the same clock and readiness discipline.

Out of scope here: the `swallowtail-adapter-claude-agent` fake-sidecar
process-leak family. It shares paths with the g05.029 runway and is folded
into card 088's closeout instead.

## Method

Card 093/094 method: reproduce under CPU load with concurrent binaries,
classify each surface (timing bound, shared temp path, teardown order, real
leak), fix the class not the symptom, prove with 20+ loaded runs.

## Acceptance Criteria

- [ ] each of the four surfaces classified with anchors and fixed at the class
- [ ] 20+ loaded runs per affected binary with zero failures and zero leaks
- [ ] no production source change unless a real leak is found and disclosed
- [ ] card 094's deferred list annotated as retired except the sidecar family

## Validation

- `cargo fmt -p swallowtail-adapter-kimi-platform -p swallowtail-host-local -- --check`
- `effigy validate:focused swallowtail-adapter-kimi-platform swallowtail-host-local`
- `effigy package:verify-affected swallowtail-adapter-kimi-platform swallowtail-host-local`
- `effigy qa:northstar`
- `git diff --check`

## Review Oracle

Invariant: no test in these binaries depends on a wall-clock bound or on
teardown order for correctness. Smallest counterexample: a sleep or
timeout whose value decides pass or fail.

## Auto-Continuation

No. Stop for exact-head review.
