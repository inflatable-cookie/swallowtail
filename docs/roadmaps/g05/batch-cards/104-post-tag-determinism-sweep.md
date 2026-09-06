# 104 Post-Tag Determinism Sweep

Status: complete; PR 251 merged; fixture-only verdict, no production source changes
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

- [x] each of the four surfaces classified with anchors and fixed at the class
- [x] 20+ loaded runs per affected binary with zero failures and zero leaks
- [x] no production source change unless a real leak is found and disclosed
- [x] card 094's deferred list annotated as retired except the sidecar family

## Validation

- `cargo fmt -p swallowtail-adapter-kimi-platform -p swallowtail-host-local -- --check`
- `effigy validate:focused swallowtail-adapter-kimi-platform swallowtail-host-local`
- `effigy package:verify-affected swallowtail-adapter-kimi-platform swallowtail-host-local`
- `effigy qa:northstar`
- `git diff --check`

## Result

Card 104 is fixture-only. Production source in both affected crates is
unchanged, and the Claude adapter plus the reserved fake-sidecar process-leak
family were untouched.

- Kimi's old 20-tick real-clock deadline could win or lose against loopback
  dispatch under load. `ThreadServices` now offers a manual deadline trigger;
  `FixtureServer::wait_for_attempt` proves the POST dispatch before the test
  fires it. The driver test no longer relies on the wall-clock deadline.
- The reviewed-head synchronization fixes close both readiness races:
  `ManualDeadlineWait::poll` returns `Ready` when the fire is observed after
  waiter registration, and the fixture POST predicate is a boolean protected
  by the same mutex paired with its condition variable before notification.
- Both local descendant cases now gate on fixture markers and recorded PIDs,
  then poll process state to observe exit. The control case explicitly releases
  its parked descendant. The fixture parent and native descendant no longer
  use outcome-deciding lifetime sleeps.
- The local-process `hold`, `sleep`, sidecar-parent, and cooperative-child
  fixture arms now use `hold_forever`, a loop around `thread::park`, so a
  spurious park return cannot make a fixture exit early. Their lifetime is now
  governed by the host stop/join or process-group reaping path; the affected
  tests cover that cleanup explicitly. Replacing the old bounded sleeps removes
  the accidental early-exit window without making cleanup depend on a timeout.
- The watcher identity case's loaded leak report did not reproduce as a
  production retention defect. The existing host path only retires a turn
  after every watcher task and process is joined and removes its active
  identity; the test now uses a parked process fixture and explicitly closes
  the outer host task-reaper lifecycle after the stale-identity assertions.
- A first loaded implementation exposed a missing control-sidecar readiness
  handshake; it was fixed before the acceptance run. The final loaded run used
  20 CPU burners, eight concurrent lanes, and three rounds: 24 Kimi runs, 24
  local-process invocations covering both descendant cases, and 24 watcher
  runs. All 72 invocations passed with zero aborts, failures, or `leaky`
  results. A post-run process-table check found no fixture, sidecar, native
  descendant, or `/bin/sleep` hold process.
- The reviewed branch rebased cleanly onto `origin/main` at Card 113 merge
  `d7ed04ff`; the diff remains limited to the nine Card 104-owned test/docs
  paths. The bounded post-rebase loaded sanity set used eight CPU burners, four
  concurrent lanes, and two rounds: eight runs per affected binary, all clean
  with zero aborts, failures, or `leaky` results. The existing 120-second
  fixture hang guard and unrelated 30-second witness were not changed.

The focused tests, nextest leak-level check, loaded proof, formatting,
focused validation, affected-package verification, Northstar QA, and diff
checks all passed. The branch is ready for exact-head review.

## Review Oracle

Invariant: no test in these binaries depends on a wall-clock bound or on
teardown order for correctness. Smallest counterexample: a sleep or
timeout whose value decides pass or fail.

## Auto-Continuation

No. Stop for exact-head review.
