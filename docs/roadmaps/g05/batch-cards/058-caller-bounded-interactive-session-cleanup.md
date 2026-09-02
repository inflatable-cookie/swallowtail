# 058 Caller-Bounded Interactive Session Cleanup

Status: planned; awaiting operator decision on breaking public API
Owner: Tom
Created: 2026-09-02
Milestone: `../023-claude-sdk-shared-lifecycle-prerequisites.md`
Depends on: Contract 010; Contract 019; operator API decision; card 057 review

## Goal

Make interactive-session close and every post-expiry cleanup/join path return
under a caller-selected host deadline, without inventing monotonic tick units
or leaving an unbounded compatibility path callable.

## Proposed Direction

Use the v0.4.0 breaking-release window to require close-time host services and
a caller-selected cleanup deadline on the shared interactive-session handle.
Apply the same evidence rule to cleanup after open and turn expiry. Do not add
a default-method shim that silently calls the old unbounded close.

This direction is a recommendation, not authority. The card cannot become
ready until the operator accepts the breaking public close seam or selects a
different caller-bounded design.

## Planned Scope

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

## Acceptance Gate

Compile the full validation and review oracle only after the operator decision
fixes the public shape. Until then this card remains planned and PR 188 remains
paused.
