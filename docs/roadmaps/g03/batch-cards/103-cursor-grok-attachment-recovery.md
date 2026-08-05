# 103 Cursor And Grok Attachment Recovery

Status: completed
Owner: Tom
Created: 2026-08-05
Milestone: `../038-provider-wide-interactive-crash-recovery.md`
Depends on: card 102

## Goal

Map exact Cursor and Grok provider sessions into bounded attachment recovery
without exposing their replay as authoritative history.

## Scope

1. Issue durable bindings from ordinary qualified session creation.
2. Validate exact plan, host, resource, access, version, and model posture.
3. Bound and discard pre-response ACP session updates.
4. Reject callbacks, foreign identity, malformed updates, overflow, late
   replay, cancellation, disconnect, and cleanup failure.
5. Return one live exact-session handle and no replay.

## Validation

- `effigy validate:focused swallowtail-runtime swallowtail-adapter-cursor swallowtail-adapter-grok`

## Stop Conditions

- stop if either route cannot prove the response attached the requested session
- stop if Grok authentication or model selection can drift during attachment

## Auto-Continuation

Continue to card 104 when both deterministic route suites pass.

## Outcome

- Cursor and Grok ordinary sessions now issue durable exact bindings
- both routes load the exact bound session and discard bounded replay
- 8 MiB update limits remain separate from 16 MiB frame limits
- foreign, malformed, oversized, late, disconnected, callback, and response
  mismatch cases return no handle and release owned resources
- focused ACP, Cursor, and Grok validation passed: 158 tests
