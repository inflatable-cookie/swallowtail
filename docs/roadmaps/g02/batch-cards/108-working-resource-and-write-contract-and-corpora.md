# 108 Working Resource And Write Contract And Corpora

Status: completed
Owner: Tom
Created: 2026-07-28
Milestone: `../032-working-resource-and-workspace-authority-feature-closure.md`
Depends on: card 107

## Objective

Close only the shared contract gaps selected by card 107 and freeze exact
offline corpora before implementation.

## Scope

1. Preserve route, host, scope, resource, access, topology, version, and
   support-authority identity.
2. Keep working-directory selection, callback I/O, direct filesystem access,
   provider tools, and containment independent.
3. Define exact admission, cancellation, deadline, uncertainty, and cleanup
   behavior for selected routes.
4. Freeze deterministic exact-range success and failure corpora.
5. Add no generic filesystem, shell, sandbox, or approval authority.

## Acceptance Criteria

- [x] every selected cell has a settled contract path
- [x] every selected version segment has deterministic evidence
- [x] working-resource support grants no implicit write authority
- [x] write support grants no implicit containment claim
- [x] implementation scope is bounded and fixture-first

## Evidence

- Research 059 finds no shared contract expansion: Contracts 010, 013, 015,
  017, 023, 029, 033, and 041 already settle the selected profile.
- The corpus freezes exact Gemini CLI `0.51.0`, `ReadWrite`, ACP write
  negotiation and dispatch, `auto_edit` / `autoEdit` agreement, rejection
  paths, safe diagnostics, and joined cleanup.
- The existing read-only Plan Mode profile remains unchanged.

## Auto-Continuation

Continue only when every selected route is contract-ready.
