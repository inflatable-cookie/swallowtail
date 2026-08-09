# 063 Route-Path Idioms Opt-In

Status: active
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
- [ ] card 195: bounded Nucleus adoption delta on its interactive session
      path
- [ ] card 196: package, guide, matrix, architecture, and acceptance
      evidence

## Goals

- [ ] one host registration and one session-option field replaces per-session
      idioms wiring
- [ ] fail-closed preflight for missing source, plan mismatch, and
      non-advertising routes
- [ ] deterministic fold rule pinned by conformance
- [ ] Codex app-server fixture proof and a bounded Nucleus adoption handoff
- [ ] default behavior unchanged: no option, no idioms work

## Boundaries

- no prompt authorship beyond the opted-in fixed fold rule
- no learned backend or correction-loop proxy in this milestone
- no non-Codex route proof beyond the capability gate
- no version bump, tag, GitHub Release, or registry mutation

## Acceptance Criteria

- [ ] Contract 056 and amendments to 010/012/037/055 govern the surface
- [ ] conformance covers fold determinism, bounds, fail-closed preflight,
      and recorder no-op
- [ ] Codex app-server proof passes deterministic fixtures without live
      provider work
- [ ] Nucleus adopts the surface without importing product policy
- [ ] focused and extracted-package validation pass

## Planning Checkpoint

After card 196, reassess the correction-loop proxy and learned-backend
selection with Nucleus evidence before any second tranche.
