# 094 Session Continuity Implementation Tranche

Status: completed
Owner: Tom
Created: 2026-07-28
Milestone: `../028-session-continuity-feature-closure.md`
Depends on: card 093

## Objective

Implement the contract-ready session-continuity tranche through existing
prepared route identities.

## Scope

1. Implement exactly:
   - Codex app-server load
   - Claude Agent ACP load and resume
   - OpenCode HTTP load and resume
2. Reuse provider-neutral load/replay and replay-free resume records from
   Contract 017. Add no generic prompt or lifecycle facade.
3. Codex:
   - project bounded ordered `thread.turns` for load
   - keep response thread identity exact
   - use `excludeTurns: true` for resume from `0.129.0`
   - bound and ignore returned turns on older qualified segments
4. Claude:
   - bind exact session id, cwd, and MCP server values
   - finish load replay before returning ready
   - expose no resume replay
   - preserve realized native-close history semantics
5. OpenCode:
   - bind exact attached target and session
   - page messages under positive item, page, and byte bounds
   - reverse page sequence without reversing item sequence
   - subscribe and continue the exact session without replay on resume
6. Preserve cancellation, deadlines, version posture, local and
   remote-authoritative topology, credential-last release, joined cleanup,
   and safe diagnostics.
7. Update matrix cells only after public prepared paths and conformance exist.
8. Use deterministic fixtures first. Keep live authentication separately
   gated.

Pi RPC load and resume remain a continuation tranche. Alibaba Conversations
and Anthropic Managed Agents remain outside this card until a retained
hosted-session contract branch exists.

## Acceptance Criteria

- [x] all five matrix cells map to public prepared operations
- [x] load returns bounded ordered replay before readiness
- [x] resume returns no replay phase
- [x] exact binding mismatch fails without a usable handle
- [x] every guaranteed version segment is exercised
- [x] cancellation, overflow, disconnect, and cleanup are joined
- [x] diagnostics expose no session id, transcript, or raw payload
- [x] no archive, restore, delete, native-close, or server-stop claim widens

## Evidence

- Codex app-server loads bounded `thread.turns`; resume requests
  `excludeTurns: true` from `0.129.0` and bounds then ignores older returned
  turns.
- Claude Agent ACP loads replay before readiness and resumes without replay
  across all ten qualified releases.
- OpenCode pages attached-session messages under fixed page, item, and byte
  limits; load restores oldest-first and resume performs no history request.
- All five operations execute through public prepared profiles. Exact
  management-binding origins remain `Loaded` or `Resumed`.
- Focused corpora, prepared-facade tests, all three adapter suites, and the
  full workspace suite pass without live access.
- The solution matrix moves five cells from `No` to `Yes`: three load and two
  resume. It now contains 432 `No` cells; session continuity contains 53.

## Auto-Continuation

Satisfied. Card 095 is ready.
