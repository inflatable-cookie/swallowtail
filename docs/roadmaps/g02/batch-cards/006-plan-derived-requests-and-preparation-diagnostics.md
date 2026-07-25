# 006 Plan-Derived Requests And Preparation Diagnostics

Status: completed
Owner: Tom
Created: 2026-07-24
Milestone: `../002-prepared-consumer-integration-boundary.md`

## Objective

Implement the provider-neutral request and diagnostic primitives required by
prepared integrations.

## Governing Refs

- Contract 037
- Contracts 008-010, 013, 029, 032-033
- completed card 005

## Scope

1. Replace implicit interactive request plan echoes with plan-derived or
   explicit construction.
2. Cover session access, provider-state, and harness-configuration agreement
   without copying consumer intent.
3. Add safe typed preparation stages and one primary diagnostic chain.
4. Preserve raw process, provider, path, environment, and credential redaction.
5. Add provider-neutral fixtures for missing, mismatched, and successful
   preparation.
6. Make the clean pre-release API change without aliases or shims.

## Acceptance Criteria

- [x] an interactive request cannot silently choose a policy different from
      its plan
- [x] explicit low-level construction remains possible
- [x] diagnostic stages distinguish target, spawn, output, exit, parse,
      classification, access, preflight, and cleanup
- [x] stable formatting remains safe
- [x] existing drivers fail before effects on mismatch
- [x] focused API and conformance tests pass

## Validation

- focused core and runtime tests
- testkit request-plan and diagnostic assertions
- public API diff and warnings-denied clippy
- `effigy check:rust`
- `git diff --check`

## Evidence Required

- exact public API change classification
- request-plan success and mismatch fixtures
- staged failure and redaction matrix
- batch log and card 007 readiness assessment

## Stop Conditions

- derivation hides consumer-selected policy
- a diagnostic requires raw payload exposure
- a driver needs an operation-shape-specific exception in shared code
- the change cannot be made cleanly before first publication

## Auto-Continuation

Yes, only after card 007 is rebaselined to ready from the completed shared
runtime surface.

## Closeout

Completed 2026-07-24.

- interactive open, load, and resume requests now carry one explicit or
  plan-derived `SessionPlanAgreement`
- session access, provider state, and harness configuration have no unrelated
  constructor defaults
- preparation failures carry one of nine safe stages and an optional redacted
  causal chain
- access evidence retains observed or caller-asserted provenance without
  changing `AccessStatus`
- every production session driver checks request-plan agreement before effects
- provider-neutral fixtures cover derivation, mismatch, missing plan state,
  provenance, stages, chaining, and redaction

The public API change is intentionally breaking for the unreleased `0.1.0`
baseline: request constructors gained a required agreement, the old policy
setters were removed, and new preparation records were added. There is no
published predecessor and no compatibility shim.

Card 007 passed its readiness gate and completed in the same validation batch.
