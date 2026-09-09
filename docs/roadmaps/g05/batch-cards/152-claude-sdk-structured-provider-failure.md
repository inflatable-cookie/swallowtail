# 152 Claude SDK Structured Provider Failure

Status: ready; provider-free production repair
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

- [ ] numeric API status, bounded terminal reason, and fixed rate state survive end to end
- [ ] `402` produces a billing/entitlement-specific safe diagnostic and classification
- [ ] `400` and `429` remain explicitly ambiguous and never claim quota exhaustion
- [ ] malformed, absent, unknown, idle, and cross-turn cases fail closed or remain generic
- [ ] raw provider text and sensitive fields never cross the sidecar wire
- [ ] no result authorizes automatic retry or fallback
- [ ] existing success/generic failure and MCP behavior remain intact

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

Pending.
