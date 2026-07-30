# 154 Remaining Adapter Decomposition

Status: ready
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

- [ ] all seven remaining error findings are removed
- [ ] direct inference, callback, and catalogue behavior remains unchanged
- [ ] route-matrix output and failure behavior remains unchanged
- [ ] public declaration hashes remain unchanged
- [ ] focused package tests and warnings-denied clippy pass

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
