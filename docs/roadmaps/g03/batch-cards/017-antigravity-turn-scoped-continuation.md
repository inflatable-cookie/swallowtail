# 017 Antigravity Turn-Scoped Continuation

Status: completed
Owner: Tom
Created: 2026-07-31
Milestone: `../006-antigravity-personal-harness-foundation.md`
Depends on: card 016

## Goal

Expose Antigravity conversation-id continuation through the portable
turn-scoped lifecycle without ambient latest-session selection.

## Scope

1. Capture the exact conversation id from a completed headless turn.
2. Build later turns with the explicit `--conversation` identity only.
3. Join each owned process and preserve turn-local streaming, cancellation,
   deadlines, and cleanup.
4. Reject missing, malformed, mismatched, or unavailable conversation ids.
5. Keep global `--continue`, provider archive, deletion, and callback claims
   out of the route.

## Acceptance Criteria

- [x] continuation never selects ambient latest provider state
- [x] provider conversation identity remains opaque and redacted
- [x] each turn has independent cancellation and one terminal outcome
- [x] missing or stale bindings fail without fallback
- [x] retention truth is explicit and provider-specific
- [x] turn-scoped continuation conformance passes deterministically

## Validation

- `effigy validate:focused swallowtail-adapter-antigravity`
- focused continuation, binding, retention, activity, cancellation, and cleanup
  tests
- no broad workspace suite or live provider conversation

## Stop Conditions

- Stop if exact-id continuation still resolves through ambient global state.
- Stop if the conversation identity cannot be retained without raw payloads.
- Do not claim provider session management from continuation alone.

## Auto-Continuation

Completed. Continue to card 018.

## Result

Research 080 qualifies exact-id restarted continuation for `agy` `1.1.9`.
The headless driver now exposes a separate interactive-session role whose first
turn starts without a selector and whose later turns use only
`--conversation <exact-id>`. It never selects ambient `--continue`.

Each turn owns and joins one child process. A completed clean first turn
commits its bounded conversation id privately. Later streams must repeat that
identity. Missing or changed identity, failure, cancellation, deadline, or
uncertain cleanup invalidates the runtime handle without retry or fresh-session
fallback. No provider-session reference, resume binding, management role, or
storage path becomes public.

Focused validation passed 27 tests across four binaries plus warnings-denied
checking in two seconds. No live provider conversation, account mutation,
credential read, consumer edit, broad workspace suite, or publication ran.
