# 2026-09-03 g05.023 Card 058 Caller-Bounded Interactive Session Cleanup

Status: complete; caller-bounded cleanup across all 22 interactive adapter packages; one PR; no merge
Owner: Tom

## Result

Card 058 removes the public unbounded interactive-session close seam.
`SessionCleanupRequest` carries one absolute caller-selected `Deadline`, and
`InteractiveSessionHandle::close` requires the session's exact `HostServices`.
The runtime rejects missing or cross-host time authority, observes time before
polling cleanup and before accepting success, and reports exact failed cleanup
when the deadline expires. No default timeout, compatibility shim, ambient
clock, or guessed duration-to-tick conversion remains.

All 28 production implementations across the 22 interactive adapter packages
and both shared fixture implementations migrated together. Structured run paths
drop turn handles before bounded session cleanup. Projected-open failures
receive the cleanup request, while management facades validate fallible binding
context before provider work.

## Boundary Proof

Deterministic runtime tests stall five cleanup stages independently:
interruption, escalation, task join, credential release, and resource release.
Each uses host time on both sides of the same absolute boundary, returns
`CleanupOutcome::Failed` on expiry, and proves the stalled future is dropped.
Pre-expired, missing-time, and cross-host requests fail before polling cleanup.
A fresh ready result is the only path accepted as `Clean`.

## Public API And Documentation

The unreleased semantic baselines record the new runtime request and helper,
the breaking trait signature, management prevalidation, and the Claude Agent
and Cline projected-open signatures. The exact removed zero-argument runtime
signature is recorded under approved v0.4 removal evidence. The API gate rejects
unapproved removals, stale approval, unknown packages, and approval without an
immutable baseline.

Contracts 010 and 019, lifecycle guidance, architecture, route matrices,
examples, changelog, and release-audit inputs now state the caller-owned bound.
Card 058 closes, but g05.023 stops on card 059's independent evidence result:
no sound owned-tree mechanism was found within current ordinary host-local
authority on macOS. PR 188 remains preserved and blocked on the operator's
route-posture decision.

## Validation

The exact restacked source passed:

- focused and affected-package selectors for `swallowtail-runtime` and
  `swallowtail-testkit`
- focused and affected-package selectors for all 22 interactive adapters, in
  groups of at most four
- `effigy package:api`
- `effigy qa:routes`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy --json scan god-files`
- `git diff --check`

No provider contact, live probe, package install, release mutation, tag, or
merge occurred.

## Authority

- [card 058](../roadmaps/g05/batch-cards/058-caller-bounded-interactive-session-cleanup.md)
- [g05.023](../roadmaps/g05/023-claude-sdk-shared-lifecycle-prerequisites.md)
- [Contract 010](../contracts/010-execution-host-services-and-inputs.md)
- [Contract 019](../contracts/019-embedded-sdk-and-cloud-client-boundary.md)
