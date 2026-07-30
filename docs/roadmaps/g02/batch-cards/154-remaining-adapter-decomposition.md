# 154 Remaining Adapter Decomposition

Status: completed
Owner: Tom
Created: 2026-07-30
Milestone: `../045-error-level-structural-health-stabilization.md`

## Goal

Remove the final six adapter error-level findings and the residual route-matrix
high fragment.

## Scope

1. Split Pi prepared-facade tests and driver validation.
2. Split Alibaba run and catalogue concerns.
3. Split DeepSeek run concerns.
4. Split xAI catalogue concerns.
5. Split provider-route base validation by input and route-record concern.

## Acceptance Criteria

- [x] all seven remaining error findings are removed
- [x] direct inference, callback, and catalogue behavior remains unchanged
- [x] route-matrix output and failure behavior remains unchanged
- [x] public declaration hashes remain unchanged
- [x] focused package tests and warnings-denied clippy pass

## Validation

- focused Pi, Alibaba, DeepSeek, and xAI tests
- provider-route matrix
- warnings-denied clippy for touched crates
- public-API and doctor delta checks

## Stop Conditions

- Stop if extraction would create a shared provider codec or policy layer.
- Stop if route-local usage, reasoning, or cleanup truth changes.
- Do not touch warning-only files without a required private seam.

## Auto-Continuation

Yes. Continue to card 155 after focused validation.

## Evidence

- Pi prepared-facade cases now split by catalogue, session/run, and shared
  fixture support. Driver validation splits private common, attachment, and
  plan checks behind unchanged entry points.
- Alibaba and DeepSeek structured-run code now splits start validation, handle
  state, and event pumping without changing request, callback, cancellation,
  usage, or cleanup behavior.
- Alibaba and xAI catalogue code now splits driver dispatch, transport, bounded
  protocol parsing, and tests. Public declarations remain in their original
  files.
- Provider-route validation now separates route records, aggregate inventory,
  and classification inputs while preserving the same shared execution
  namespace and failure messages.
- Focused validation passed 112 tests across Pi, Alibaba Model Studio,
  DeepSeek, and xAI.
- Focused warnings-denied clippy, route matrices, formatting, Python syntax,
  and the 24-crate public-API declaration baseline passed.
- Doctor reports 142 warnings and zero errors. All seven assigned findings are
  removed.
