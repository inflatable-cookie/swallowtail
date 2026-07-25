# 131 Kimi Capability Range Conformance And Closeout

Status: completed
Owner: Tom
Updated: 2026-07-24
Milestone: `../043-kimi-code-capability-range.md`

## Objective

Prove Kimi capability evolution without widening ACP, persistent-session,
access, or isolation authority.

## Scope

- exact baseline, latest-qualified, rejection, and unverified-newer points
- legacy boolean and declared effort-level reasoning setup
- unchanged persistent ACP profile and Contract 034 assertion pack
- local and remote-authoritative executable discovery and session execution
- empty-options new, load, resume, replay, write, cancellation, disconnect,
  redaction, and joined cleanup regressions
- default deterministic QA, doctor delta, currentness, and closeout log
- no live authentication

## Acceptance Criteria

- [x] two exact qualified release behaviors share one public session shape
- [x] qualified and unverified-newer execution remain distinct
- [x] dynamic option drift fails without fallback
- [x] existing Kimi persistent lifecycle remains unchanged when options are
      empty
- [x] no provider-specific option record leaks into core or runtime
- [x] full validation passes apart from documented structural debt

## Validation

- focused Kimi range and conformance tests
- workspace all-target check
- workspace warnings-denied clippy
- `effigy qa`
- `effigy doctor` delta review
- `git diff --check`

## Auto-Continuation

No. Return to provider coverage after the range closes.

## Outcome

Exact `0.28.1`, `0.29.0`, and unverified-newer behavior dispatch pass under
both authoritative host topologies. Missing, ambiguous, malformed,
unsupported, rejected, unconfirmed, and drifted reasoning shapes fail without
fallback and join all owned work.

The unchanged persistent ACP profile and Contract 034 assertions pass.
Provider option ids remain private to the Kimi adapter. Roadmap 043 is closed;
card 132 is the next provider-coverage evidence task.
