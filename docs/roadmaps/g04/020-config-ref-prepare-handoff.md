# 020 Config-Ref Prepare Handoff

Status: planned
Owner: Tom
Created: 2026-08-20
Depends on: completed g04.019
Vision tags: consumer integration, route readiness, explicit selection
Contract refs: 014, 037, 047, 057
Planning state: cards 056-058 ready

## Problem

Admission stores opaque `ConfigFieldRef` values. Every realized addable
prepare still takes a separate host `InstanceTargetRef`, `ExecutableRef`,
or `EnvironmentRef`. Card 018 left resolving those references out of
scope. A consumer cannot hand an admitted instance to `prepare_*` without
keeping a second copy of the target.

## Generation Runway Goal

Close remaining 057/047 seams and expand addable coverage on proved
shapes.

## Goals

- [ ] inventory prepare inputs versus stored config and credential refs
      on the six addable routes
- [ ] realize a portable handoff that keeps values host-private
- [ ] prove the six addable `prepare_*` entries consume that handoff

## Non-Goals

- putting paths, URLs, or env bodies into portable records or 047
- hosted OAuth
- overlay keying changes
- 047 presentation metadata
- new addable routes
- rewriting `public-api-0.3.3`

## Execution Plan

### Batch 20.1 — Inventory

- [ ] Execute card 056.
- [ ] map each of the six addable prepare entries onto stored refs
- [ ] name whether 057 or 037 needs a seam amendment

### Batch 20.2 — Portable Handoff

- [ ] Execute card 057 after card 056.
- [ ] host still resolves refs; Swallowtail never stores values
- [ ] 037 remains after admission and still binds an exact target

### Batch 20.3 — Six-Route Proof

- [ ] Execute card 058 after card 057.
- [ ] Anthropic, DeepSeek, Codex, Claude Agent, Ollama, and llama.cpp
      attached consume the handoff
- [ ] guides stop saying stored refs do not feed prepare

## Acceptance Criteria

- [ ] admitted config refs are the prepare inputs for those six routes
- [ ] public records still carry no paths, URLs, or env bodies
- [ ] 047 still has no targets
- [ ] `public-api-0.3.3` stays immutable

## Lane Runway

- previous: g04.019 llama.cpp attached
- this milestone: config-ref prepare handoff
- next: g04.021 unmarked overlay rows
- later: further addable inventory, 047 presentation metadata
- generation continues toward 30-50; do not roll over

## Decision Gates

- Stop if values leak into diagnostics, the store, or 047.
- Stop if admission starts preparing.
- Stop if hosted OAuth or OpenHands production wiring starts.
