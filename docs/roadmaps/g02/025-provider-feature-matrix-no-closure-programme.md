# 025 Provider Feature Matrix No-Closure Programme

Status: completed
Owner: Tom
Created: 2026-07-28
Depends on: g02.024 and card 060
Vision tags: provider breadth, exact capabilities, evidence-led expansion
Contract refs: 003-039
Planning state: cards 080-083 complete

## Problem

The 21-solution matrix still contains many `No` cells outside model catalogue
and structured run. Some describe real upstream absence. Others may be stale,
unimplemented, or represented by a different operation shape.

Swallowtail needs a CSV-aware, evidence-backed audit before turning individual
columns into implementation lanes.

## Goals

- [x] Inventory every `No` by feature column and provider solution.
- [x] Detect matrix errors against realized public prepared facades.
- [x] Revalidate plausible first-family capabilities against current official provider or
      maintained-project evidence.
- [x] Classify honest absence, not-applicability, contract gaps, and ready
      implementation separately.
- [x] Rank feature families by useful conversions and architectural value.
- [x] Promote the usage-evidence family before implementation.
- [x] Leave no unexplained `No`.

## Non-Goals

- forcing every `No` to `Yes`
- borrowing a capability from another route or solution
- flattening provider-specific operations into generic methods
- assuming realtime, direct, harness, or serving shapes are interchangeable
- consumer edits, live authentication, package publication, or release mutation

## Execution Plan

### Batch 25.1 — Exact Inventory

- [x] Execute card 080.
- [x] Produce one CSV-aware count and cell inventory.
- [x] Correct serving-only claims already contradicted by the realized
      operation shape.

### Batch 25.2 — First Feature Family

- [x] Execute card 081 for Claude Agent ACP, Pi RPC, and OpenCode usage.
- [x] Refresh upstream evidence and promote the cumulative aggregation rule.

### Batch 25.3 — First Implementation Tranche

- [x] Execute card 082 only after contract readiness.
- [x] Use fixture-first prepared paths and exact route identities.

### Batch 25.4 — Closeout And Continuation

- [x] Execute card 083.
- [x] Update matrix counts, package proof, and the next feature-family
      checkpoint.

## Acceptance Criteria

- [x] CSV parsing respects quoted fields and exact solution identities
- [x] every audited `No` has an evidence classification
- [x] unsupported and not-applicable remain distinct
- [x] realized capability and prepared facade claims agree
- [x] first-family temporally unstable claims cite current authoritative evidence
- [x] the first implementation tranche is contract-ready before code
- [x] matrix checks fail on count or classification drift

## Decision Gates

- Ask the operator if feature-family selection would establish product policy.
- Stop if official and maintained-project evidence conflict on route authority.
- Do not implement a feature through credentials, endpoints, or transports not
  already authorized for that solution.
- Keep honest `No` values when the operation shape cannot support the feature.

## Next Planning Checkpoint

Roadmap 026 and card 084 continue with the exact 48-cell generation-control
audit. Do not roll to g03.
