# 227 ZCode App-Server Driver Core

Status: completed
Owner: Tom
Created: 2026-08-17
Milestone: `../071-zcode-app-server-foundation.md`
Depends on: card 226

## Goal

Implement exact installed discovery and one bounded ZCode app-server
structured run without consumer or provider policy inference.

## Scope

1. Add the exact runtime claim and payload digest for `0.16.3`.
2. Add target-bound discovery against the host-approved `zcode.cjs`.
3. Spawn interpreted `node` + `zcode.cjs app-server` with host-approved
   config, cwd, and stdio ownership.
4. Decode create, runtime-preferences, subscribe, send, session events,
   usage, thinking progress, text, tools, idle, terminal, safe failure,
   and unknowns.
5. Fold the Swallowtail-owned idle interval; do not treat send `accepted`
   as a result.
6. Join process-kill cancellation, deadline, event delivery, and cleanup.

## Out Of Scope

- prepared facade, guide, matrices, or live Effigy selector
- native `session/stop`, session resume, catalogue, or subagent control

## Acceptance Criteria

- [x] no TUI, login, desktop GUI, or ACP-bridge wrap occurs
- [x] create hangs are avoided by answering runtime-preferences
- [x] output and terminal records correlate to the admitted operation
- [x] missing-key failures map to portable credential/provider failure
      without leaking secrets
- [x] unsupported semantic records fail or remain explicitly namespaced
- [x] all process exits preserve separate provider, harness, host, runtime,
      and cleanup truth

## Validation

- `effigy validate:focused swallowtail-adapter-zcode` — passed (19 tests,
  warnings-denied Clippy) after cards 227-228

## Stop Conditions

- stop if the process service cannot keep stdout protocol-only
- stop if idle fold requires parsing private session files
- stop if cancellation would claim `session/stop`

## Auto-Continuation

Continue to card 228 once exact low-level discovery and execution are ready.
