# 2026-08-05 Provider-Wide Interactive Crash Recovery

Roadmap: `../roadmaps/g03/038-provider-wide-interactive-crash-recovery.md`
Cards: 102-106

## Changed

- extended Contract 050 from three restoration strengths to five
- added exact provider-session attachment with bounded replay discarded
- added fresh-session replacement with explicit provider-context loss
- made durable bindings represent exact model-less prepared routes
- mapped Cursor and Grok ACP to exact attachment recovery
- mapped Antigravity continuation, Gemini ACP, Pi RPC, and Qwen continuation to
  fresh replacement
- kept the seven stronger reconciliation and continuation mappings unchanged
- kept one-prompt routes outside automatic retry

## Current State

All 11 prepared interactive harness routes expose
`prepare_working_state_restoration`. Consumers inspect one static method, call
`restore`, and match one portable outcome. No failed method falls through to a
weaker method.

Cursor and Grok preserve the exact provider session. Their pre-response replay
is bounded to 4,096 updates and 8 MiB, discarded, and never used as transcript
or terminal evidence. Invalid identity, shape, size, ordering, callback,
disconnect, or readiness evidence returns no handle.

Gemini ACP remains on fresh replacement. Its exact corpus advertises load but
does not prove the full attachment boundary.

No authenticated provider work, provider prompt, callback answer, live
session load, or network probe ran.

## Validation

- common focused validation: 276 tests passed
- Cursor, Grok, and ACP focused validation: 158 tests passed
- Antigravity, Gemini, Pi, and Qwen focused validation: 160 tests passed
- Codex, OpenCode, and Kimi compatibility validation: 370 tests passed
- affected-package proof passed for 13 exact packages
- `effigy format:check`
- `effigy qa:docs`
- `effigy qa:routes`
- `git diff --check`

## Next Move

Execute card 093. Define the separate observe-then-attach sequence for routes
which can reconcile a lost operation before returning a settled live session.
