# 018 OpenCode Session Deletion Proof

Status: completed
Owner: Tom
Created: 2026-07-26
Depends on: g02.015
Vision tags: OpenCode, attached HTTP, provider data deletion
Contract refs: 011, 014, 017, 029, 037-038
Planning state: cards 055-057 completed

## Problem

OpenCode exposes session deletion through its attached HTTP service.
Swallowtail's qualified six-route subset excludes it, and handle close only
releases the attached runtime work.

## Goals

- [x] Add deletion to the exact recursively frozen OpenCode range corpus.
- [x] Add bound low-level and prepared deletion without taking server
      lifecycle authority.
- [x] Preserve endpoint, server-version, delegated-access, effect-boundary,
      and provider-data-deletion truth.
- [x] Keep local archive and provider restore unsupported.

## Execution Plan

### Batch 18.1 — Range Evidence

- [x] Execute card 055 after g02.017 closes.

### Batch 18.2 — Attached Driver

- [x] Execute card 056 after the exact selected schema is qualified.

### Batch 18.3 — Conformance

- [x] Execute card 057 after the production path is complete.

## Acceptance Criteria

- [x] all qualified OpenCode segments include exact deletion schema evidence
- [x] attached service ownership remains external
- [x] one bound inactive session is the only deletion target
- [x] no archive, restore, resume, retry, or transport fallback is fabricated
- [x] unverified-newer and endpoint drift remain visible
- [x] both host topologies pass deterministic failure and uncertainty cases

## Planning Gap

Resolved by Research 039 and card 055. All 45 exact tagged releases retain a
complete delete closure. Two schema revisions, eight exact published
segments, and two runtime evidence revisions preserve missing, descendant,
active-target, authentication, error, exclusion, and unverified-newer truth.
Card 056 may implement the production path.

Card 056 completes that path through one shared low-level/prepared operation.
Card 057 completes full range, topology, failure, cleanup, and regression
closeout. Roadmap 018 is complete.
