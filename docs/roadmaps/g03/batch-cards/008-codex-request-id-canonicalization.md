# 008 Codex Request-ID Canonicalization

Status: completed
Owner: Tom
Created: 2026-07-31
Milestone: `../003-codex-request-id-canonicalization.md`
Depends on: card 003

## Goal

Repair the consumer-reproduced Codex app-server numeric request-ID failure
without a consumer workaround or widened protocol acceptance.

## Scope

1. Replace arbitrary provider request-ID stringification with one shared
   string-or-signed-integer canonicalizer.
2. Use that canonicalizer at callback admission and
   `serverRequest/resolved` activity completion.
3. Preserve the raw JSON-RPC request ID used by the exact-once callback reply.
4. Add deterministic string, integer, unmatched, invalid-shape, and exact
   activity/correlation coverage.
5. Freeze the integer representation in the app-server activity corpus.
6. Run focused Codex and affected-package validation only.

## Acceptance Criteria

- [x] string and integer request IDs complete their matching request activity
- [x] callback, provider request, activity start, and activity completion keep
  one opaque correlation
- [x] unmatched legal resolutions remain harmless
- [x] non-protocol JSON shapes fail with the safe malformed-notification class
- [x] callback replies remain exact-once and retain the raw numeric ID
- [x] no provider or consumer effect runs
- [x] card 004 returns as the sole ready and next task

## Validation

- `effigy validate:focused swallowtail-adapter-codex`
- `effigy package:verify-affected swallowtail-adapter-codex`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy format:check`
- `git diff --check`
- no broad workspace or authenticated provider suite

## Auto-Continuation

No. Return to card 004 after recording deterministic evidence for Nucleus.

## Evidence

- one private typed request key retains string versus signed-integer wire type
- the opaque provider request reference retains existing string or decimal
  integer presentation
- the callback hub retains the original JSON value for the provider response
- string `"900"` and integer `900` resolve distinct activity ids correctly
- numeric typed-question response emits matching started and completed request
  activity, then reaches normal turn completion
- 137 focused Codex tests passed with warnings denied
- the extracted Codex package compiled
- no live provider, authentication, installation, or consumer effect ran
