# 015 Second-Proof Addable Inventory

Status: completed
Owner: Tom
Created: 2026-08-20
Depends on: completed g04.014
Vision tags: consumer integration, route readiness, explicit selection
Contract refs: 011, 014, 029, 037, 047, 052, 057
Planning state: cards 042-044 completed
Research: 170

## Problem

The first-proof shapes and Contract 052 consumer path are on `main`. Most
production routes still cannot be listed or admitted through 057. Expanding
coverage has to stay on the proved hosted API-key, installed, and
local-runtime shapes. Hosted interactive OAuth stays gated.

## Generation Runway Goal

Expand addable-route coverage on the proved hosted, installed, and
local-runtime shapes. This milestone inventories three named candidates and
names the first implementation tranche.

## Goals

- [x] inventory DeepSeek continuation, Claude Agent ACP, and llama.cpp
      attached against Contract 057
- [x] keep hosted interactive OAuth an explicit remaining gate
- [x] confirm g04.016 hosted API-key DeepSeek continuation as the next
      implementation roadmap unless inventory contradicts it
- [x] keep adapter wiring planned until this inventory closes

## Non-Goals

- writing addable descriptors or changing prepared facades
- hosted URL-open OAuth
- addable descriptors for `claude-code.headless`,
      `claude-code.response-only`, or `llama-cpp.owned`
- marking remaining production routes as addable
- live provider, install, login, or billing work
- OpenHands production wiring

## Execution Plan

### Batch 15.1 — Surface Inventory

- [x] Execute card 042.
- [x] map each named route onto 057
- [x] write Research 170

### Batch 15.2 — Gap Classification

- [x] Execute card 043 after card 042.
- [x] classify reuse versus descriptor work versus gated
- [x] keep Claude Agent subscription-only for the installed row
- [x] keep llama.cpp attached separate from owned

### Batch 15.3 — Tranche Confirmation

- [x] Execute card 044 after card 043.
- [x] confirm g04.016 DeepSeek continuation as the next implementation
      roadmap
- [x] leave Claude Agent ACP and llama.cpp attached planned behind it

## Acceptance Criteria

- [x] each named candidate has an existing-surface map and a 057 gap list
- [x] hosted OAuth stays gated
- [x] no adapter crate changes in this milestone
- [x] g04.016 compiled after this inventory, not before

## Lane Runway

- previous: g04.014 Contract 052 consumer path
- this milestone: second-proof inventory and tranche selection
- next: g04.016 hosted API-key DeepSeek continuation
- later: Claude Agent ACP, llama.cpp attached, hosted OAuth gate

## Decision Gates

- Stop if inventory would store raw secrets or create a Swallowtail server.
- Stop if Claude Agent is reclassified as hosted OAuth.
- Stop if llama.cpp owned is folded into attached.
- Stop if OpenHands would gain a production route.
