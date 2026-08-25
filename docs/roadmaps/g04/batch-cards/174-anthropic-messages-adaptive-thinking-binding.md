# 174 Anthropic Messages Adaptive-Thinking Binding

Status: complete
Owner: Tom
Created: 2026-08-25
Updated: 2026-08-25
Milestone: [g04.062 Anthropic Messages Adaptive Thinking](../062-anthropic-messages-adaptive-thinking.md)
Depends on: card 173; non-empty Research 209 deliver-now table

## Goal

Bind only Research 209's exact adaptive-thinking rows through a typed
adapter-local prepared input, immutable evidence, exact request encoding, and
bounded private stream/continuation state.

## Work

1. Add the smallest opaque typed `AnthropicThinkingMode::adaptive()` surface;
   do not add raw values, a generic map, or a portable reasoning capability.
2. Admit the mode only on exact Research 209 model/profile rows and carry it
   through request, prepared evidence, operation/session policy, and driver
   validation before endpoint or credential use.
3. Encode only the exact qualified adaptive/omitted-display object. Preserve
   byte-identical omission and the independent `output_config.effort` field.
4. Extend the bounded SSE grammar for exact thinking, redacted-thinking,
   thinking-delta, and signature-delta shapes. Reject semantic drift.
5. For structured attempts, validate and discard private blocks at terminal;
   emit no thinking content or activity.
6. For direct continuation, retain the complete qualified private block
   sequence in zeroizing bounded memory and replay it unmodified in exact order
   before the correlated tool-use block.
7. Bind private state to the same instance, facade, access, route, model, and
   runtime session. Reject missing, duplicate, reordered, oversized, altered,
   foreign, or stale state without network fallback.
8. Clear private blocks on terminal disposal, invalidation, cancellation,
   deadline, session close, and fresh restoration. Preserve joined cleanup.
9. Keep thought text, signatures, redacted data, and raw blocks out of public
   events, activity, output, callbacks, evidence, formatting, and diagnostics.
10. Add focused fixtures, tests, example/API baseline, and guide changes only
    as required by the delivered surface.

## Acceptance Criteria

- [x] only Research 209 exact rows prepare and dispatch
- [x] omission remains byte-identical and effort composes independently
- [x] exact adaptive/omitted request bytes are deterministic
- [x] structured attempts expose and retain no private thinking content
- [x] direct continuation captures and replays the complete private sequence
- [x] every private byte is bounded, route-bound, redacted, and zeroized
- [x] malformed, incomplete, reordered, oversized, or drifted blocks fail
      before unsafe continuation
- [x] restoration remains fresh replacement with no private-state recovery
- [x] no shared contract/runtime or breaking public API change
- [x] `cargo fmt -p swallowtail-adapter-anthropic` passes
- [x] `effigy validate:focused swallowtail-adapter-anthropic` passes
- [x] `effigy package:verify-affected swallowtail-adapter-anthropic` passes
- [x] `git diff --check` passes

## Stop Conditions

- Research 209 is empty or contradicts the planned mapping
- safe replay requires public thought disclosure, durable storage, generic JSON,
  shared contract/runtime change, or a breaking API
- effort and thinking cannot remain independently exact

## Out Of Scope

- summarized thinking activity, manual budgets, unrelated Anthropic features,
  live provider work, currentness, release, merge, generation rollover, or g04
  closure
