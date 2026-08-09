# 063 Route-Path Idioms Opt-In

Status: completed
Owner: Tom
Created: 2026-08-09
Depends on: Research 119, Spec 007, Contract 056
Vision tags: learned preferences, idioms, consumer mechanism, route feature
Contract refs: 010, 012, 036, 037, 052-055

## Problem

Consumers must re-wire idiom selection and delivery per session.
Contract 056 fixes the route-path surface; nothing realizes it yet.

## Generation Runway

Adds one opt-in route feature to the consumer-mechanism breadth of g03.062
without changing default route behavior. Candidate lane — not queued against
the g03 evidence gate until the operator selects it. Nucleus is the testbed.

## Execution Plan

- [x] card 193: realize the runtime surface — host ports, session option,
      fold rule, and conformance fixtures
- [x] card 194: realize the prepared binding and capability gate, then the
      Codex app-server proof
- [x] card 195: bounded Nucleus adoption delta on its interactive session
      path
- [x] card 196: package, guide, matrix, architecture, and acceptance
      evidence

## Goals

- [x] one host registration and one session-option field replaces per-session
      idioms wiring
- [x] fail-closed preflight for missing source, plan mismatch, and
      non-advertising routes
- [x] deterministic fold rule pinned by conformance
- [x] Codex app-server fixture proof and a bounded Nucleus adoption handoff
- [x] default behavior unchanged: no option, no idioms work

## Boundaries

- no prompt authorship beyond the opted-in fixed fold rule
- no learned backend or correction-loop proxy in this milestone
- no non-Codex route proof beyond the capability gate
- no version bump, tag, GitHub Release, or registry mutation

## Acceptance Criteria

- [x] Contract 056 and amendments to 010/012/037/055 govern the surface
- [x] conformance covers fold determinism, bounds, fail-closed preflight,
      and recorder no-op
- [x] Codex app-server proof passes deterministic fixtures without live
      provider work
- [x] Nucleus adopts the surface without importing product policy
- [x] focused and extracted-package validation pass

## Planning Checkpoint

The lane is complete. Reassess the correction-loop proxy and
learned-backend selection with Nucleus evidence before any second tranche.
Return to the operator for the next lane.
