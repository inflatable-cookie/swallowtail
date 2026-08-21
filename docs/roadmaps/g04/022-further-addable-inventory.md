# 022 Further Addable Inventory

Status: completed
Owner: Tom
Created: 2026-08-20
Depends on: completed g04.021
Vision tags: consumer integration, route readiness, explicit selection
Contract refs: 011, 037, 047, 052, 057
Planning state: cards 062-064 completed
Research: 171

## Problem

Six production routes export addable descriptors. Remaining production
routes stay on the prepared-facade path. Expansion must stay on the
proved hosted API-key, installed, and local-runtime shapes. Hosted
OAuth stays parked. Research 170 named the second-proof tranche only.

## Generation Runway Goal

Close remaining 057/047 seams and expand addable coverage on proved
shapes.

## Goals

- [x] inventory remaining production routes against the three proved
      057 shapes
- [x] classify reuse versus descriptor work versus gated
- [x] confirm the next implementation roadmap after g04.023

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

- [x] Execute card 062.
- [x] map remaining production routes onto hosted API-key, installed, or
      local-runtime
- [x] write a research note; do not compile implementation cards yet

### Batch 22.2 — Gap Classification

- [x] Execute card 063 after card 062.
- [x] keep OAuth parked
- [x] keep owned, headless, and response-only rows off sibling addable
      descriptors

### Batch 22.3 — Tranche Confirmation

- [x] Execute card 064 after card 063.
- [x] name the first implementation roadmap after g04.023
- [x] leave later named routes planned behind it

## Acceptance Criteria

- [x] each inventoried route has a shape, 057 gap, or gated reason
- [x] hosted OAuth stays parked
- [x] no adapter crate changes in this milestone
- [x] the next implementation roadmap is named, not started

## Lane Runway

- previous: g04.021 unmarked overlay rows
- this milestone: further addable inventory — complete in worker PR
- next: g04.023 047 presentation metadata
- later: g04.024 hosted API-key Kimi Platform Chat, then named candidates
- generation continues toward 30-50; do not roll over

## Decision Gates

- Stop if inventory would store raw secrets or create a Swallowtail server.
- Stop if a sibling row is folded into another route's addable descriptor.
- Stop if OpenHands would gain a production route.
- Stop if hosted OAuth is compiled.
