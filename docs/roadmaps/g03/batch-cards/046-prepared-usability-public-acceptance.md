# 046 Prepared Usability Public Acceptance

Status: completed
Owner: Tom
Created: 2026-07-31
Milestone: `../017-prepared-facade-multi-consumer-usability.md`
Depends on: card 045

## Goal

Accept the two public conveniences, align guidance with bound operations, and
record the remaining consumer adoption without editing consumer repositories.

## Acceptance Criteria

- [x] the compile-tested Codex example uses canonical ChatGPT access
- [x] guidance distinguishes bound normal paths from low-level extraction
- [x] the public API baseline records both intentional additions
- [x] affected extracted packages compile
- [x] a Nucleus handoff names only helper and bound-operation adoption
- [x] docs, Northstar, focused, API, package, and diff checks pass
- [x] no provider or consumer effect runs

## Validation

- `effigy validate:focused swallowtail-host-local swallowtail-adapter-codex`
- `effigy package:verify-affected swallowtail-host-local swallowtail-adapter-codex`
- `effigy package:api`
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`
- no broad workspace or authenticated provider suite

## Auto-Continuation

No. Return to the g03 compatibility-maintenance checkpoint.

## Evidence

- compile-tested example composes canonical ChatGPT access with separate
  caller-supplied status and uses bound prepared operations
- the guide documents local deadline derivation, exact access authority, and
  the low-level escape boundary
- focused validation: 174 passed
- affected extracted-package proof: two packages passed
- public API declaration baseline: 26 crates passed
- docs, Northstar, and diff checks passed
- no authenticated or consumer effect ran
