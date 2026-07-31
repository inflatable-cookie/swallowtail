# 017 Antigravity Turn-Scoped Continuation

Status: planned
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

- [ ] continuation never selects ambient latest provider state
- [ ] provider conversation identity remains opaque and redacted
- [ ] each turn has independent cancellation and one terminal outcome
- [ ] missing or stale bindings fail without fallback
- [ ] retention truth is explicit and provider-specific
- [ ] turn-scoped continuation conformance passes deterministically

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

Yes. Continue to card 018 after continuation conformance passes.

