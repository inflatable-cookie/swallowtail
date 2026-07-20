# Soundcheck Adoption Handoff

Status: completed
Owner: Tom
Roadmap: 007 Soundcheck Structured-Run Readiness
Updated: 2026-07-19

## Goal

Prove Swallowtail covers Soundcheck's reusable connector mechanics and record a
bounded downstream replacement and rollback plan.

## Scope

- parity fixture for model selection, reasoning, schema, screenshot, progress,
  timeout, cancellation, and terminal output
- mapping from current Soundcheck connector calls to public Swallowtail APIs
- consumer-owned validation/repair/ranking/apply boundary
- pinned dependency, feature gate, fallback removal, and rollback sequence
- downstream test and manual acceptance checklist

## Out Of Scope

- edits in the Soundcheck repository
- a permanent dual implementation
- release publication
- Nucleus adoption

## Acceptance Criteria

- no product type is required by the Swallowtail fixture
- the handoff replaces one connector seam rather than moving the module
- rollback is an explicit dependency/configuration reversal
- legacy connector removal occurs only after Soundcheck-owned parity tests pass
- roadmap 007 can close without claiming consumer adoption is complete

## Validation

- public-API parity fixture
- documentation and dependency audit
- `effigy qa`
- `git diff --check`

## Stop Condition

Stop if parity still requires a Soundcheck-owned policy inside Swallowtail.

## Closeout

- a provider-neutral public-API fixture proves catalog selection, provider
  default metadata, reasoning, schema, image, search policy, ordered progress,
  terminal output, deadline, cancellation, joined cleanup, and redaction
- controlled exec runs now ignore ambient user configuration and rules, deny
  approvals, suppress tool subprocess environment inheritance, and permit the
  host-approved temporary non-Git resource used by the consumer shape
- the handoff maps one connector seam, exact per-turn policies, host wiring,
  an exact-revision dependency, a temporary compile-time gate, downstream
  validation, rollback, and legacy-removal conditions
- Soundcheck source and product policy remain unchanged
- the downstream artifact is `../soundcheck-adoption-handoff.md`
