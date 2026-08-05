# 041 Portable Failure Classification

Status: completed
Owner: Tom
Created: 2026-08-05
Depends on: g03.040
Vision tags: diagnostics, consumer integration, provider compatibility
Contract refs: 003-004, 006, 009, 014, 037, 044, 051
Planning state: cards 115-117 complete

## Problem

Swallowtail carries safe diagnostics and distinct lifecycle failure stages,
but consumers must still recognize adapter-owned diagnostic codes to decide
what an ordinary provider or harness error means. Several adapters already
classify the same failures privately and discard that type at the public
boundary.

## Generation Runway Goal

Expose one portable, evidence-bounded failure classification on every safe
diagnostic while preserving exact route codes and lifecycle truth.

## Goals

- [x] promote the portable classification contract and core vocabulary
- [x] expose a route-neutral terminal failure view
- [x] map typed direct-provider failure evidence
- [x] map qualified harness failure evidence without parsing prose
- [x] close provider-wide fallback, corpus, guide, and package truth

## Execution Plan

- [x] card 115: contract, core records, and runtime projection
- [x] card 116: direct-provider and harness mappings
- [x] card 117: cross-route acceptance, guidance, and closeout

## Boundaries

- no provider prose, stderr, output, or raw-body parsing
- no replacement of exact safe diagnostic codes
- no automatic retry or fallback policy
- no callback, refusal, cancellation, timeout, or cleanup flattening
- no stronger class than machine-readable evidence supports

## Acceptance Criteria

- [x] all routes expose an honest portable classification through existing
      safe diagnostics
- [x] equivalent typed provider failures share a portable kind
- [x] unclassified harness failures remain usable as `Unknown`
- [x] terminal source and diagnostic origin remain independently visible
- [x] focused and affected-package validation pass without authenticated work

## Lane Runway

Cards 115-117 are complete. g03 returns to its evidence gate.
