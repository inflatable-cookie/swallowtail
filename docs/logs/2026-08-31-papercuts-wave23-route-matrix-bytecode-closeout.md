# Papercuts wave 23 route-matrix bytecode closeout

Date: 2026-08-31
Handoff: `docs/handoffs/20260831-195246-papercuts-wave23-route-matrix-bytecode.md`
PR: [#137](https://github.com/inflatable-cookie/swallowtail/pull/137), merged
as `8cb66d1b512f6406a206f506a8beba018cb0f023`

## Outcome

- Disabled Python bytecode writes before route-inventory imports in the
  consumer-docs and guide checkers.
- Set `PYTHONDONTWRITEBYTECODE=1` in the route-matrix shell wrapper.
- Closed the matching `PAPERCUTS.md` entry without changing route data,
  validation meaning, workflows, or the roadmap.
- Accepted exact worker head
  `86804e3866b4783ab8556992850e4eec6c735cb2`.

## Validation

- With the host cache prefix neutralized, an unguarded import reproduced a
  source-tree `.pyc`; all three guarded selector paths left no bytecode behind.
- Consumer-docs, guides, routes, aggregate docs, and Northstar QA passed.
- A controlled invalid feature matrix remained rejected.
- The first Stable CI attempt hit the known unrelated OpenCode deadline race.
  The exact test passed 10/10 locally; the unchanged-head retry passed, along
  with all other required checks.

## Scope and next

- Both reserved wave 23 papercuts are complete; no later papercut is queued.
- The g05 roadmap Next Task remains card 024.
- No provider behavior, route claim, public API, or release evidence changed.
