# 104 Retained Execution Contract And Corpora

Status: completed
Owner: Tom
Created: 2026-07-28
Milestone: `../031-retained-execution-and-recovery-feature-closure.md`
Depends on: card 103

## Objective

Close only the shared contract gaps selected by card 103 and freeze exact
offline corpora before implementation.

## Scope

1. Preserve route, state, cursor, access, version, topology, and
   support-authority identity.
2. Keep retrieval, stream reattachment, reconnect, and provider-managed
   recovery independent.
3. Define exact cancellation, deadline, terminal, uncertainty, and cleanup
   behavior for selected routes.
4. Freeze deterministic exact-range success and failure corpora.
5. Add no generic retry, recovery, prompt, or fallback API.

## Acceptance Criteria

- [x] every selected cell has a settled contract path
- [x] every selected version segment has deterministic evidence
- [x] consumer retry and recovery policy remains downstream
- [x] uncertain provider state cannot become confirmed recovery
- [x] implementation scope is bounded and fixture-first

## Auto-Continuation

Continue only when every selected route is contract-ready.

## Outcome

Contract 042 now separates harness-managed retry, Swallowtail attempts,
consumer retry, active turns, attachments, connections, cursors, and session
resume. It requires explicit recovery acceptance, maximum-one Kimi
local-server reattachment, no prompt replay, safe remote uncertainty, and
joined cleanup.

Research 057 and `retained-execution.json` freeze the exact
`0.28.1` and `0.29.0..=0.29.2` policy, retry, cursor, acknowledgement,
failure, redaction, cancellation, topology, and cleanup corpus. Card 105 is
contract-ready for Kimi headless recovery, local-server recovery, and
local-server stream reattachment.
