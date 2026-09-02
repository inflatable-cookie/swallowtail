# 058 Caller-Bounded Interactive Session Cleanup

Status: ready; operator accepted the v0.4 breaking close seam
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

- [ ] the public interactive-session close seam requires caller-supplied host
      services and one caller-selected cleanup deadline
- [ ] no old unbounded close path, default timeout, compatibility shim, or
      guessed duration-to-tick conversion remains callable
- [ ] close, open-abort, turn-expiry interrupt, escalation, pump/task join,
      credential release, and resource release return by the same observable
      hard cleanup boundary
- [ ] deadline expiry produces an honest failed or degraded cleanup result and
      cannot be reported as clean
- [ ] every production implementation and consumer fixture migrates in one
      coordinated public-API change
- [ ] API evidence, guides, examples, matrices, changelog, and release audit
      inputs state the v0.4 break exactly

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
