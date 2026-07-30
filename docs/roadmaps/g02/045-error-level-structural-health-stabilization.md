# 045 Error-Level Structural Health Stabilization

Status: completed
Owner: Tom
Created: 2026-07-30
Depends on: g02.044
Vision tags: maintainability, package isolation, validation discipline
Contract refs: 001, 036-037, 044
Planning state: cards 150-155 completed

## Problem

Effigy health passes, but doctor remains red on 33 error-level oversized-file
findings. Five are critical. The debt concentrates provider behavior, fixture
logic, and matrix validation in files that are hard to review and expensive to
change safely.

This is stabilization work. Decomposition must preserve public declarations,
provider behavior, fixtures, version claims, lifecycle truth, and validation
coverage.

## Generation Runway Goal

Return doctor to warning-only structural posture without adding provider
behavior or widening the public API. Leave warning reduction and validation-
latency work as explicit later decisions.

## Goals

- [x] freeze the exact error-level inventory and private module seams
- [x] remove all five critical findings
- [x] remove the remaining Codex and OpenCode error findings
- [x] remove shared-runtime and stateful-harness adapter error findings
- [x] remove remaining direct-adapter error findings
- [x] close with zero error-level structural findings and passing health

## Non-Goals

- provider, protocol, model, version, access, lifecycle, or capability changes
- public API redesign or compatibility shims
- warning-level file decomposition
- full test-runner or CI redesign
- consumer repository edits
- publication, candidate replacement, or external effects

## Execution Plan

### Batch 45.1 — Inventory And Seam Freeze

- [x] Execute card 150.
- [x] freeze 33 exact error findings by crate and source kind
- [x] select behavior-preserving private module seams
- [x] bind each implementation card to focused validation

### Batch 45.2 — Critical Files

- [x] Execute card 151.
- [x] split the five critical source, test, and script files
- [x] retain exact fixtures, test names, and matrix output

### Batch 45.3 — Codex And OpenCode Concentration

- [x] Execute card 152.
- [x] split the remaining five Codex and seven OpenCode error files
- [x] retain exact range, activity, callback, lifecycle, and prepared behavior

### Batch 45.4 — Runtime And Stateful Harnesses

- [x] Execute card 153.
- [x] split runtime, Claude Agent, Gemini, and Kimi error files
- [x] retain public declarations and cross-transport semantics

### Batch 45.5 — Remaining Adapters

- [x] Execute card 154.
- [x] split Pi, Alibaba, DeepSeek, and xAI error files
- [x] split the residual provider-route matrix high fragment
- [x] retain route-local inference and callback behavior

### Batch 45.6 — Acceptance

- [x] Execute card 155.
- [x] prove zero error-level structural findings
- [x] run focused packages, workspace check, public-API, route, docs, and
  package gates
- [x] record warning posture and select the next stabilization checkpoint

## Acceptance Criteria

- [x] doctor health passes
- [x] doctor reports zero error-level oversized-file findings
- [x] no warning-level finding is promoted to error
- [x] public declaration hashes remain unchanged unless separately justified
- [x] provider route, lifecycle, feature, and activity matrices remain exact
- [x] focused package tests and warnings-denied clippy pass per batch
- [x] no live provider effect, consumer edit, or publication occurs
- [x] one clear next task remains

## Decision Gates

- Stop if a split requires public API or behavior changes.
- Stop if a provider-specific invariant cannot stay beside its owning code.
- Stop if concurrent functional work overlaps a target file without a stable
  focused baseline.
- Do not use compatibility aliases or duplicate old and new implementations.
- Do not run a broad test suite when focused package evidence covers the batch.

## Next Planning Checkpoint

Roadmap g02.046 owns validation-latency work. Warning-only structural
reduction remains deferred.
