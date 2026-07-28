# 093 Session Continuity Contract And Corpora

Status: completed
Owner: Tom
Created: 2026-07-28
Milestone: `../028-session-continuity-feature-closure.md`
Depends on: card 092

## Objective

Prove the existing session-continuity contracts cover the selected five cells
and freeze their exact maintained-range corpora.

## Scope

1. Confirm Contracts 009, 017, and 038 settle:
   - Codex app-server load
   - Claude Agent ACP load and resume
   - OpenCode HTTP load and resume
2. Add no shared contract unless exact corpus work finds a contradiction.
3. Freeze Codex `0.80.0..=0.145.0` fixtures across exact schema milestones:
   history-bearing resume response, bounded ordered replay, replay-free
   resume selection, wrong thread, malformed history, overflow, cancellation,
   disconnect, and cleanup.
4. Freeze Claude Agent ACP `0.53.0..=0.61.0`, excluding withdrawn `0.58.0`:
   exact load/resume negotiation, bounded replay completion, replay-free
   resume, opaque binding, mismatch, cancellation, close-with-retained-history,
   disconnect, and cleanup.
5. Freeze OpenCode `1.14.48..=1.18.4` across every qualified schema segment:
   session detail, message-list pagination and ordering, exact-session
   continuation, opaque binding, mismatch, abort, disconnect, and
   attached-server cleanup.
6. Keep load, resume, native close, archive, restore, and delete independent.
7. Use no live account, credential, executable, provider request, container,
   or model server.

## Acceptance Criteria

- [x] selected route identities and version surfaces are exact
- [x] load replay and replay-free resume have separate fixtures
- [x] close effect and retained provider state remain explicit
- [x] every guaranteed version segment is represented
- [x] cancellation and cleanup leave no detached work
- [x] diagnostics expose no session id, transcript, or raw payload
- [x] implementation scope is contract-ready and bounded

## Result

- Contracts 009, 017, and 038 cover all five selected cells. No shared
  contract changed.
- Codex has six exact continuity segments across
  `0.80.0..=0.145.0`. `excludeTurns` begins at `0.129.0`; older resume
  responses remain bounded and expose no replay phase.
- Claude Agent ACP freezes all ten qualified published releases across
  `0.53.0..=0.61.0`, excluding `0.58.0`, with awaited load replay and
  replay-free resume.
- OpenCode freezes seven recursively closed wire surfaces and twelve
  published segments across all 45 qualified releases.
- Negative fixtures cover mismatch, wrong ordering, unexpected replay,
  overflow posture, cancellation, disconnect, redaction, and joined cleanup.

## Evidence

- Research 052 records the exact contract fit and tagged-source findings.
- 60 focused corpus, compatibility, lifecycle, and adjacent protocol tests
  pass.
- No production behavior, live access, executable, provider state, container,
  or model server was used.

## Auto-Continuation

Satisfied. Continue to card 094.
