# 105 Claude SDK Termination Cause Propagation

Status: ready
Owner: Tom
Created: 2026-09-06
Updated: 2026-09-06
Milestone: `../029-claude-sdk-interactive-parity.md`
Depends on: `v0.4.2` at `f94dd16f`; card 100's unresolved live termination; Desktop consumer evidence 2026-09-06 (session opened, one Send, `sidecar_terminated` after ~2 s, no reply)

## Defect

The first real consumer turn on `v0.4.2` ended with the generic failure
`swallowtail.claude-agent.sdk.sidecar_terminated` and nothing else. The
inner cause exists on the wire and is discarded in Rust:

1. `sdk/wire.rs:72` declares `Terminal(ClaudeAgentSdkFailure)` with
   `#[allow(dead_code)]`; `sdk/connection/pump.rs:166` matches
   `Terminal(_)` and emits the generic code. The sidecar's terminal code
   (`unknown_message`, `internal_error`, `callback_invalid`,
   `record_too_large`, and the rest of the `terminal(...)` sites) never
   reaches the consumer.
2. `sdk/wire/decode.rs:75` keeps only `stopReason` and `isError` from
   `turn_ended`; the sidecar's sanitized `subtype`, `numTurns`,
   `durationMs`, `errorTextPresent`, `errorTextType`, and
   `resultFieldPresence` are dropped.
3. The sidecar close response's bounded native exit evidence
   (`nativeExitObserved`, `nativeExitEvent`, code, signal,
   `sdkTransportCloseRan`, `closeTimeline`) is not projected into the
   route `CleanupOutcome` diagnostic in a consumer-readable form.

Card 100's own live record hit the same wall: `subtype success`,
`is_error true`, native exit `1`, cause unresolved.

## Scope

1. Terminal: carry the sidecar terminal code into the failure code as
   `sidecar_terminated: <code>` (the card 100 `open_rejected: <code>` shape)
   and into the diagnostic message; the sidecar message text stays bounded.
2. Turn end: decode every sanitized `turn_ended` field and project it into
   the turn's terminal diagnostic and the `TurnEnded` event so a consumer can
   log `subtype`, `isError`, `numTurns`, `durationMs`, and error-text
   presence without any error text crossing.
3. Close: project the native exit evidence and `closeTimeline` labels into
   the `CleanupOutcome` diagnostic.
4. Sidecar stderr: the host already bounds it (`pump.rs:38`); retain the
   sanitized tail (bounded bytes, redacted by the existing allowlist) on the
   terminal failure diagnostic so the native's last words survive.
5. Qualify the pinned `rate_limit_event` under the direct operator scope
   extension: known advisory events project progress; result/error remains
   authoritative for turn failure. Malformed and genuinely unknown messages
   stay terminal. Record exact evidence and boundary proofs in Result.
6. Fixture proofs on the fake SDK: each terminal code surfaces verbatim; a
   `turn_ended` with `isError` carries all fields; close evidence appears in
   the cleanup diagnostic; stderr tail is bounded and redacted.
7. Guide section "Reading a terminated session"; changelog `[Unreleased]`;
   additive baseline. This is `v0.4.3` content.

## Out Of Scope

Any change to what the sidecar sends the SDK; resume, MCP, model, effort;
forwarding provider error text.

## Acceptance Criteria

- [ ] every sidecar terminal code reaches the consumer verbatim in the failure code
- [ ] all sanitized `turn_ended` fields reach the consumer; no error text crosses
- [ ] close evidence reaches the cleanup diagnostic
- [ ] bounded redacted stderr tail on terminal failures
- [ ] `unknown_message` policy ruled with evidence
- [ ] fixture proofs; guide; changelog; additive baseline; one PR

## Validation

- `cargo fmt -p swallowtail-adapter-claude-agent -- --check`
- `effigy validate:focused swallowtail-adapter-claude-agent`
- `effigy package:verify-affected swallowtail-adapter-claude-agent`
- `effigy package:api`
- `effigy qa:northstar`
- `git diff --check`

## Review Oracle

Invariant: a session that dies tells the consumer why, in a bounded code the
consumer can act on, and never in provider error text. Smallest
counterexample: a `match Terminal(_)`.

## Stop Conditions

The cause requires provider error text to be actionable (return to
Chatterbox for a redaction ruling).

## Auto-Continuation

No. Stop for exact-head review.

## Result

Implementation ready for independent review. The stable runtime diagnostic code
remains `swallowtail.claude-agent.sdk.sidecar_terminated`; its message now carries
`sidecar_terminated: <code>`, matching `open_rejected`'s existing shape. The wire
`TurnEnded` variant retains every sanitized result field; failed runtime turns
include those fields in their terminal diagnostic. These types are crate-private,
so no exported Rust API baseline addition is needed.

Degraded/failed cleanup diagnostics include bounded native-close evidence.
Stderr retains at most 2 KiB and projects at most 240 characters plus a truncation
marker through fixed diagnostic vocabulary; arbitrary words and paths are
redacted. Only stderr consumed before the terminal record is available: this
change does not claim a complete native stderr drain after termination.

### Pinned SDK evidence and remaining cause

Inspected the official `@anthropic-ai/claude-agent-sdk` 0.3.259 tarball whose
SHA-256 matches the frozen identity:
`0c5740e44a536ab6fd32f2a7de0d508b75d34782ebc219b87aa8d834449a3f7e`.
Its `sdk.d.ts` declares `SDKRateLimitEvent` as a rate-limit information update
and includes it in `SDKMessage`. `SDKRateLimitInfo.status` is exactly `allowed`,
`allowed_warning`, or `rejected`; quota and overage observations are optional.
`SDKResultMessage` documents exactly one result per turn as the turn-complete
signal. This settles the projection without inventing a provider-failure policy.

The direct root/operator continuation authorizes qualification on this card and
PR245, superseding the diagnostic-only limitation at `fd00ae03`. A well-formed
`rate_limit_event` now projects only `{event: progress}` for all three declared
statuses. Required `rate_limit_info` object, status enum, and nonempty string
`uuid`/`session_id` are validated; optional quota/account/timing payload is never
forwarded. Malformed envelopes and unknown statuses keep `unknown_message`.
Genuinely unknown message types retain the same terminal behavior.

A `rejected` notification is not itself a result and does not manufacture a
terminal outcome. The following provider error result or thrown query failure
still fails the turn through the existing path. This preserves real rejection
without treating changing quota/overage information as a session-kill command.
Fake-SDK sequences prove allowed/allowed_warning -> assistant text -> successful
result, rejected -> error result without error text, malformed notifications,
and interrupt/close after advisory progress. Existing runtime failure,
cancellation, permission and cleanup proofs remain green.

The pinned `SDKResultError` carries `errors: string[]`; the `SDKResultMessage`
documentation also specifies error text in `result` for subtype success with
`is_error: true`. Presence/type projection now covers both and retains singular
`error` observation. Fake-SDK proofs assert that neither text crosses the wire.
The SDK input stream, tool permissions, and query options are unchanged.

The old fixture demonstrated the repaired projection gap; it did not prove that
Desktop emitted that record. There is still no captured live JSONL or provider
reproduction in this lane. Actual Desktop root cause remains unproved until a
later tagged real Send.

Release input: v0.4.3 diagnostic and rate-limit projection repair, same qualified package/native axes and
behavior revision. Root owns independent exact-head review, merge, candidate
preparation and any separately authorized release. The decisive consumer follow-up
is one bounded Send on the reviewed build, recording only terminal code/message
and cleanup evidence. No native UI, global tools, auth or consumer files changed.

### Validation receipts

- Extended focused adapter gate: 377/377 tests; warnings-denied all-target
  Clippy passed. Nextest `3413ca87-ce26-4fbc-b586-4998470e021d`.
- Affected-package proof passed for `swallowtail-adapter-claude-agent`.
- Public API gate passed; no exported API delta.
- Northstar docs and final formatting/whitespace checks passed.
- No live provider test or real Send success is claimed.
