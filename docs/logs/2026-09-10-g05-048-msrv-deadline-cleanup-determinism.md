# 2026-09-10 g05.048 MSRV Deadline-Cleanup Determinism

Date: 2026-09-10
Task: `../roadmaps/g05/048-msrv-deadline-cleanup-determinism.md`
No provider call, credential access, Desktop edit, tag creation or push,
registry publication, release-prepare rerun, or live-acceptance inference.

## Result

The recurring pinned-MSRV scheduler race in the Claude ACP structured-run
deadline proof is repaired with deterministic, separately controlled
observations. The old compound test
`cancellation_and_deadline_stop_the_turn_then_join_operation_cleanup`
asserted that an operation timeout forces the joined session cleanup past
its independent cleanup deadline.

Scheduler-order counterexample: the run task's session-cleanup future can
complete cleanly before `bound_session_cleanup` creates or observes the
cleanup deadline wait — the fixture agent answers `session/close`
synchronously and the pump can resolve the response while the cleanup's
first poll is still in flight — so the runtime accepts `CleanupOutcome::Clean`
under a frozen host clock (0 < boundary tick 1) and the terminal carries no
cleanup diagnostic. That is exactly the observed failure in both hosted runs
(34348374964 in the `v0.4.4` lane, 34456800235 for the `v0.5.0` merge SHA):
status `TimedOut`, cleanup diagnostic `None` at `structured_run.rs:304`.

The repair replaces the compound test with three exact tests:

- `cancellation_stops_the_turn_then_joins_the_clean_operation_cleanup` —
  cancellation oracle unchanged.
- `operation_deadline_times_out_the_turn_then_joins_a_clean_session_cleanup` —
  the operation deadline expires, yielding `TerminalStatus::TimedOut`, and the
  joined session cleanup completes `Clean` before its boundary; the cleanup
  deadline observation is scripted to stay unreached, so the clean result is
  not forced into a failure.
- `session_cleanup_crossing_its_caller_deadline_reports_deadline_expired` —
  the held close response keeps the cleanup genuinely pending, the second
  host-clock observation expires, and the terminal carries exactly
  `swallowtail.session_cleanup.deadline_expired`.

Fixture surfaces: `FixtureHost::with_deadline_waits` scripts every
`wait_until` observation in call order and fails on script exhaustion instead
of guessing; `FixtureHost::with_held_session_close_response` and
`release_held_session_close_response` hold back and release the close
response so a cleanup genuinely crosses its caller boundary. All changes are
inside `tests/structured_run.rs` and `tests/support/`; no production, tool
chain, workflow, version, release-receipt, or historical-evidence surface
changed.

## Validation

Pinned Rust 1.95.0 (MSRV floor toolchain):

- the three exact structured-run tests repeated 40× each (120 runs), zero
  failures;
- the full parallel `--test integration` board (170 tests) repeated 15×,
  zero failures;
- `cargo test --workspace --all-features --locked` green once at repair head
  `843fedf4`;
- `cargo +1.95.0 clippy -p swallowtail-adapter-claude-agent --all-targets
  --all-features --locked` clean.

Stable 1.97.1 integration board green. `effigy validate:focused
swallowtail-adapter-claude-agent` passed in 28 seconds.

## Disposition

The repair PR from `ns-790b9257-3f96-40d9-9b6e-ec8be991bbde` goes to
independent exact-head review; the queue owns merge and one new qualifying
all-green hosted run (pinned-MSRV floor included) before returning the exact
candidate SHA to Tom for the separate annotated-tag decision. Zero provider
contact. `v0.5.0` absent locally and remotely; `v0.4.4` immutable at
`49c9e3b2`; `.release-prepared.json` and every `0.5.0` release surface
unchanged.
