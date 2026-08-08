# 156 Shared Probe, Binding-Parse, And Stage Helpers

Status: completed
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

- [x] shared helpers exist with their own focused tests
- [x] pi and oh-my-pi pass focused, affected-package, and extracted-package
      proof with an unchanged public API baseline
- [x] their discovery and failure module duplication measurably shrinks

## Stop Conditions

- stop if a migrated adapter changes classification output or discovery
  behavior

## Auto-Continuation

Yes, to card 157 after acceptance.

## Validation

- `effigy validate:focused swallowtail-runtime swallowtail-testkit swallowtail-adapter-pi swallowtail-adapter-oh-my-pi`
- `effigy package:verify-affected` for the pilot pair
- `effigy package:api`

## Completion Evidence

- new `swallowtail-runtime/src/installed_discovery.rs` owns three
  provider-neutral helpers (runtime gained `futures-channel` and `semver`
  dependencies):
  - `parse_semantic_version_binding(axis, value)` — the total
    empty/length/trim/control/semver binding parse that the thirteen
    sibling adapters previously duplicated
  - `probe_installed_executable_version(...)` — the full bounded probe
    scaffold: host-service and axis validation, task-service spawn and
    join, `--version` process start, bounded stdout capture, deadline and
    cancellation races, stop-and-classify on every path, exit and
    classification, with the adapter's static diagnostic-code namespace
    supplied through `installed_probe_codes!("swallowtail.<adapter>")`
  - `probe_runtime_failure`, `probe_outcome_failure`, and
    `preparation_failure` — the shared discovery stage mappers
- pilot migration: pi and oh-my-pi discovery dropped from ~250 lines each
  to ~75 (the probe machinery lives once in runtime), their prepared
  failure mappers delegate to the shared mappers, and their public binding
  helpers delegate to `parse_semantic_version_binding` with unchanged
  signatures; 460 adapter lines deleted
- four focused runtime tests cover the parse helper and both stage mappers
- behavior parity: all existing pi and oh-my-pi tests pass unchanged
  (including the exact-banner and bare-semver parser tests), focused
  validation passes for all four packages, affected-package proof passes
  for the pair, the semantic API baseline is unchanged for the two adapters
  (runtime additions captured in the regenerated v0.3.0 baseline), and the
  workspace round passes 1,499 tests
