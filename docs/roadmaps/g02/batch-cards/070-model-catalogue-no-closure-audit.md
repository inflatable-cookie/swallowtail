# 070 Model Catalogue No-Closure Audit

Status: completed
Owner: Tom
Created: 2026-07-27
Milestone: `../021-model-catalogue-coverage.md`

## Objective

Recheck every remaining model-catalogue `No`, implement every qualified
machine-readable source, and replace false binary gaps with exact interface
semantics.

## Scope

1. Revalidate every former `No` against exact or current authoritative
   evidence.
2. Correct the Qwen and Alibaba classifications from Research 042.
3. Add Qwen safe-mode stream-JSON catalogue preparation and production
   execution.
4. Add Alibaba deployable-model control-plane catalogue preparation and
   production execution.
5. Update the solution CSV, route guide, architecture, contract, roadmap, and
   front-door currentness.
6. Retain ACP negotiated options, caller-supplied choices, and not-applicable
   serving or managed-agent shapes without forcing them through
   `ModelCatalogDriver`.

## Acceptance Criteria

- [x] every qualified machine-readable source has a Swallowtail path
- [x] the solution matrix contains no unexplained `No`
- [x] Qwen discovery opens no model session and claims no OS sandbox
- [x] Alibaba deployment candidates do not imply Conversations invocability
- [x] non-catalogue classifications describe the selected interface exactly
- [x] fixtures require no live credential, provider call, model invocation,
      deployment, or publication

## Evidence

- Research 043
- Qwen exact `0.19.11` and current `0.21.0` control-plane source
- Alibaba official 2026-06-06 deployable-model API reference
- Qwen prepared process fixture and hosted-provider bounded decoder tests
- 21-row provider-solution feature matrix with zero `No` values

## Auto-Continuation

No. Return to the operator-held lifecycle adoption decision.
