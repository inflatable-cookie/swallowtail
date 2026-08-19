# 003 Current Source Tag Before Readiness Implementation

Status: planned
Owner: Tom
Created: 2026-08-19
Depends on: g04.002
Vision tags: source release, compatibility maintenance
Contract refs: 001, 029, 036-037, 052
Planning state: cards 006-007 completed (`0.3.3` local candidate); cards 008-009 planned behind operator gates

## Problem

Current source is 40 packages and 47 production routes. Immutable `v0.3.2` is
30 packages and 36 routes. Ten reviewed adapters, currentness bound raises, and
related public-tree work are unreleased. The route-readiness facade should not
start from that untagged delta.

Contract 036 currently classifies additive packages, additive routes, and
newly qualified interface versions as patch-compatible when no break is
selected. The planning hypothesis is therefore `v0.3.3`, not `v0.4.0`, unless
the release inventory finds a breaking API, MSRV, range, or guaranteed-behavior
change.

## Generation Runway Goal

Pin the post-g03 source tree as an immutable Git tag before any readiness
facade implementation. This is packaging of current work, not the facade.

## Goals

- [x] inventory unreleased packages, routes, and public API against `v0.3.2`
- [x] classify patch `0.3.3` or minor `0.4.0` under Contract 036
- [x] prepare one exact source-tag candidate
- [ ] ship the annotated tag only after separate operator authorization

## Non-Goals

- route-readiness facade records, store, or sign-in loop
- crates.io publication, GitHub Release object, binary, or installer
- OpenHands production wiring
- Aider, Kiro headless, Gemini requalification, or Pi continuity
- consumer repository edits

## Execution Plan

### Batch 3.1 — Release Inventory And Contract

- [x] Execute card 006.
- [x] freeze package, route, and API inventories
- [x] confirm patch versus minor
- [x] keep OpenHands as a package without a production route

### Batch 3.2 — Local Candidate

- [x] Execute card 007 after card 006.
- [x] changelog, notes, coordinated version, and credential-free gates

### Batch 3.3 — Canonical CI

- [ ] Execute card 008 after operator acceptance of the local candidate.
- [ ] commit, push, and require canonical CI at the exact SHA

### Batch 3.4 — Annotated Tag

- [ ] Execute card 009 after separate exact authorization.
- [ ] create and push one annotated immutable tag
- [ ] only then may later g04 implementation cards become ready

## Acceptance Criteria

- [x] all current-source packages share the selected coordinated version
- [x] historical `v0.3.2` inventories remain immutable
- [x] OpenHands remains fail-closed in release notes
- [x] no readiness-facade types ship in this tag
- [ ] annotated tag resolves to the green commit
- [x] later implementation roadmaps stay planned until this tag exists

## Decision Gates

- Stop on API incompatibility, MSRV change, range shrink, or missing
  authorization.
- Stop if facade implementation has already started.
- Reclassify to `0.4.0` only with recorded Contract 036 break evidence.
