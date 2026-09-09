# 152 Claude SDK Structured Provider Failure

Status: complete; PR 297 merged at `24f88fb8a1328aa0e85b9c91989962ba32b9c590` (reviewed head `5d85cc93fd08cedbf8089ad171e35c88d3691ad2`)
Owner: Claude Agent SDK adapter worker
Created: 2026-09-09
Milestone: `../036-v0-4-4-release-readiness.md`
Depends on: Card 151 complete; Research 300; Contract 019 structured-failure rule

## Goal

Preserve the Claude Agent SDK's safe structured provider-failure facts so a
billing-specific response is visible when the upstream result actually proves
one, without exposing or parsing provider prose.

## Scope

1. Extend the private sidecar `turn_ended` projection with optional validated
   `apiErrorStatus`, optional bounded `terminalReason`, and the latest active-
   turn `rateLimitStatus` from the exact three-value enum. Reset rate state at
   each turn boundary; idle notifications cannot contaminate a later turn.
2. Reject malformed present values. Never forward `result`, `error`, `errors`,
   quota payloads, account/session identifiers, request identifiers, headers,
   paths, credentials, or provider prose.
3. Carry the three facts through strict wire decoding into the failed terminal
   diagnostic. Use stable route codes and portable failure classifications only
   where the documented status is unambiguous. At minimum: `402` is
   billing/entitlement unavailable; `400` is invalid-request-or-spend-limit;
   `429` is rate-or-spend-limit; absent or unknown status remains generic.
4. Preserve the one-attempt rule. No status or rate event authorizes automatic
   retry, fallback, replay, or account mutation.
5. Add provider-free sidecar, wire, event, classification, malformed-value,
   redaction, turn-reset, and unknown-status fixtures. Preserve all current
   success and generic-failure behavior when the new fields are absent.
6. Update the Claude SDK guide, changelog, exact sidecar/source identity
   evidence, and public API baseline only if the accepted implementation
   changes those surfaces.

Owned mutable paths: `crates/swallowtail-adapter-claude-agent/src/sdk/**`,
`crates/swallowtail-adapter-claude-agent/sidecar/**`, that crate's exact focused
tests and fixtures, directly affected Claude SDK guide/identity/changelog/API
surfaces, this card result, and one factual log. Queue closeout owns shared
indexes.

Forbidden: provider execution; live probe; Desktop edits; raw error-text
forwarding or matching; credentials/account data; retry/fallback changes;
registered-tool qualification; feature-matrix availability; candidate, tag,
release, or unrelated dependency changes.

## Acceptance Criteria

- [x] numeric API status, bounded terminal reason, and fixed rate state survive end to end
- [x] `402` produces a billing/entitlement-specific safe diagnostic and classification
- [x] `400` and `429` remain explicitly ambiguous and never claim quota exhaustion
- [x] malformed, absent, unknown, idle, and cross-turn cases fail closed or remain generic
- [x] raw provider text and sensitive fields never cross the sidecar wire
- [x] no result authorizes automatic retry or fallback
- [x] existing success/generic failure and MCP behavior remain intact

## Validation

- `effigy validate:focused swallowtail-adapter-claude-agent`
- `effigy package:verify-affected swallowtail-adapter-claude-agent`
- focused sidecar-asset, wire, driver, redaction, and failure-classification tests
- `effigy qa:docs`
- `git diff --check`
- exact-head independent review

## Review Oracle

Smallest counterexample: the numeric status is still discarded; `400` or `429`
is mislabeled billing/quota; raw error prose crosses the wire; rate state leaks
between turns; or a new classification creates retry behavior.

## Stop Conditions

- exact SDK `0.3.259` evidence contradicts the planned field types;
- a safe classification requires provider-text parsing; or
- the repair requires a live provider observation.

## Auto-Continuation

No implementation worker continuation. After accepted merge, Chatterbox will
promote the already-authorized Desktop one-shot diagnostic against its exact
source SHA if the balance remains zero.

## Result

Implemented provider-free. The sidecar projects validated `apiErrorStatus`
(`100..=599`), `terminalReason` (`/^[A-Za-z0-9_.-]{1,96}$/`), and the latest
active-turn `rateLimitStatus` (`allowed`/`allowed_warning`/`rejected`) on
`turn_ended`; malformed present values fail closed as `unknown_message`;
rate state resets at each turn boundary and idle notices never attach. The
strict wire decodes the three facts (absent/null stays generic) into the
failed terminal diagnostic: `402` →
`swallowtail.claude-agent.sdk.provider_billing_unavailable` with
`(Provider, EntitlementUnavailable, ConfigurationChangeRequired)`; `400` →
`provider_invalid_request_or_spend_limit` and `429` →
`provider_rate_or_spend_limit`, both `(Provider, Unknown, Unknown)` and never
`QuotaExhausted`; unlisted/absent stays generic `provider_failed`. No status
or rate notice authorizes retry; no prose, quota, account, or credential
crosses the wire; one-attempt, redaction, MCP, cleanup, and absent-field
behavior preserved.

Validation (provider-free): `effigy validate:focused
swallowtail-adapter-claude-agent` 498 passed; `effigy
package:verify-affected swallowtail-adapter-claude-agent` passed; `effigy
qa:docs` passed; `git diff --check` clean. New fixtures: wire decode,
classification, malformed-value, redaction, turn-reset, and unknown-status
cases in `wire_tests.rs`, driver `framing.rs`, sidecar-asset tests with five
new fake-SDK scenarios, and `events.jsonl`. Exact-head independent re-review
was accepted at `5d85cc93fd08cedbf8089ad171e35c88d3691ad2` in review comment
`5598151018`; PR 297 merged into `main` as
`24f88fb8a1328aa0e85b9c91989962ba32b9c590`. Hosted PR checks passed; the
configured Pinned MSRV floor tests job was skipped. No provider run occurred,
and no qualification, release, tag, or Desktop diagnostic followed.

The known pre-existing
`readiness::wrapper_death_preserves_partial_capture_journal` flake remains
deferred and recorded in `PAPERCUTS.md`; it reproduced on the clean base and
was not introduced by Card 152.
