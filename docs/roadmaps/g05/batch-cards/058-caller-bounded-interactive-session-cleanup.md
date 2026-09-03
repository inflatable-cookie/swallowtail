# 058 Caller-Bounded Interactive Session Cleanup

Status: complete; caller-bounded close migrated across all 22 interactive adapter packages
Owner: Tom
Created: 2026-09-02
Milestone: `../023-claude-sdk-shared-lifecycle-prerequisites.md`
Depends on: Contract 010; Contract 019; operator decision recorded 2026-09-02;
  completed card 057

## Goal

Make interactive-session close and every post-expiry cleanup/join path return
under a caller-selected host deadline, without inventing monotonic tick units
or leaving an unbounded compatibility path callable.

## Direction

Use the v0.4.0 breaking-release window to require close-time host services and
a caller-selected cleanup deadline on the shared interactive-session handle.
Apply the same evidence rule to cleanup after open and turn expiry. Do not add
a default-method shim that silently calls the old unbounded close.

The operator accepted this breaking seam on 2026-09-02: use whatever bounded
shared lifecycle change is required to make the Claude Agent SDK route work.
This is release-boundary authority, not permission to weaken cleanup truth or
add a silent default.

## Scope

- promote the selected close request/deadline shape in Contracts 010 and 019
- update the provider-neutral interactive-session handle and every production
  implementation without provider-specific leakage
- prove expiry bounds the public return path, escalation, joined tasks,
  resource release, and credential release
- preserve honest cleanup failure when the hard cleanup deadline expires
- update semantic API evidence, guides, examples, matrices, and release audit
  inputs for the breaking change

## Out Of Scope

SDK feature expansion; process-tree evidence owned by card 057; provider calls;
release preparation; compatibility shims; guessed duration-to-tick conversion.

## Acceptance Criteria

- [x] the public interactive-session close seam requires caller-supplied host
      services and one caller-selected cleanup deadline
- [x] no old unbounded close path, default timeout, compatibility shim, or
      guessed duration-to-tick conversion remains callable
- [x] close, open-abort, turn-expiry interrupt, escalation, pump/task join,
      credential release, and resource release return by the same observable
      hard cleanup boundary
- [x] deadline expiry produces an honest failed or degraded cleanup result and
      cannot be reported as clean
- [x] every production implementation and consumer fixture migrates in one
      coordinated public-API change
- [x] API evidence, guides, examples, matrices, changelog, and release audit
      inputs state the v0.4 break exactly

## Outcome

`SessionCleanupRequest` carries one absolute caller-selected deadline.
`InteractiveSessionHandle::close` now consumes that request and the session's
exact `HostServices`; the zero-argument signature is absent. Runtime host and
time validation wraps the complete cleanup future, observes time before work
and before accepting success, drops stalled work at expiry, and returns the
exact failed diagnostic instead of `Clean`.

All 28 production implementations across 22 adapter packages and both shared
fixture implementations migrated. Structured projections relinquish turn
handles before bounded session cleanup. Fallible projected-open paths receive
the cleanup request, while management facades validate fallible bindings before
provider work. Deterministic runtime tests stall interruption, escalation, task
join, credential release, and resource release behind the same deadline.

Unreleased semantic evidence records the new runtime type, helper, trait
signature, management prevalidation, and the Claude Agent and Cline projected-
open signatures. One exact approved `v0.4.0` removal records the old runtime
close signature; every other immutable-baseline removal still fails the API
gate.

## Validation

```sh
effigy validate:focused swallowtail-runtime swallowtail-testkit
effigy package:verify-affected swallowtail-runtime swallowtail-testkit
effigy package:api
effigy qa:routes
effigy qa:docs
effigy qa:northstar
effigy --json scan god-files
git diff --check
```

Add each adapter package whose `InteractiveSessionHandle` implementation must
change, in groups of at most four per selector invocation. Do not substitute a
broad workspace test unless a changed shared boundary makes the named package
partition impossible, and record that reason.

## Review Oracle

Invariant: once any public operation observes its deadline, no provider,
transport, task, credential, resource, or cleanup future can keep that public
operation pending beyond the caller-selected hard cleanup boundary.

Smallest counterexample: the sidecar stops answering after the turn deadline;
the interrupt response, pump join, or credential release never resolves, and
the public future remains pending forever.

Required proof: deterministic stalled futures at every post-expiry stage,
host-time observations on both sides of the bound, exact cleanup outcome, and
compile-time removal of the old close signature.

## Stop Conditions

Stop on a design needing ambient time, guessed tick units, a default deadline,
an unbounded compatibility path, provider-specific shared vocabulary, or a
cleanup stage that can outlive the public return bound.

## Auto-Continuation

No. Exact-head frontier review before merge. PR 188 stays paused until this
card and card 059 merge.
