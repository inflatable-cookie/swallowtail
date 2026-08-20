# 006 Addable Catalog, Admission, And Config Fields

Status: completed
Owner: Tom
Created: 2026-08-19
Depends on: g04.005
Vision tags: consumer integration, route readiness, explicit selection
Contract refs: 006, 008, 014, 037, 047, 057
Planning state: cards 016-018 completed

## Problem

The kernel can store records. Consumers still cannot assemble addable routes
or admit a configured instance. Discovery still probes one already selected
driver. Production adapters still have no addable-route descriptors.

## Generation Runway Goal

Realize addable-route catalog, admission, and config field descriptors.

## Goals

- [x] assemble an addable-route catalog from adapter-local descriptors
- [x] admit configured instances, including several of one family
- [x] attach config-field descriptors as opaque host-owned references
- [x] keep discovery candidates distinct from addable rows and instances

## Non-Goals

- production Anthropic, Codex, or Ollama descriptors (later first-proof)
- sign-in loop or new host ports
- readiness refresh, subject reveal, or overlay projection
- a registry crate of every production route
- preparing the instance (Contract 037 stays after admission)

## Execution Plan

### Batch 6.1 — Catalog Assembly

- [x] Execute card 016 after g04.005.
- [x] consumer-assembled catalog from adapter-local descriptors
- [x] available, unavailable, and unsupported observations
- [x] testkit fixtures only; no production adapter crate

### Batch 6.2 — Admission

- [x] Execute card 017 after card 016.
- [x] write an admitted configured instance through the store
- [x] several instances of one family remain distinct ids
- [x] admission does not prepare or change 047 readiness

### Batch 6.3 — Config Fields

- [x] Execute card 018 after card 017.
- [x] config-field descriptors on admitted instances
- [x] values stay host-private

## Acceptance Criteria

- [x] catalog assembly needs no umbrella registry
- [x] topology grouping is not `ExecutionLayer`
- [x] a discovered candidate cannot be admitted as if it were an addable row
- [x] admission does not call Contract 037 preparation
- [x] public records still carry no paths, URLs, or secret bytes

## Lane Runway

- previous: g04.005 kernel
- this milestone: catalog, admission, config fields
- next: g04.007 sign-in loop and host ports
- later compile: refresh, subject, overlay, first-proof routes

## Decision Gates

- Stop if a production adapter is wired before the first-proof milestone.
- Stop if admission prepares, selects a model, or writes 047.
- Stop if config fields leak host paths into portable records.
