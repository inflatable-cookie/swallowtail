# 044 Anthropic Direct Conformance

Status: completed
Owner: Tom
Updated: 2026-07-20
Milestone: `../013-anthropic-direct-inference-proof.md`

## Objective

Prove the Anthropic driver against the hosted direct profile and close the
first cross-adapter tranche.

## Scope

- hosted direct profile, cancellation limits, errors, usage, rate, cleanup,
  redaction, no-process behavior, and optional live catalogue check

## Out Of Scope

- mandatory live inference
- consumer routing policy

## Acceptance Criteria

- [x] default QA is credential-free
- [x] live checks are explicit and separately gated
- [x] usage and limits never trigger retry or fallback
- [x] full QA passes

## Evidence

- deterministic fake endpoint is the default gate; no live credential is read
- no live check is registered until an explicit operator-owned endpoint and
  credential gate exists
- the provider-neutral hosted-direct profile now covers explicit output bounds
  alongside no-process, no-resource, endpoint, credential, and evidence rules
- adapter tests prove one attempt across success, error, unknown event, and
  cancellation paths; usage and rate observations have no control behavior
- credential release is counted after catalogue, success, provider failure,
  unknown-event success, and cancellation cleanup
- full repository QA passes with 168 tests; the separately gated OpenCode
  installed probe remains ignored by default
- doctor returns the pre-existing 19 oversized-file findings: 12 warnings and
  7 errors; no Anthropic file adds a finding

## Validation

- focused conformance tests
- `effigy qa`
- `effigy doctor`
- `git diff --check`

## Stop Conditions

- live auth becomes required for deterministic acceptance
- provider failure distinctions cannot be normalized safely

## Auto-Continuation

No. Close roadmap 013 and reassess ACP readiness.
