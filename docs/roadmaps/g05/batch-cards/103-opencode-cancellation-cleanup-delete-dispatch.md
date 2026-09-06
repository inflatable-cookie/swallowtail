# 103 OpenCode Cancellation Cleanup DELETE Dispatch

Status: review; fixture-only verdict, no `v0.4.2` content
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

## Review Oracle

Invariant: a cancelled or deadline-expired session import always releases
its lease or fails typed, and a fixture failure is a test failure, never an
abort. Smallest counterexample: a code path where cancellation returns
before the DELETE is dispatched, or a `Drop` that can panic.

## Auto-Continuation

No. Stop for exact-head review; Chatterbox rules on `v0.4.2` content.
