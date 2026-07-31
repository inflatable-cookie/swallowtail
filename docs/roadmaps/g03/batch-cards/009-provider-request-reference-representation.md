# 009 Provider Request Reference Representation

Status: completed
Owner: Tom
Created: 2026-07-31
Milestone: `../004-provider-request-reference-representation.md`
Depends on: card 008

## Goal

Complete the portable identity contract exposed by Codex's legal numeric
request ids while the exact consumer evidence remains current.

## Scope

1. Add text and signed-integer representation metadata to
   `ProviderRequestRef`.
2. Keep its value opaque, subject to the existing non-empty validation, and
   redacted by default.
3. Make equality, ordering, and hashing representation-aware.
4. Use the portable reference as Codex's request lookup and activity identity
   source, removing the duplicate private typed key.
5. Add deterministic common-type and Codex end-to-end regression evidence.
6. Run focused core, runtime, and Codex validation plus affected-package proof.

## Acceptance Criteria

- [x] text `"900"` and signed integer `900` remain distinct common references
- [x] consumers can inspect representation without inspecting provider payloads
- [x] all existing text constructors and adapters retain their current meaning
- [x] Codex numeric replies retain the original raw JSON-RPC id
- [x] activity start and completion retain one representation-aware correlation
- [x] no authenticated, provider, or consumer effect runs
- [x] card 004 returns as the sole ready and next task

## Validation

- `effigy validate:focused swallowtail-core swallowtail-runtime swallowtail-adapter-codex`
- `effigy package:verify-affected swallowtail-core swallowtail-runtime swallowtail-adapter-codex`
- `effigy package:api`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy format:check`
- `git diff --check`
- no broad workspace or authenticated provider suite

## Auto-Continuation

No. Return to card 004 after deterministic closeout.

## Evidence

- `ProviderRequestRepresentation` exposes `Text` and `SignedInteger` without
  exposing the opaque value
- common equality, ordering, and hashing include representation
- Codex uses `ProviderRequestRef` as its request activity lookup key
- 293 focused tests passed across core, runtime, and Codex
- all three affected extracted packages compiled
- the intentional core public declaration delta is recorded
- no live provider, authentication, installation, or consumer effect ran
