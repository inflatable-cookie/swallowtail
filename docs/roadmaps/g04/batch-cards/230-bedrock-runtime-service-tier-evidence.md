# 230 Bedrock Runtime Service-Tier Evidence

Status: completed
Owner: Tom
Created: 2026-08-27
Updated: 2026-08-27
Milestone: [g04.082 Parallel Per-Route Feature Qualification](../082-parallel-per-route-feature-qualification.md)
Depends on: Bedrock Runtime prepared facade; g04.081 closeout
Research: [231 Bedrock Runtime Service-Tier Evidence](../../../research/231-bedrock-runtime-service-tier-evidence.md)

## Goal

Freeze exact Bedrock Runtime `performanceConfig.latency` and `serviceTier`
request, model, region, account, response, and failure truth, then promote
Research 231 with a non-empty deliver-now table or an honest empty set.

## Work

1. [x] Keep route `bedrock.runtime`, `ConverseStream`, delegated cloud
       identity, one-request structured run, and current output bound unchanged.
2. [x] Reconcile evidence for the public `SDK_VERSION = 1.136.0` claim against
       the Cargo pin `aws-sdk-bedrockruntime = 1.139.0`. Record the mismatch;
       do not change either surface or silently treat one as the other.
3. [x] Freeze exact official service model, SDK generated types, API reference,
       and source for `performanceConfig.latency` and `serviceTier`: values,
       omission, request encoding, response fields, validation errors, and
       interaction with `ConverseStream`.
4. [x] Determine exact model, region, inference-profile, account, entitlement,
       and capacity dependencies. Build a closed preparation-time table only
       when it does not depend on credentials or mutable remote facts.
5. [x] Separate requested, SDK-built, service-accepted, effective, returned,
       billed, and latency-observed truth. A product label such as Fast is not
       evidence.
6. [x] Audit prepared input/evidence, facade, model selection, SDK invocation,
       decoder, error mapping, fixtures, guide, matrices, and API baseline
       without changing production surfaces.
7. [x] Prove omission retains the current SDK builder call and request behavior.
8. [x] Promote Research 231 and complete the reserved lane log. Do not edit
       shared milestone, inventory, programme, triage, matrices, or indexes.

## Acceptance Criteria

- [x] SDK/service-version mismatch and exact selected evidence point are explicit
- [x] exact field/value/model/profile table or honest empty set is recorded
- [x] account/region/entitlement facts never become static capability claims
- [x] omission, failure, returned-state, billing, and observed latency are bounded
- [x] no production code, public API, shared authority, currentness, release,
      merge, rollover, or g04 closure changes

## Outcome

Research 231 promotes an **empty deliver-now set**. Exact AWS and SDK evidence
shows optional `performanceConfig.latency` and `serviceTier` on
`ConverseStream`, but eligibility, acceptance, returned tier, billing, and
observed latency depend on model, region, inference profile, quota, capacity,
and account facts the route-open Runtime facade cannot close at preparation
time. Public SDK constant `1.136.0` and Cargo pin `1.139.0` disagree; tier-field
generated shapes are identical across both versions. Omission retains the
current builder call (`model_id`, `messages`, `inference_config.max_tokens`
only).

Frozen corpus:
`crates/swallowtail-adapter-bedrock/tests/fixtures/bedrock-runtime-service-tier-evidence/`.

## Validation

```sh
effigy validate:focused swallowtail-adapter-bedrock
effigy qa:northstar
git diff --check
```

## Stop Conditions

- the SDK claim/pin mismatch prevents an honest exact evidence point — **not
  triggered**; mismatch recorded, shapes identical
- eligibility depends on mutable account, region, capacity, entitlement, or
  remote catalogue facts that preparation cannot bind — **triggered**
- service acceptance or returned tier can silently substitute without a
  bounded disposition — **bounded in Research 231; no row admitted**
- proof needs credentials, AWS calls, paid work, dependency updates, or a
  shared-contract change — **not used**

## Out Of Scope

Thinking fields, tools, guardrails, catalogue route, SDK currentness repair,
production binding, live AWS work, release, merge, shared closeout, rollover,
or g04 closure.
