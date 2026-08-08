# Codex Provider History Page Proof

Date: 2026-08-08
Roadmap: g03.057
Card: 177

## Outcome

`codex.app-server` serves Contract 054 newest-first history pages from a
bounded `thread/read(includeTurns: true)` snapshot.

Prepared `CodexPreparedSessionHistory` / `prepare_session_history` build a
plan-bound page request. Execution projects turns through
`project_thread_history`, slices with `page_provider_session_history_window`,
and returns `Exact` totals. Older pages reuse opaque plan-bound cursors.
Overflow fails closed. The path issues no turn start/interrupt, resume,
archive, restore, or delete. Ordinary `load_session` readiness is unchanged.

The v0.3.0 candidate semantic API baseline was regenerated for the additive
Codex prepared surface.

## Local Validation

- `effigy validate:focused swallowtail-adapter-codex`: 169 passed
- `effigy package:api`: 28 packages at the regenerated v0.3.0 candidate
  baseline

## Boundaries

No native Codex turn-pagination qualification, other routes, guide inventory
closeout, live Codex work, tag, or release. Card 178 owns the guide.
