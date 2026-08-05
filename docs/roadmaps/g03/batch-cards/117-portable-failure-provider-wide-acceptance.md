# 117 Portable Failure Provider-Wide Acceptance

Status: completed
Owner: Tom
Created: 2026-08-05
Milestone: `../041-portable-failure-classification.md`
Depends on: card 116

## Goal

Close the common consumer interface, route fallback, guidance, and package
evidence for portable failures.

## Scope

1. Add cross-route core/runtime conformance evidence.
2. Document one consumer handling path with exact diagnostic escape hatch.
3. Reconcile architecture, contract, roadmap, and log truth.
4. Run focused, affected-package, docs, and route validation.

## Validation

- focused validation for changed common and adapter packages
- affected-package verification for changed common and adapter packages
- `effigy package:api` candidate diff review
- `effigy qa:docs`
- `effigy qa:routes`

## Stop Conditions

- stop if any production route lacks the honest unknown fallback
- stop if the guide requires adapter-owned diagnostic matching

## Auto-Continuation

No. Close g03.041 and return to the evidence gate.

## Completion

- shared testkit conformance proves the unknown fallback and evidence-backed
  classified path without route-owned code matching.
- the consumer guide uses `TerminalOutcome::failure`, preserves cleanup as a
  separate outcome, and keeps exact diagnostics as a support escape hatch.
- focused validation passed across common crates and all changed adapters.
- affected-package verification passed for common crates and all changed
  adapter packages; Kimi Platform fixtures are package-local and compile from
  the extracted crate.
- the public-API selector reports the expected additive held-candidate diff;
  the release baseline was not mutated outside a candidate lane.
- docs and route validation passed without authenticated provider work.
