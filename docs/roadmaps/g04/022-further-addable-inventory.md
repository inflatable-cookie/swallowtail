# 022 Further Addable Inventory

Status: planned
Owner: Tom
Created: 2026-08-20
Depends on: completed g04.021
Vision tags: consumer integration, route readiness, explicit selection
Contract refs: 011, 037, 047, 052, 057
Planning state: cards 062-064 ready
Research: 170

## Problem

Six production routes export addable descriptors. Remaining production
routes stay on the prepared-facade path. Expansion must stay on the
proved hosted API-key, installed, and local-runtime shapes. Hosted
OAuth stays parked. Research 170 named the second-proof tranche only.

## Generation Runway Goal

Close remaining 057/047 seams and expand addable coverage on proved
shapes.

## Goals

- [ ] inventory remaining production routes against the three proved
      057 shapes
- [ ] classify reuse versus descriptor work versus gated
- [ ] confirm the next implementation roadmap after g04.023

## Non-Goals

- writing addable descriptors in this milestone
- hosted URL-open OAuth
- OpenHands production wiring
- advertising `claude-code.headless`, `claude-code.response-only`, or
      `llama-cpp.owned` from sibling addable rows
- marking every remaining production route as addable
- live provider, install, login, or billing work

## Execution Plan

### Batch 22.1 — Surface Inventory

- [ ] Execute card 062.
- [ ] map remaining production routes onto hosted API-key, installed, or
      local-runtime
- [ ] write a research note; do not compile implementation cards yet

### Batch 22.2 — Gap Classification

- [ ] Execute card 063 after card 062.
- [ ] keep OAuth parked
- [ ] keep owned, headless, and response-only rows off sibling addable
      descriptors

### Batch 22.3 — Tranche Confirmation

- [ ] Execute card 064 after card 063.
- [ ] name the first implementation roadmap after g04.023
- [ ] leave later named routes planned behind it

## Acceptance Criteria

- [ ] each inventoried route has a shape, 057 gap, or gated reason
- [ ] hosted OAuth stays parked
- [ ] no adapter crate changes in this milestone
- [ ] the next implementation roadmap is named, not started

## Lane Runway

- previous: g04.021 unmarked overlay rows
- this milestone: further addable inventory
- next: g04.023 047 presentation metadata
- later: named addable implementations from this tranche
- generation continues toward 30-50; do not roll over

## Decision Gates

- Stop if inventory would store raw secrets or create a Swallowtail server.
- Stop if a sibling row is folded into another route's addable descriptor.
- Stop if OpenHands would gain a production route.
- Stop if hosted OAuth is compiled.
