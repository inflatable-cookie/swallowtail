# 2026-07-29 Codex Exec Activity And Fidelity Closeout

## Changed

- Added a production exec JSONL activity projector separate from app-server.
- Published the exact exec activity profile through prepared structured runs.
- Kept final structured output, final operation output, and activity separate.
- Closed card 124 and roadmap g02.036.

## Current State

- Assistant, reasoning-summary, file-change, and warning items are
  completion-only.
- Commands, MCP tools, and searches carry start and completion.
- Todo lists carry start, replacement updates, and completion.
- Collaboration carries start and completion from qualified `0.92.0`.
- Unknown semantic events remain bounded and namespaced. Missing identity,
  lifecycle widening, and malformed content fail closed.
- Stable `0.146.0` remains an allowed unverified-newer attempt on the
  `0.145.0` activity guarantee.
- No consumer, authentication, provider, or live harness operation changed.

## Evidence

- 124 deterministic Codex adapter tests
- exact prepared profile checks across baseline, collaboration, latest
  qualified, and unverified-newer points
- workspace compile and lint
- public API declaration baseline
- all 23 local package archives and the extracted workspace
- packaged Codex prepared profile suite

## Next

Card 125 revalidates ACP activity semantics and freezes the shared plus
provider-specific corpus before production protocol projection.
