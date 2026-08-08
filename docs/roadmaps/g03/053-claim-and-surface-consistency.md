# 053 Claim And Surface Consistency

Status: done
Owner: Tom
Created: 2026-08-08
Generation: g03
Depends on: g03.052
Vision tags: consistency, compatibility truth, maintainability
Contract refs: 003, 029, 037, 047
Planning state: cards 161-164 completed

## Problem

A verified deep audit found compatibility and facade surfaces that mean
different things per adapter:

- version-claim semantics drift within one shared claim API: Codex is the only
  adapter using `InterfaceSupportStatus::Deprecated` for retained-behavior
  windows (`adapter-codex/src/selection.rs:94-111`), while every other
  adapter models old behavior as additional `Maintained` segments (for example
  `adapter-kimi/src/selection.rs:75-85`); a consumer reading support status
  cannot rely on consistent meaning
- claim ids mix dash and dot schemes and window numbering is inconsistent
  across adapters of identical age
- Bedrock and llama.cpp define no compatibility claim at all and rely on
  opaque facades, unlike every hosted peer
- the runtime re-implements one plan/agreement/request skeleton seven times
  (`provider_session_operation`, `provider_session_reconciliation`,
  `provider_session_import`, `provider_run_reconciliation`,
  `provider_recovered_resource_cleanup`, `settled_session_restoration`,
  `working_state_restoration`); one uses a `typed_request!` macro and another
  hand-writes the identical struct, and validation predicates drift between
  them
- the facade surface is uneven: muse lacks `prepare_working_state_restoration`
  while every headless peer has it; kimi carries a frozen
  `#[allow(dead_code)]` REST/WS corpus (`adapter-kimi/src/local_server/
  protocol.rs:3-8`); `core::EventEnvelope` is now testkit-only vocabulary
  duplicated by `runtime::RuntimeEvent`

## Goals

- [ ] decide one shared meaning for support status and behavior segments
- [ ] standardize claim identity and window numbering; settle the claim-less
      adapter posture
- [ ] consolidate the runtime plan/agreement/request family
- [ ] close the facade-surface gaps or record them as intentional with a
      disposition

## Execution Plan

- [x] Execute card 161 (version-claim semantics decision).
- [x] Execute card 162 (claim identity standardization and claim-less
      posture).
- [x] Execute card 163 (runtime plan-family consolidation).
- [ ] Execute card 164 (facade-surface gap closure).

## Boundaries

- no guaranteed range, capability, or behavior change without separate
  qualification
- no public API change without the pre-1.0 compatibility accounting in
  Contract 036
- no tag, release, registry publication, or live provider work

## Acceptance Criteria

- [x] one documented semantics governs support status across all adapters
- [x] claim ids and windows follow one scheme; claim-less adapters either gain
      claims or carry an explicit disposition
- [x] the runtime plan family shares one core with per-role validation tables
- [x] facade gaps are closed or explicitly disposed in architecture

## Next Planning Checkpoint

The suite planning checkpoint here: decide remaining duplication targets and
whether further consistency work needs new contracts before the generation
returns to its evidence gate.
