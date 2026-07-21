# 038 Local Hosted Services And Conformance

Status: completed
Owner: Tom
Updated: 2026-07-20
Milestone: `../011-hosted-transport-foundations.md`

## Objective

Implement concrete local endpoint and credential approvals, prove the hosted
foundation as one batch, and close roadmap 011.

## Governing References

- Contract 010
- Contract 011
- Contract 014
- Roadmap 011

## Scope

- local host-approved endpoint mapping
- local host-approved secret and delegated credential mapping
- exact audience enforcement
- explicit lease release and redaction
- hosted direct and attached-network foundation fixtures
- architecture, roadmap, log, and front-door closeout

## Out Of Scope

- durable credential store
- network request execution
- live provider authentication
- OpenCode or Anthropic adapter

## Implementation Steps

1. Add concrete local endpoint and credential service behavior.
2. Test missing approval, audience mismatch, redaction, and release.
3. Run hosted profile and cross-host mismatch fixtures.
4. Run full repository QA once for cards 036-038.
5. Close roadmap 011 and ready card 039 if no planning gap remains.

## Acceptance Criteria

- [x] local services resolve only approved opaque references
- [x] audience mismatch fails without secret exposure
- [x] delegated credentials expose no bytes
- [x] hosted fixtures require no process service or live network
- [x] full QA passes or failures are recorded honestly

## Validation

- focused local-host and testkit tests
- `effigy qa`
- `effigy doctor`
- `git diff --check`

## Evidence Required

- deterministic service tests
- final test count
- dependency and secret/redaction scan

## Evidence

- local endpoint approvals map opaque references to exact redacted driver
  values under one audience.
- local secret and delegated credential approvals reject unknown or mismatched
  audiences and track issued scope/reference/audience leases through release.
- the hosted conformance profile proves endpoint/credential binding, no-resource
  direct runs, provider-evidence separation, and no process requirement.
- full repository QA passes with 129 tests.
- `effigy doctor` remains at the recorded structural-debt baseline: 19
  oversized-file findings, including 7 errors; this batch adds none.

## Stop Conditions

- concrete service needs ambient credential-store scanning
- secret or endpoint appears in failure output
- full validation exposes a contract-changing failure

## Auto-Continuation

No. Close the foundation batch, make card 039 the sole next task, and report.
