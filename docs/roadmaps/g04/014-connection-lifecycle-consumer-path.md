# 014 Connection Lifecycle Consumer Path

Status: completed
Owner: Tom
Created: 2026-08-20
Depends on: completed g04.013
Vision tags: consumer integration, route readiness, explicit selection
Contract refs: 011, 037, 047, 052, 057
Planning state: cards 039-041 completed
Research: 169

## Problem

The 057 facade and three first-proof addable routes are on `main`. A
consumer still has to reverse-engineer adapter tests to assemble a catalog,
admit an instance, collect an API key or skip credentials, refresh
readiness, and reach the existing prepared facade. Contract 052 has no
connection-lifecycle feature guide, the first-proof route guides still start
at prepare, and there is no compiling 057 example.

## Generation Runway Goal

Prove representative hosted, installed, and local-runtime shapes and publish
a consumer path. This milestone publishes that path. It does not add more
addable descriptors.

## Goals

- [x] publish a Contract 052 feature guide for the realized 057 lifecycle
- [x] amend the Anthropic Messages, Codex app-server, and Ollama attach
      route guides with addable-route and admission sequences
- [x] ship compiling normal-path 057 examples for those three routes
- [x] map the new feature family in the integration guide map without
      claiming the remaining production routes have addable descriptors

## Non-Goals

- hosted interactive OAuth
- addable descriptors for any route beyond the three first-proofs
- marking all 47 production routes complete for 057
- replacing prepared-facade examples as the canonical route examples
- adding a feature-matrix column
- changing Contract 047 `Ready` / `NotReady`
- inventing Codex or Ollama catalogue `provider_id` values
- live provider, install, login, or billing work
- rewriting `public-api-0.3.3`
- OpenHands production wiring

## Execution Plan

### Batch 14.1 — Feature Guide

- [x] Execute card 039.
- [x] write `docs/guides/connection-lifecycle.md`
- [x] index it from `docs/guides/README.md` and point Key Concepts at it
- [x] do not add the guide-map family row yet; `qa:guides` requires
      complete coverage and an exact portable-feature token inventory

### Batch 14.2 — First-Proof Route Amendments

- [x] Execute card 040 after card 039.
- [x] amend the three first-proof route guides with 057 sequences
- [x] keep the remaining production route guides on the prepared-facade
      path; they have no addable descriptor

### Batch 14.3 — Examples And Guide Map

- [x] Execute card 041 after card 040.
- [x] add compiling examples for the three first-proofs
- [x] add portable feature token `connection_lifecycle` and the complete
      feature-family row
- [x] record in architecture that the 052 path exists for those three
      routes only

## Acceptance Criteria

- [x] a consumer can follow the feature guide from catalog assembly to the
      existing prepared facade without reading adapter tests
- [x] Anthropic, Codex app-server, and Ollama attach route guides name
      their 057 addable, admission, credential, refresh, subject, update,
      and overlay facts
- [x] remaining production routes are not documented as addable
- [x] compiling examples exist and pass `effigy check:examples`
- [x] `effigy qa:guides` passes with the new complete feature family
- [x] no secret bytes, live probes, or hosted OAuth
- [x] `public-api-0.3.3` stays immutable

## Lane Runway

- previous: g04.013 local Ollama attach
- this milestone: Contract 052 consumer path for the 057 lifecycle
- next: hosted interactive OAuth remains a remaining gate
- later: Contract 029 currentness stays a recurring maintenance lane

## Decision Gates

- Stop if a guide claims a route has an addable descriptor that the
      adapter does not export.
- Stop if the guide-map family row lands before the examples and checker
      token exist.
- Stop if overlay invents a catalogue provider id.
- Stop if 047 `Ready` / `NotReady` changes.
- Stop if hosted OAuth or OpenHands production wiring starts.
- Stop if a universal router, credential store, or Swallowtail server
      appears in the guide.
