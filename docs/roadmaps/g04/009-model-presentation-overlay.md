# 009 Model Presentation Overlay

Status: completed
Owner: Tom
Created: 2026-08-20
Depends on: completed g04.008
Vision tags: consumer integration, route readiness, explicit selection
Contract refs: 020, 047, 057
Planning state: cards 025-026 completed

## Problem

Overlay marker records already persist in the store. Consumers still cannot
apply hide, ordinal, consumer-default, and favourite onto a bound catalogue
without flattening gateway rows or changing 047 readiness.

## Generation Runway Goal

Realize the model-presentation overlay without flattening catalogues.

## Goals

- [x] apply overlay markers keyed to configured-instance, provider, and
      model ids
- [x] keep provider catalogue defaults distinct from consumer-default
- [x] refuse invented models, cross-instance copies, and `NotReady`
      selectable

## Non-Goals

- putting overlay metadata into 047 in this milestone
- accent color and other chrome
- gateway flattening
- first-proof adapter catalogues
- changing `Ready` / `NotReady`

## Execution Plan

### Batch 9.1 — Apply Overlay

- [x] Execute card 025 after g04.008.
- [x] project hide, ordinal, consumer-default, and favourite onto one bound
      catalogue result

### Batch 9.2 — Overlay Refusals

- [x] Execute card 026 after card 025.
- [x] refuse invent, copy-across-instance, and `NotReady` selectable

## Acceptance Criteria

- [x] overlay keys are exact instance, provider, and model ids
- [x] provider default stays distinct from consumer-default
- [x] mixed gateway rows remain consumer assembly of several catalogues
- [x] 047 selection readiness is unchanged

## Lane Runway

- previous: g04.008 refresh and subject
- this milestone: overlay projection
- later: first-proof routes

## Decision Gates

- Stop if overlay changes `Ready` / `NotReady`.
- Stop if overlay invents a model or copies across instances.
