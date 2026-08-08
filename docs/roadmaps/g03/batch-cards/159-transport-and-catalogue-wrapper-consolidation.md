# 159 Transport And Catalogue Wrapper Consolidation

Status: ready
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

- [x] the transport and catalogue duplication is measured precisely
- [x] the topology constraint blocking a shared home is recorded with the
      operator decision surface
- [x] no qualified failure output changed

## Stop Conditions

- stop if a migrated adapter changes endpoint, streaming, or catalogue
  behavior

## Auto-Continuation

Yes, to card 160 after acceptance.

## Validation

- `effigy qa:routes`, `effigy qa:docs`, focused validation for the touched
  packages

## Completion Evidence

- transport wrapper measured: the curl-crate execution driver is the
  duplicated shape, with kimi-platform↔anthropic `transport/io.rs` at 0.94
  similarity, alibaba↔xai `catalogue/driver.rs` at 0.90, and eleven
  adapters depending on the `curl` crate directly
- catalogue helpers measured: the shared slice is the small validation
  family (`bounded_text` in eight files, `optional_bounded_text` in five,
  `optional_u64` in four); the actual model-envelope parsing is
  provider-specific in every adapter
- disposition recorded for both halves, mirroring the card-158 projector
  outcome:
  - a shared curl transport wrapper needs either a new package (a Contract
    036 release-topology change: 28 to 29 packages) or a `curl` dependency
    in runtime, which violates the recorded "only core, futures-core, and
    zeroize dependencies" posture; both are operator decisions
  - the catalogue validation helpers could live in runtime with a
    `serde_json` dependency, but preserving each adapter's exact failure
    codes requires plumbing the code through every call, and the migration
    risk outweighs the roughly sixty-line gain
- no qualified failure output or catalogue behavior changed; `effigy qa:routes`
  and `effigy qa:docs` pass
