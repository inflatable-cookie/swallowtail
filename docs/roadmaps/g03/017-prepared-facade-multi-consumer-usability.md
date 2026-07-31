# 017 Prepared Facade Multi-Consumer Usability

Status: completed
Owner: Tom
Created: 2026-07-31
Depends on: g03.016
Vision tags: consumer stability, prepared integration, explicit authority
Contract refs: 006, 010, 037
Planning state: cards 044-046 completed

## Problem

Nucleus and Soundcheck prove the prepared Codex facade at application scale,
but both repeat two library-owned details: local monotonic deadline conversion
and the fixed ChatGPT-subscription access profile. Nucleus also still bypasses
existing bound operations in three paths.

## Goal

Remove the two proven library-neutral seams, preserve every consumer-owned
choice, and leave a precise Nucleus adoption handoff without another facade
layer.

## Goals

- [x] expose local-host deadline derivation from an explicit duration
- [x] expose the canonical Codex ChatGPT-subscription access profile
- [x] preserve separate caller-supplied access status and provenance
- [x] keep API-key and enterprise-token access distinct
- [x] prove the public example uses bound operations and canonical access
- [x] record the remaining Nucleus-only bound-operation adoption

## Execution Plan

### Batch 17.1 — Local Deadline Convenience

- [x] Execute card 044.
- [x] implement saturating local monotonic deadline derivation
- [x] prove exact duration conversion and overflow behavior

### Batch 17.2 — Codex Canonical Access Profile

- [x] Execute card 045.
- [x] add one ChatGPT-subscription profile constructor
- [x] retain explicit profile identity, access status, and provenance
- [x] keep other Codex authentication routes outside the helper

### Batch 17.3 — Public Guidance And Acceptance

- [x] Execute card 046.
- [x] update the compile-tested Codex example and guide
- [x] accept the intentional public API additions
- [x] record the Nucleus adoption delta without editing consumers

## Boundaries

- no umbrella crate, provider router, generic setup, or generic prompt method
- no default provider, driver, executable, environment, model, timeout, or
  access status
- no credential discovery, sign-in, account inspection, or entitlement probe
- no ChatGPT-to-API credential or billing substitution
- no consumer repository edit
- no provider prompt, catalogue, session, installation, publication, or broad
  workspace suite

## Acceptance Criteria

- [x] callers choose every duration and access-profile identity explicitly
- [x] local deadline overflow saturates safely
- [x] the Codex helper encodes interactive OAuth, subscription allowance, Codex
  audience, provider support, and no credential reference
- [x] bound operations remain the documented normal path
- [x] the low-level escape hatch remains public
- [x] focused, public-API, docs, and affected-package checks pass
- [x] one sole Next Task pointer remains

## Next Planning Checkpoint

After card 046, return to g03 maintenance. Nucleus may independently adopt the
new helpers and existing bound catalogue/session methods; Soundcheck needs no
bound-operation migration.
