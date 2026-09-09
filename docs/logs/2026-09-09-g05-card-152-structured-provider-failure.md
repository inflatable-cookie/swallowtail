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

Exact-head independent review and merge pending; no Desktop one-shot follows
until this merge supplies its SHA.
