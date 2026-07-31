# 003 Codex Request-ID Canonicalization

Status: completed
Owner: Tom
Created: 2026-07-31
Depends on: g03.001
Vision tags: maintained compatibility, consumer stability, exact correlation
Contract refs: 014, 041, 044
Planning state: card 008 completed; g03.002 resumed at card 004

## Problem

Nucleus reproduced a legal Codex app-server `0.146.0` request identifier that
Swallowtail admitted at typed-callback start but rejected at
`serverRequest/resolved`. Codex `RequestId` permits string or signed integer
values. Admission canonicalized both into an opaque provider request
reference; activity resolution required text.

The mismatch fails an otherwise valid resumed turn and hides exact provider
correlation behind `swallowtail.codex.app_server.malformed_notification`.

## Contract Posture

Contracts 014, 041, and 044 already require opaque provider request identity,
exact one-shot callback correlation, and matching request activity lifecycle.
No contract delta is needed. The Codex adapter implementation is narrower than
the qualified wire and must be repaired.

## Goal

Use one strict Codex request-ID canonicalizer at callback admission and
activity resolution. Accept protocol-defined strings and signed integers only.
Preserve the raw JSON-RPC identifier inside the callback exchange so the
provider response repeats its original representation.

## Execution

- [x] Execute card 008.
- [x] centralize strict string-or-signed-integer normalization
- [x] complete request activity through the same opaque correlation
- [x] freeze numeric resolution in the app-server activity corpus
- [x] prove invalid shapes fail and unmatched valid resolution remains harmless
- [x] prove one numeric typed-question response resumes to normal completion
- [x] run focused Codex and extracted-package validation without live effects
- [x] restore g03 card 004 as the sole next task

## Boundaries

- no consumer workaround or consumer repository edit
- no prompt or provider-prose parsing
- no arbitrary JSON stringification
- no duplicate callback or activity vocabulary
- no weakening of exact-once response handling
- no compatibility change outside Codex app-server
- no installation, authentication, model call, or provider effect

## Acceptance

- [x] string start and resolution share one activity and correlation
- [x] integer start and resolution share one activity and correlation
- [x] raw integer request identity is used in the JSON-RPC response
- [x] null, object, array, boolean, floating-point, and out-of-range integer
  shapes fail safely
- [x] unmatched legal resolution emits no activity and does not fail
- [x] focused package and extracted-package validation pass
- [x] Nucleus can update its local path dependency and rerun card 026

## Next

After closeout, resume roadmap g03.002 at card 004. This compatibility
interruption does not alter the Claude/Gemini tranche or generation boundary.
