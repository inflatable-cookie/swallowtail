# 050 llama.cpp Attached Conformance

Status: completed
Owner: Tom
Updated: 2026-07-20
Milestone: `../015-llama-cpp-attached-runtime-proof.md`

## Objective

Prove the attached self-hosted runtime profile and close the initial transport-
diversity runway.

## Scope

- attached profile, external ownership, route discovery, capability evidence,
  cancellation, deadline, failure, redaction, and cleanup

## Out Of Scope

- owned serving profile
- Monkey adoption

## Acceptance Criteria

- [x] shared attached profile passes
- [x] external service is never stopped
- [x] full QA passes
- [x] next coverage priorities are re-ranked from evidence

## Validation

- focused conformance tests
- `effigy qa`
- `effigy doctor`
- `git diff --check`

## Evidence

- the provider-neutral attached-self-hosted profile passes common assertions
  plus external-service preservation
- local and remote-authoritative hosts use the same public catalogue seam
- success, capability drift, one-attempt failure, cancellation, deadline,
  redaction, and joined cleanup are deterministic
- no process or serving-lifecycle role exists; the fake server remains
  reachable after every close path
- full QA passes with 207 tests; doctor remains at the known 19 findings
- next information priority is WebSocket direct inference, then a second ACP
  agent; owned serving waits for an explicit Monkey/artifact boundary

## Remaining Risks

- no operator-supplied GGUF was present, so no live b9910 probe ran
- installed b9910 trails upstream b10069; other builds fail closed
- remote endpoints, server API-key auth, router mode, tools, reasoning, schema
  output, and multimodality remain unsupported

## Stop Conditions

- proof crosses the serving/artifact ownership boundary

## Auto-Continuation

No. Return to a provider-coverage planning checkpoint.
