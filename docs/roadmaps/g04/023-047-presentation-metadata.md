# 023 047 Presentation Metadata

Status: completed
Owner: Tom
Created: 2026-08-20
Depends on: completed g04.022
Vision tags: consumer integration, route readiness, explicit selection
Contract refs: 020, 047, 057
Planning state: cards 065-067 completed

## Problem

Contract 057 allows optional 047 presentation metadata later. It must not
change selection readiness. Accent color and other chrome stay
consumer-owned. Overlay hide, ordinal, consumer-default, and favourite
stay overlay markers, not 047 fields. 047 still says display names,
grouping chrome, and product labels remain downstream.

## Generation Runway Goal

Close remaining 057/047 seams and expand addable coverage on proved
shapes.

## Goals

- [x] classify which optional fields may enter 047 versus overlay versus
      consumer chrome
- [x] amend 047/057 only for the named field set
- [x] realize those fields without changing `Ready` / `NotReady`

## Non-Goals

- accent color in 047
- moving overlay hide/favourite into 047
- emails, tokens, or targets on the snapshot
- hosted OAuth
- rewriting `public-api-0.3.3`

## Execution Plan

### Batch 23.1 — Field Inventory

- [x] Execute card 065.
- [x] preferred direction: optional already-stored 057 instance labels
      onto the snapshot; overlay markers stay overlay
- [x] field set is not forked

### Batch 23.2 — Contract Amendment

- [x] Execute card 066 after card 065.
- [x] 047/057/architecture name the fields
- [x] selection readiness formula is unchanged

### Batch 23.3 — Realize Fields

- [x] Execute card 067 after card 066.
- [x] additive API in `public-api-unreleased`
- [x] `public-api-0.3.3` stays immutable

## Acceptance Criteria

- [x] 047 `Ready` / `NotReady` is unchanged
- [x] overlay markers are not 047 snapshot fields
- [x] accent color stays consumer-owned
- [x] emails, tokens, and targets stay absent
- [x] `public-api-0.3.3` stays immutable

## Outcome

The only named optional 047 presentation field is the host-owned
`instance_label` carried from the Contract 057 admitted-instance store
record. Runtime admission and snapshot projection realize it additively.
Overlay markers, accent color, authenticated-subject values, emails, tokens,
and targets remain outside the snapshot. `Ready` / `NotReady` is unchanged.

Validation passed on the worker branch. g04.024 remains named behind this
milestone; its implementation cards are not compiled here.

## Lane Runway

- previous: g04.022 further addable inventory
- this milestone: optional 047 presentation metadata
- later: g04.024 hosted API-key Kimi Platform Chat, then named addable implementations from g04.022
- generation continues toward 30-50; do not roll over

## Decision Gates

- Stop if overlay metadata is added to 047 as a way to change selection.
- Stop if `Ready` / `NotReady` changes.
- Stop if accent color enters 047.
