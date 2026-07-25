# 006 Consumer Runtime Proof

Status: completed
Owner: Tom
Created: 2026-07-24
Depends on: g02.004 and g02.005
Vision tags: release discipline, consumer upgrade support, runtime evidence
Contract refs: 002, 011, 029, 036-037
Planning state: card 015 complete; candidate replacement moved to g02.012

## Problem

The first unpublished `0.1.0` candidate proves package reproducibility and
consumer compilation but not deterministic runtime preparation. Its handoffs
describe low-level integration that the prepared facade will replace.

The candidate must be regenerated from the simplified public path.

## Goals

- [x] Add deterministic cross-consumer runtime preparation gates.
- [x] Prove Nucleus and Soundcheck against packaged artifacts, not sibling
      source assumptions.
- [x] Preserve candidate evidence while provider-wide facade work proceeds.
- [x] Move replacement and publication return to g02.012.

## Non-Goals

- [ ] Do not publish, tag, push, create a release, or change registry owners.
- [ ] Do not call compile-only checks runtime acceptance.
- [ ] Do not require live provider credentials for deterministic release QA.
- [ ] Do not widen any provider-interface guarantee.
- [ ] Do not create a compatibility shim for the superseded pre-release API.

## Execution Plan

### Batch 6.1 — Cross-Consumer Runtime Evidence

- [x] Execute card 015 after both migrations close.
- [x] Run deterministic catalogue, structured-run, read-only session, and
      bounded-workspace preparation against packaged Swallowtail artifacts.
- [x] Prove safe failures, exact plans, and joined cleanup.

### Batch 6.2 — Replacement Candidate And Handoffs

- [x] Supersede card 016 with card 036 after Contract 037 expands to all
      production routes.
- [x] Retain the unpublished candidate as historical evidence.
- [x] Keep all external release mutation unauthorized.

## Acceptance Criteria

- [x] consumer evidence executes deterministic Codex runtime preparation
- [x] exact package artifacts back both consumer proofs
- [x] current facade examples and consumer integrations agree
- [x] API changes are classified against the unreleased baseline
- [x] all existing provider version guarantees remain exact
- [x] live probes remain supplemental and gated
- [x] candidate replacement is held behind provider-wide facade acceptance

## Risks And Mitigations

- Risk: deterministic consumer proof depends on local paths. Mitigation: use
  extracted candidate packages and controlled fake host/provider fixtures.
- Risk: replacing evidence leaves stale checksums. Mitigation: rebuild every
  candidate artifact and evidence digest from one clean commit.
- Risk: facade work broadens the release set. Mitigation: retain the contracted
  23-package graph unless a separately promoted contract changes it.

## Evidence Requirements

- packaged-artifact Nucleus and Soundcheck runtime smokes
- cross-consumer preparation failure matrix
- full package, public API, docs, MSRV, content, checksum, and repository QA
- updated release notes and consumer handoffs
- exact clean source commit and candidate hashes
- explicit statement that no external release mutation occurred

## Decision Gate

Card 015 passes against a transient package candidate with no live credentials
or provider calls. Card 016 is superseded. Roadmap g02.007 is active;
publication remains unauthorized.
