# 156 Shared Probe, Binding-Parse, And Stage Helpers

Status: planned
Owner: Tom
Created: 2026-08-08
Milestone: `../052-shared-adapter-scaffolding.md`
Depends on: card 155

## Goal

Extract the provider-neutral installed-executable probe, version-binding
parse, and failure-stage mapping helpers into shared crates, with pi and
oh-my-pi as the pilot migration.

## Scope

1. Add to `swallowtail-runtime` or `swallowtail-testkit` (per the
   provider-neutrality rule):
   - an installed-executable probe scaffold parameterized by a version-format
     parser, claim, and axis, owning bounded output capture, cancellation,
     deadline, process join, and classification outcome
   - a total `InterfaceVersionBinding` parse helper (complementing card 148)
   - a `PreparationStage` failure mapper
2. Migrate the pi and oh-my-pi discovery, selection, and failure modules to
   the shared helpers (the pair is 42% identical today).
3. Prove behavior parity: unchanged public API baseline and the existing
   fixture corpora pass unchanged.

## Out Of Scope

- provider-specific version regexes, claims, or request builders (stay
  adapter-local)
- other adapter families (cards 157-160)
- public API or diagnostic-code changes

## Acceptance

- [ ] shared helpers exist with their own focused tests
- [ ] pi and oh-my-pi pass focused, affected-package, and extracted-package
      proof with an unchanged public API baseline
- [ ] their discovery and failure module duplication measurably shrinks

## Stop Conditions

- stop if a migrated adapter changes classification output or discovery
  behavior

## Auto-Continuation

Yes, to card 157 after acceptance.

## Validation

- `effigy validate:focused swallowtail-runtime swallowtail-testkit swallowtail-adapter-pi swallowtail-adapter-oh-my-pi`
- `effigy package:verify-affected` for the pilot pair
- `effigy package:api`
