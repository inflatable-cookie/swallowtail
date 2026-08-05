# 123 Integration Guide Coverage Acceptance

Status: completed
Owner: Tom
Created: 2026-08-05
Milestone: `../042-complete-integration-guide-system.md`
Depends on: card 122

## Goal

Make route, feature, guide, and example completeness deterministic and close
the documentation programme.

## Scope

1. Add a coverage check over production routes, feature headers, guide owners,
   and examples.
2. Mark guide-map rows complete only after Contract 052 review.
3. Reconcile guide, architecture, contract, roadmap, and log indexes.
4. Run docs, examples, routes, focused, and affected-package evidence.

## Validation

- guide coverage selector
- `effigy check:examples`
- `effigy qa:docs`
- `effigy qa:routes`
- focused and affected-package checks for packages with new examples

## Auto-Continuation

No. Close g03.042 and return to the evidence gate.

## Completion

- added `effigy qa:guides` and made it part of ordinary docs QA
- the selector compares the 33 production routes, 34 matrix features, nine
  additional portable/operator surfaces, complete guide-map rows, guide-index
  membership, local guide files, and existing Rust examples
- reconciled the guide, architecture, contract, roadmap, log, script, and root
  front doors
- focused validation passed 96 tests for Antigravity, Cursor, and Grok
- independently assembled and compiled the three packages that gained examples
- examples, docs, guide coverage, route matrices, formatting, and diff checks
  passed
- no live or authenticated provider work ran
