# 026 Generation-Control Feature Closure

Status: active
Owner: Tom
Created: 2026-07-28
Depends on: g02.025
Vision tags: exact capabilities, provider breadth, safe generation control
Contract refs: 003, 005-016, 020, 037, 039-040
Planning state: cards 084-085 complete; card 086 ready; card 087 planned

## Problem

The current matrix has 48 generation-control `No` cells:

- 14 output-token-limit gaps
- 14 reasoning-selection gaps
- 20 structured-output gaps

Some are likely unimplemented provider features. Others may be unsupported,
route-specific, or materially different controls. Matrix truth cannot infer
one from another.

## Goals

- [ ] Revalidate every generation-control `No` against current qualified
      provider or maintained-project evidence.
- [ ] Detect false negatives against realized prepared paths.
- [ ] Separate hard provider limits, effort or mode selection, native schema
      enforcement, prompt-only conventions, and consumer post-validation.
- [ ] Promote only the smallest missing shared contracts.
- [ ] Implement a representative high-value tranche through existing route
      identities.
- [ ] Re-audit all 48 cells and retain honest absence.

## Non-Goals

- treating a prompt instruction as native structured output
- treating client truncation as a provider output-token limit
- converting reasoning visibility into reasoning selection
- adding implicit model, route, credential, endpoint, or version fallback
- live authentication, consumer edits, publication, or release mutation

## Execution Plan

### Batch 26.1 — Exact Currentness Audit

- [x] Execute card 084.
- [x] Produce one row-by-row evidence classification for all 48 cells.
- [x] Rank conversion candidates by consumer value and architectural
      information.

### Batch 26.2 — Contract Promotion

- [x] Execute card 085 for the selected seven-cell tranche.
- [x] Promote exact provider-neutral distinctions needed by selected routes.
- [x] Freeze safe fixtures before production code.

### Batch 26.3 — Representative Implementation

- [ ] Execute card 086 only for contract-ready routes.
- [ ] Keep each provider's unsupported and negotiated behavior visible.

### Batch 26.4 — Matrix Closeout

- [ ] Execute card 087.
- [ ] Prove package truth and select the next feature family from the retained
      matrix runway.

## Acceptance Criteria

- [ ] all 48 starting cells have current evidence classifications
- [ ] native enforcement and consumer conventions remain distinct
- [ ] every changed cell maps to a public prepared path
- [ ] version and access authority remain exact
- [ ] machine counts and classification drift fail deterministically
- [ ] one next family remains explicit

## Decision Gates

- Ask the operator if selecting among equally valid provider routes would set
  product priority.
- Stop if a provider's public surface does not distinguish enforcement from a
  prompt convention.
- Do not borrow a control from another route in the same provider family.

## Next Planning Checkpoint

Card 086 implements the seven selected OpenAI, Ollama, and OpenCode cells. Do
not roll to g03.
