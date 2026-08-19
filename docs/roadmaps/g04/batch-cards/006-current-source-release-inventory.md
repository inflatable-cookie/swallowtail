# 006 Current Source Release Inventory

Status: completed
Owner: Tom
Created: 2026-08-19
Milestone: `../003-current-source-tag-before-readiness.md`

## Goal

Freeze the unreleased package, route, and public-API delta from `v0.3.2` and
classify `0.3.3` versus `0.4.0` under Contract 036.

## Scope

1. Inventory current-source packages and production routes.
2. Compare semantic API baselines for existing packages.
3. Keep OpenHands as a package without a production route.
4. Record the selected coordinated version before changelog mutation.

## Out Of Scope

- readiness facade types
- tag or remote mutation
- publication
- changing provider claims

## Acceptance Criteria

- [x] package and route counts match architecture and Contract 036
- [x] patch versus minor is explicit
- [x] no facade implementation is in the candidate set

## Evidence

- current source: 40 packages, 47 production routes, Rust `1.95.0`
- immutable `v0.3.2`: 30 packages, 36 routes
- ten additive packages; eleven additive production routes; OpenHands has no
  production route
- existing-package APIs: 27 identical to `v0.3.2`; Claude Agent, Cursor, and
  Grok are additive-only; zero removals
- selected coordinated version: patch `0.3.3`, not `0.4.0`
- `[Unreleased]` still omits DeepSeek Harness and ZCode; card 007 must add
  those entries before promotion
- no Spec 011 facade types in the candidate set

## Validation

- `effigy package:metadata` — 40 crates at `0.3.2`, Rust `1.95`
- `effigy package:api` — 30 immutable `v0.3.2` packages plus 13 reviewed
  unreleased API surfaces
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy qa:routes` — 47 production routes
- `git diff --check`

## Auto-Continuation

Yes, into card 007 after the version class is recorded.
