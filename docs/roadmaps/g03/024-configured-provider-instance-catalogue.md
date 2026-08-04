# 024 Configured Provider Instance Catalogue

Status: completed
Owner: Tom
Created: 2026-08-04
Depends on: g03.023
Vision tags: provider-wide truth, consumer integration, explicit selection
Contract refs: 002, 005-006, 008, 014, 020, 029, 037, 047
Planning state: cards 064-065 completed

## Problem

Nucleus g05.073 cannot offer truthful provider selection from Swallowtail's
separate configured-instance, prepared-route, access, and model-catalogue
records. Reassembling those facts in Nucleus would duplicate provider identity
and readiness policy.

## Goals

- [x] define one bounded portable configured-instance projection
- [x] retain exact facade, route, access, provider, and model evidence
- [x] keep unavailable instances observable and non-selectable
- [x] reject cross-instance or cross-route catalogue assembly
- [x] publish a concise consumer assembly path

## Execution Plan

### Batch 24.1 — Contract And Runtime Admission

- [x] Execute card 064.
- [x] promote Contract 047 and architecture placement
- [x] add immutable runtime catalogue records and strict admission
- [x] cover ready, unavailable, mismatched, duplicate, bounded, and redacted
  cases deterministically

### Batch 24.2 — Acceptance And Nucleus Handoff

- [x] Execute card 065 after the runtime surface settles.
- [x] run focused and affected-package validation
- [x] publish the exact public assembly path for Nucleus g05.073
- [x] return the sole Next Task to the g03 evidence gate

## Boundaries

- no Nucleus or other consumer edit
- no provider router, default, fallback, preference, or session policy
- no credential or target authority in the projection
- no provider probe, model-catalogue execution, or live provider work
- no provider-specific type in core or runtime
- no generation rollover

## Acceptance Criteria

- [x] a consumer can assemble several exact configured instances into one
  immutable catalogue
- [x] model entries are bound to the exact prepared catalogue route that
  produced them
- [x] unavailable, unauthenticated, unsupported, failed, and empty instances
  remain visible but non-selectable
- [x] public types contain no credential or target reference
- [x] focused runtime validation and affected-package verification pass
- [x] Nucleus receives a compile-oriented integration handoff

## Next Planning Checkpoint

After card 065, Nucleus may resume g05.073. Swallowtail returns to the g03
evidence gate unless adoption exposes another portable defect.
