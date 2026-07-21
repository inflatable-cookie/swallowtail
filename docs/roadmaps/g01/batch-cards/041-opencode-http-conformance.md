# 041 OpenCode HTTP Conformance

Status: completed
Owner: Tom
Updated: 2026-07-20
Milestone: `../012-opencode-http-harness-proof.md`

## Objective

Prove the OpenCode adapter against shared harness and hosted-network contracts.

## Scope

- deterministic network-harness profile
- local and remote-authoritative host identity
- cancellation, deadline, disconnect, redaction, and cleanup
- optional installed unauthenticated health/schema probe

## Out Of Scope

- authenticated provider inference in default QA
- consumer adoption

## Acceptance Criteria

- [x] shared profile passes without weakened assertions
- [x] optional installed probe is separately gated
- [x] no provider credential or raw payload reaches diagnostics
- [x] full QA passes

## Validation

- deterministic local and remote-authoritative HTTP harness tests pass
- the sixth synthetic profile proves attached network-harness lifecycle
- `effigy qa` passes with 150 tests
- `effigy doctor` retains the known 19 findings and 7 errors
- `git diff --check` passes

The optional installed probe is excluded from default QA. Run
`effigy probe:opencode-installed` only with an explicit
`SWALLOWTAIL_OPENCODE_PROBE_ENDPOINT` pointing at an operator-started,
unauthenticated server.

## Stop Conditions

- fixture and installed behavior diverge materially
- cleanup can detach network reader work

## Auto-Continuation

No. Close roadmap 012 and confirm card 042 remains the right direct lane.
