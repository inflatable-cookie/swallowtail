# 012 Provider-Wide Acceptance And Candidate Return

Status: completed
Owner: Tom
Created: 2026-07-25
Depends on: g02.009, g02.010, and g02.011
Vision tags: integration guidance, package proof, release discipline
Contract refs: 011, 029, 036-037
Planning state: cards 034-036 complete; exact publication decision pending

## Problem

Per-adapter facade tests do not prove that the package family presents a
coherent normal path. The held release candidate predates provider-wide facade
coverage.

## Goals

- [x] Publish an exact route-to-facade matrix and compile-tested examples.
- [x] Prove all prepared routes from packaged artifacts.
- [x] Re-run consumer-facing and release compatibility evidence.
- [x] Replace the unpublished candidate only after complete acceptance.

## Execution Plan

### Batch 12.1 — Guidance

- [x] Execute card 034.

### Batch 12.2 — Package Proof

- [x] Execute card 035.

### Batch 12.3 — Candidate Return

- [x] Execute card 036 only after cards 034-035 pass.

## Acceptance Criteria

- [x] all 22 production routes appear once in the route matrix
- [x] every route has a compile-tested normal-path example or focused example
      test
- [x] package artifacts pass deterministic prepared execution
- [x] Nucleus and Soundcheck retain their simplified Codex integration
- [x] no stale candidate claims provider-wide readiness
- [x] publication remains an explicit operator decision

## Decision Gate

Card 036 stops with one exact unpublished replacement candidate. Registry,
owner, upload, tag, push, workflow, and release mutation remain unauthorized.
