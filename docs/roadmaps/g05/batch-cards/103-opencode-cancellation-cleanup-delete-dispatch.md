# 103 OpenCode Cancellation Cleanup DELETE Dispatch

Status: complete; fixture-only verdict, no `v0.4.2` content; merged at `d7b483dd9d850fb0f6f3f04e4297e1bf7662333b`
Owner: Tom
Created: 2026-09-06
Updated: 2026-09-06
Milestone: `../032-v0-4-2-release-readiness.md`
Depends on: the card 100 review findings on CI runs 34001247730, 34001650005, 34002653532; cards 078 and 097 merged

## Defect

`swallowtail-adapter-opencode` test
`prepared_facade::session_import::cancellation_deadline_and_cleanup_release_leases_without_owning_the_server`
aborts on macOS CI at roughly even odds. Mechanism from the logs: the fixture
server thread panics at `tests/http_support/delete_gate.rs:41` and `:51`,
"DELETE dispatch was never observed within HANG_GUARD" (120 s), and then
`FixtureServer::drop` resume-unwinds that panic inside `Drop`, which is
non-unwinding, so the binary aborts. Two questions, in order of importance:

1. Why does the route not dispatch the lease-release DELETE within 120 s
   under a cancellation-plus-deadline race? A missing release under
   cancellation would be a production cleanup defect in code that ships in
   `v0.4.2` (OpenCode changed in cards 078 and 097).
2. Why does the fixture convert a thread panic into an abort?

## Scope

1. Reproduce locally under load (the card 093/094 method) and trace the
   cancellation and deadline paths in the OpenCode prepared session-import
   facade and driver to determine whether the DELETE can genuinely be
   skipped or deadlocked under a race, or whether only the fixture's gate
   ordering is wrong. State the answer with code anchors.
2. If production: fix the release path so cancellation and deadline both
   dispatch or fail typed, with a deterministic fixture that forces the
   race; this becomes `v0.4.2` content and card 101 waits for it.
3. If fixture-only: fix the fixture ordering so the gate cannot be missed.
4. In either case: `FixtureServer::drop` must never `resume_unwind`; record
   the fixture-thread panic and assert it from a non-Drop teardown, keeping
   the non-Drop call site's resume behaviour (`http_support/mod.rs:145`).
5. Prove with 20+ runs of the binary under load, zero aborts.

## Out Of Scope

Other crates; the release candidate; card 094's other surfaces.

## Acceptance Criteria

- [x] the production-or-fixture question is answered with anchors
- [x] no `resume_unwind` remains in any OpenCode test `Drop`
- [x] 20+ loaded runs with zero aborts and zero failures
- [x] production source unchanged unless a real defect was found, in which
      case the fix is disclosed and separately reviewed

## Validation

- `cargo fmt -p swallowtail-adapter-opencode -- --check`
- `effigy validate:focused swallowtail-adapter-opencode`
- `effigy package:verify-affected swallowtail-adapter-opencode`
- `effigy qa:northstar`
- `git diff --check`

## Result

Card 103 is complete and fixture-only. The gate was `GET /session/status?`,
not a missing DELETE. The fixture's 10 ms wall-clock budget raced thread
spawn, lease acquisition, and loopback health work under loaded macOS CI;
the driver correctly refused an already-expired deadline and released leases
on every path. The abort was a poisoned-mutex double panic: the hang guard
panicked while holding the gate, then `FixtureServer::drop` touched the
poisoned lock while unwinding. It was not a production cleanup failure and
adds no `v0.4.2` content.

The fixture now tolerates poison, `Drop` does not raise the fixture panic,
explicit shutdown performs the assertion, and manual deadline triggering
prevents the gate from being missed. The retained `JoinOnDrop` test keeps the
guarded `resume_unwind` behavior at the non-Drop suppression seam. Twenty-four
loaded runs completed with zero aborts and zero failures; focused validation,
package verification, formatting, Northstar QA, and diff checks passed.

Merged through PR 238 at `d7b483dd9d850fb0f6f3f04e4297e1bf7662333b` after all
11 hosted checks passed. Card 101 may prepare the candidate; Card 082 remains
branch-only until the `v0.4.2` tag.

## Review Oracle

Invariant: a cancelled or deadline-expired session import always releases
its lease or fails typed, and a fixture failure is a test failure, never an
abort. Smallest counterexample: a code path where cancellation returns
before the DELETE is dispatched, or a `Drop` that can panic.

## Auto-Continuation

No. Stop for exact-head review; Chatterbox rules on `v0.4.2` content.
