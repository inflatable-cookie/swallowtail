# 094 Deadline Fixture Determinism Sweep

Status: ready
Owner: Tom
Created: 2026-09-05
Updated: 2026-09-05
Milestone: `../030-v0-4-1-release-readiness.md`
Depends on: card 091 attempt-4 floor excerpt; card 093 as the pattern precedent

## Goal

Remove every test in the workspace whose pass or fail depends on real
process timing against a host deadline, starting with the Pi sidecar
lifecycle failure that stopped card 091's fourth prepare. Release-lane repair
under the `v0.4.1` feature freeze; production code unchanged.

## Defect

`crates/swallowtail-adapter-pi/tests/sidecar_driver/lifecycle.rs`
`host_deadline_uses_native_abort_and_resolves_timed_out` opens a session
against `SidecarScenario::Hold` with `with_immediate_time()` (fixture clock
at 1000 with every timer firing immediately), times out a turn, then asserts
that `close_session` reports
`swallowtail.session_cleanup.deadline_expired`. Runtime
`bound_session_cleanup` is correct: it returns the inner cleanup outcome if
that future is ready before the deadline observation, otherwise
`deadline_expired`. Whether the inner cleanup is already ready on its first
poll depends on whether the held fake sidecar process has exited after the
turn's native abort. That is real-process timing, so the assertion is
scheduler-dependent: `None` (clean) on a loaded host, `deadline_expired` on
a quiet one. The v0.4.0 OpenCode deadline fixture and card 093's sidecar
asset fixture were the same shape.

## Scope

1. Fix the Pi lifecycle test by controlling the process lifecycle
   explicitly: the `Hold` scenario must keep the fake sidecar alive and
   unresponsive to close until the test releases it, so cleanup can never be
   ready before the immediate deadline fires; release it for teardown. The
   test then proves one outcome deterministically. If the intent is to prove
   the clean path too, add a separate test that releases the child before
   close and asserts `Clean`.
2. Sweep the workspace for the same shape: every test that asserts a
   deadline, timeout, cancellation, or cleanup outcome while a real child
   process, thread, or independently scheduled timer can change which branch
   wins. Start from `grep -rln "deadline_expired\|TimedOut\|with_immediate_time\|from_millis" crates/*/tests`
   and the Claude Agent structured-run, SDK driver cancellation, and SDK
   driver lifecycle tests already known to share the pattern. Classify each
   as deterministic, fixed here, or out of scope with a one-line reason.
3. For each fixed test, apply the card 093 rule: ordering first, one large
   named hang guard only for a dead process, never a bound a passing test
   relies on.
4. Prove determinism per touched test binary: 20+ runs under deliberate CPU
   load with zero failures, recorded in the card result, plus one full
   `cargo test --workspace --all-features --locked` under pinned `1.95.0`.
5. Keep every `crates/**/src` path unchanged. If a fixture cannot be made
   deterministic without a production change, record it and stop.

## Out Of Scope

Production behaviour; the release candidate; raising bounds as the fix;
tests whose timing sensitivity is already proven absent.

## Acceptance Criteria

- [ ] the Pi lifecycle test proves one outcome under any host load
- [ ] the sweep ledger classifies every candidate test with a reason
- [ ] every fixed binary passed the loop-under-load proof
- [ ] a full pinned-toolchain workspace test run passed
- [ ] production source is byte-identical

## Validation

- `cargo fmt --all -- --check`
- `effigy validate:focused <each touched adapter package, at most four per run>`
- `rustup run 1.95.0 cargo test --workspace --all-features --locked`
- `effigy qa:northstar`
- `git diff --check`

## Review Oracle

Invariant: no passing test in the workspace depends on which of two
independently scheduled events wins. Smallest counterexample: a retained
race with a widened bound, or a sweep entry marked deterministic without a
reason.

## Auto-Continuation

No. Stop for exact-head review; card 091 re-prepares on the merged base.

## Result

Fixture-uniqueness release gate prepared from current main `dc04df04`:

- **Claude Agent SDK sidecar asset — fixed here.** Its temporary-directory
  name now includes a process-wide `AtomicUsize` sequence after the process ID
  and timestamp. `create_dir` rejects any collision instead of sharing an
  existing root through `create_dir_all`.
- **Anthropic prepared fixture — deterministic.** Its attachment path already
  includes the process-wide `NEXT_ATTACHMENT: AtomicUsize` sequence.
- **Claude Code support — deterministic.** Its watcher temporary root already
  includes the process-wide `TEMPORARY_ROOT_SEQUENCE` counter.
- **Claude structured-run watcher proof — fixed here.** Its workspace now has
  an atomic sequence suffix, uses `create_dir`, and its unwind test checks the
  exact allocated path after owner teardown.
- **Claude live watcher probe — fixed here.** Its workspace now has an atomic
  sequence suffix and uses `create_dir`.
- **Oh My Pi fixture host — fixed here.** Attachment paths now have an atomic
  sequence suffix and use `File::create_new`, the fail-loud file equivalent of
  `create_dir`.
- **OpenCode prepared fixture — fixed here.** Attachment paths now have an
  atomic sequence suffix and use `File::create_new`.

The exact pinned reproduction command completed 20 runs with 20 passes and
zero failures:

```text
rustup run 1.95.0 cargo test -p swallowtail-adapter-claude-agent --all-features --locked --test claude_agent_sdk_sidecar_asset
```

Focused validation passed 492 tests across the three touched packages. Pinned
Rust `1.95.0` package tests passed for Oh My Pi and OpenCode; pinned Claude
structured-run and live-watcher tests passed. Formatting and the 40-package
semantic API gate passed.

This result covers only the fixture-uniqueness release gate recorded after the
fifth prepare. The broader Card 094 deadline sweep remains incomplete and
deferred post-tag under the operator compression decision; this result does not
claim otherwise. All changes are test-only plus this Result. Production source,
sidecars, Cargo files, changelog, release baselines, contracts, and Card 091 are
unchanged.
