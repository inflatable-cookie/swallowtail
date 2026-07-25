# 002 Release Contract And Package Topology

Status: completed
Owner: Tom
Created: 2026-07-24
Milestone: `../001-release-boundary-and-package-readiness.md`

## Objective

Promote the evidence-backed release, package, compatibility, and authority
boundary before changing Cargo manifests.

## Governing Refs

- card 001 outputs
- Research 033
- provisional Spec 004
- Contracts 001, 002, 004, 005, and 029

## Scope

1. Promote the selected package and dependency topology into architecture.
2. Add one durable release contract covering:
   - public and intentionally unpublished crates
   - version coordination and dependency requirements
   - pre-1.0 compatibility and deprecation
   - MSRV and supported-target changes
   - package contents and reproducibility
   - changelog, API-diff, docs, and consumer evidence
   - human authority for registry, tag, and release mutation
3. Archive Spec 004 after promotion.
4. Rebaseline cards 003-004 against the promoted contract.

## Acceptance Criteria

- [x] every policy choice is traceable to card 001 evidence or operator input
- [x] provider-interface compatibility remains separate
- [x] package and release mutation authority is explicit
- [x] no manifest or release implementation occurs before promotion
- [x] card 003 becomes ready only from durable rules

## Validation

- contract and architecture diff audit
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy doctor` delta review
- `git diff --check`

## Stop Conditions

- card 001 leaves a material policy tie
- the selected package graph cannot be published without changing ownership
- the contract would imply API 1.0 or an unauthorized release

## Auto-Continuation

Yes, only after card 003 is rebaselined to ready from the promoted contract.

## Evidence

- The operator approved provisional Spec 004 on 2026-07-24.
- `docs/architecture/release-and-package-topology.md` records the accepted
  23-package structure and exact three-stage dependency order without claiming
  manifest realization.
- Contract 036 governs coordinated pre-1.0 compatibility, internal registry
  requirements, MSRV, targets, package evidence, consumer handoffs, and human
  release authority.
- Research 033 is promoted and Spec 004 is archived.
- Card 003 is rebaselined from Contract 036 and ready.
