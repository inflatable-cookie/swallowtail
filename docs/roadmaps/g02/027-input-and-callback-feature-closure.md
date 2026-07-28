# 027 Input And Callback Feature Closure

Status: active
Owner: Tom
Created: 2026-07-28
Depends on: g02.026
Vision tags: exact capabilities, consumer-mediated interaction, provider breadth
Contract refs: 003, 005-016, 037, 039-040
Planning state: card 088 ready; cards 089-091 planned

## Problem

The route-exact matrix has 74 input and callback `No` cells:

- 20 attachment gaps
- 18 consumer-tool-exchange gaps
- 16 approval-or-question-exchange gaps
- 20 external-search gaps

These columns mix provider inputs, harness callbacks, provider requests, and
execution policy. A shared prompt or generic callback API would erase the
authority differences.

## Goals

- [ ] Revalidate every starting `No` against its exact route and maintained
      version posture.
- [ ] Detect false negatives against realized prepared paths.
- [ ] Separate provider input, consumer tool exchange, approval or question
      callbacks, observed-and-stopped requests, and provider-owned search.
- [ ] Promote only missing shared contracts needed by a selected tranche.
- [ ] Implement a representative tranche across materially different
      transports.
- [ ] Re-audit all 74 starting cells and retain honest absence.

## Non-Goals

- treating provider-owned tools as consumer callback exchange
- treating observed permission requests as approval support
- allowing attachments to imply workspace or filesystem authority
- allowing external search to imply arbitrary network access
- adding implicit provider, route, model, credential, or version fallback
- consumer edits, live authentication, publication, or release mutation

## Execution Plan

### Batch 27.1 — Exact Currentness Audit

- [ ] Execute card 088.
- [ ] Classify all 74 starting cells by exact route, operation shape, and
      authority.
- [ ] Rank conversions by consumer value and architectural information.

### Batch 27.2 — Contract And Corpus Gate

- [ ] Execute card 089 only after the audit selects exact routes.
- [ ] Promote narrow shared distinctions and freeze deterministic corpora.

### Batch 27.3 — Representative Implementation

- [ ] Execute card 090 only for contract-ready routes.
- [ ] Keep callback admission, correlation, authority, and cleanup exact.

### Batch 27.4 — Matrix Closeout

- [ ] Execute card 091.
- [ ] Prove package truth and select the next retained feature family.

## Acceptance Criteria

- [ ] all 74 starting cells are classified exactly once
- [ ] input transport and execution authority remain separate
- [ ] callbacks never execute inside adapters
- [ ] every changed cell maps to a public prepared path
- [ ] version, access, and topology remain exact
- [ ] machine counts and classification drift fail deterministically

## Decision Gates

- Ask the operator if selecting among equally useful route tranches would set
  product priority.
- Stop if maintained evidence cannot distinguish consumer callbacks from
  provider-owned behavior.
- Do not borrow a capability from a sibling route in a composite solution.
- Keep honest `No` when the operation shape cannot expose the feature.

## Next Planning Checkpoint

Card 088 performs the evidence audit. No provider implementation begins before
its currentness and contract gate.
