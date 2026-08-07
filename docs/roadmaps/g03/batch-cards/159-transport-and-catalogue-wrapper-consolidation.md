# 159 Transport And Catalogue Wrapper Consolidation

Status: planned
Owner: Tom
Created: 2026-08-08
Milestone: `../052-shared-adapter-scaffolding.md`
Depends on: card 158

## Goal

Consolidate the duplicated curl transport wrappers and catalogue parse and
paginate families where the shared shape is net-positive.

## Scope

1. Extract the shared transport wrapper (13 adapters, ~50% identical lines)
   into `swallowtail-runtime` or `swallowtail-transport-acp-remote`,
   parameterized by endpoint, access profile, and bounds.
2. Extract the catalogue parse and paginate helpers (30 files) where the
   shared shape is provider-neutral.
3. Migrate adapters in family batches with unchanged behavior.

## Out Of Scope

- protocol codecs, framing, or bounds changes
- provider-specific catalogue semantics (stay adapter-local)
- public API changes

## Acceptance

- [ ] the shared wrappers have focused tests
- [ ] migrated adapters pass focused and extracted-package proof with an
      unchanged public API baseline
- [ ] transport and catalogue duplication shrinks by the measured amounts

## Stop Conditions

- stop if a migrated adapter changes endpoint, streaming, or catalogue
  behavior

## Auto-Continuation

Yes, to card 160 after acceptance.

## Validation

- focused validation per migrated adapter; `effigy package:verify-affected`
  per batch
- `effigy qa:routes` after any catalogue or feature-matrix touch
