# 105 Gemini ACP Attachment Recovery Qualification

Status: completed negatively
Owner: Tom
Created: 2026-08-05
Milestone: `../038-provider-wide-interactive-crash-recovery.md`
Depends on: card 104

## Goal

Decide whether exact Gemini ACP evidence can promote its replacement action to
the stronger bounded attachment-recovery method.

## Scope

1. Revalidate exact maintained initialize and load behavior.
2. Require exact provider-session identity and readiness response.
3. Freeze pre-response update, callback, bound, failure, and cleanup cases.
4. Promote only if exact source or deterministic corpus proves the boundary.
5. Keep replacement supported if the stronger gate fails.

## Validation

- `effigy validate:focused swallowtail-protocol-acp swallowtail-adapter-gemini`

## Stop Conditions

- stop promotion if session identity or readiness is ambiguous
- do not require authenticated provider work by default

## Auto-Continuation

Continue to card 106 with either honest result.

## Outcome

- exact Gemini `0.51.0` evidence advertises `loadSession` but classifies load as
  observed, not claimed
- no deterministic load transcript proves identity, replay completion,
  callback behavior, bounds, or readiness
- Gemini retains the supported fresh replacement mapping
- no authenticated provider work ran
