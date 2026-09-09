# 2026-09-09 g05 Card 152 Structured Provider Failure Implementation

Card 152 repairs the Claude Agent SDK structured provider-failure projection
provider-free. Research 300 proved Card 316's capsule carried present
`api_error_status` and `terminal_reason` that the sidecar discarded; Contract
019 (planning head `6b8ecdff`) fixed the safe boundary. No provider ran.

## What changed

- Sidecar (`sidecar/claude-agent-sdk-sidecar.mjs`): `turn_ended` now carries
  validated `apiErrorStatus` (safe integer `100..=599`),
  `terminalReason` (`/^[A-Za-z0-9_.-]{1,96}$/`), and the latest active-turn
  `rateLimitStatus` (`allowed`/`allowed_warning`/`rejected`). Present but
  malformed values fail closed as `unknown_message`. Rate state resets at
  each turn boundary; idle notices validate without attaching.
- Strict wire (`src/sdk/wire.rs`, `wire/decode.rs`): `TurnEnded` carries the
  three facts; absent/null stays `None`, malformed fails closed as
  `InvalidEvent`. No wire or behavior revision change: the decoder accepts
  records without the new fields.
- Failed diagnostic (`src/sdk/turn.rs`, `turn/events.rs`): `402` maps to
  `swallowtail.claude-agent.sdk.provider_billing_unavailable` with
  `(Provider, EntitlementUnavailable, ConfigurationChangeRequired)`;
  `400` and `429` map to distinct explicitly mixed route codes with
  `(Provider, Unknown, Unknown)` and never `QuotaExhausted`; absent/unlisted
  stays generic `provider_failed`. No status authorizes retry.
- Fixtures and tests: wire decode, classification, malformed-value,
  redaction, turn-reset, and unknown-status cases in `wire_tests.rs`,
  driver `framing.rs` (six new scenarios in the fixture host), five new
  fake-SDK scenarios with sidecar-asset tests, journal keys, and the
  `events.jsonl` shape. Guide, changelog, and card result updated.
- One-attempt, redaction, MCP, cleanup, success, and absent-field behavior
  preserved.

## Validation

- `effigy validate:focused swallowtail-adapter-claude-agent`: passed.
- `effigy package:verify-affected swallowtail-adapter-claude-agent`: passed.
- `effigy qa:docs`: passed. `git diff --check`: clean.
- Known pre-existing flake, unrelated and already tracked in PAPERCUTS.md
  ("Cold nested courier target starves the capture wrapper test budget"):
  `readiness::wrapper_death_preserves_partial_capture_journal` fails
  intermittently under parallel load on the pristine base head too.
- During this work a sidecar edit accidentally deleted the
  `firstTurnRejection` guard in `handleQuery`, hanging session-terminal
  retries; caught by `first_turn_rejection_...` tests, guard restored,
  deletion audit across all touched files clean.

Exact-head independent re-review was accepted at
`5d85cc93fd08cedbf8089ad171e35c88d3691ad2` in review comment `5598151018`,
then PR 297 merged into `main` as
`24f88fb8a1328aa0e85b9c91989962ba32b9c590`. Hosted PR checks passed; the
configured Pinned MSRV floor tests job was skipped. No provider run occurred,
and no qualification, release, tag, or Desktop diagnostic followed.

## Closeout

The integration checkout was clean on `main` and synchronized with
`origin/main` at the merge commit before this closeout.

Final provider-free evidence remained truthful: `effigy
validate:focused swallowtail-adapter-claude-agent` passed with 498 tests;
`effigy package:verify-affected swallowtail-adapter-claude-agent` passed;
`effigy qa:docs` passed; and `git diff --check` was clean.

The known pre-existing
`readiness::wrapper_death_preserves_partial_capture_journal` flake remains
deferred and recorded in `PAPERCUTS.md`; it reproduced on the clean base and
was not introduced by Card 152. The sole Next Task pointer now records that
planning direction is needed; no new direction was introduced here.
