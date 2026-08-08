# 005 Paged Provider Session History

Status: promoted
Owner: Tom
Updated: 2026-08-08

## Purpose

Plan a read-only, newest-first provider history page surface that shares
replay item projection with load and reconciliation without changing their
lifecycle or authority.

## Scope

In:

- portable history page request/response, cursors, bounds, and totals
- new read-only driver role distinct from load, resume, import, and
  reconciliation
- Codex app-server first proof (synthetic pages under existing replay bounds)
- consumer/operator guide notes

Out:

- Nucleus local transcript paging (consumer store)
- weakening `load_session` complete-before-ready
- turning reconciliation into a scroll API
- native Codex turn-pagination qualification beyond noting the later gate
- live provider work in the first cards

## Decisions Needed

Settled by Research 114 and promoted into Contract 054:

1. unify substrate (`SessionReplayItem`, bounds, cursors, completeness)
2. separate operation from load and reconciliation
3. newest-first consumer pages with opaque older cursors
4. totals as Exact / AtLeast / Unknown
5. Codex synthetic pages first; native wire paging later

## Acceptance Criteria

- [x] research memo records evidence and recommendation
- [x] Contract 054 and amendments to 017/048 govern the surface
- [x] architecture notes the shared substrate and separate roles
- [x] roadmap and ready cards sequence runtime, Codex proof, and guide work

## Promotion Targets

- `docs/contracts/054-paged-provider-session-history.md`
- `docs/contracts/017-provider-owned-session-load-replay-and-host-containment.md`
- `docs/contracts/048-cross-process-active-operation-reconciliation.md`
- `docs/architecture/system-architecture.md`
- `docs/roadmaps/g03/057-paged-provider-session-history.md`
