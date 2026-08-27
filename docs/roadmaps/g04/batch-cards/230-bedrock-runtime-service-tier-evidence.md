# 230 Bedrock Runtime Service-Tier Evidence

Status: ready
Owner: Tom
Created: 2026-08-27
Milestone: [g04.082 Parallel Per-Route Feature Qualification](../082-parallel-per-route-feature-qualification.md)
Depends on: Bedrock Runtime prepared facade; g04.081 closeout

## Goal

Freeze exact Bedrock Runtime `performanceConfig.latency` and `serviceTier`
request, model, region, account, response, and failure truth, then promote
Research 231 with a non-empty deliver-now table or an honest empty set.

## Work

1. [ ] Keep route `bedrock.runtime`, `ConverseStream`, delegated cloud
       identity, one-request structured run, and current output bound unchanged.
2. [ ] Reconcile evidence for the public `SDK_VERSION = 1.136.0` claim against
       the Cargo pin `aws-sdk-bedrockruntime = 1.139.0`. Record the mismatch;
       do not change either surface or silently treat one as the other.
3. [ ] Freeze exact official service model, SDK generated types, API reference,
       and source for `performanceConfig.latency` and `serviceTier`: values,
       omission, request encoding, response fields, validation errors, and
       interaction with `ConverseStream`.
4. [ ] Determine exact model, region, inference-profile, account, entitlement,
       and capacity dependencies. Build a closed preparation-time table only
       when it does not depend on credentials or mutable remote facts.
5. [ ] Separate requested, SDK-built, service-accepted, effective, returned,
       billed, and latency-observed truth. A product label such as Fast is not
       evidence.
6. [ ] Audit prepared input/evidence, facade, model selection, SDK invocation,
       decoder, error mapping, fixtures, guide, matrices, and API baseline
       without changing production surfaces.
7. [ ] Prove omission retains the current SDK builder call and request behavior.
8. [ ] Promote Research 231 and complete the reserved lane log. Do not edit
       shared milestone, inventory, programme, triage, matrices, or indexes.

## Acceptance Criteria

- [ ] SDK/service-version mismatch and exact selected evidence point are explicit
- [ ] exact field/value/model/profile table or honest empty set is recorded
- [ ] account/region/entitlement facts never become static capability claims
- [ ] omission, failure, returned-state, billing, and observed latency are bounded
- [ ] no production code, public API, shared authority, currentness, release,
      merge, rollover, or g04 closure changes

## Validation

```sh
effigy validate:focused swallowtail-adapter-bedrock
effigy qa:northstar
git diff --check
```

## Stop Conditions

- the SDK claim/pin mismatch prevents an honest exact evidence point
- eligibility depends on mutable account, region, capacity, entitlement, or
  remote catalogue facts that preparation cannot bind
- service acceptance or returned tier can silently substitute without a
  bounded disposition
- proof needs credentials, AWS calls, paid work, dependency updates, or a
  shared-contract change

## Out Of Scope

Thinking fields, tools, guardrails, catalogue route, SDK currentness repair,
production binding, live AWS work, release, merge, shared closeout, rollover,
or g04 closure.
