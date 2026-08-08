# Runtime Provider History Page

Date: 2026-08-08
Roadmap: g03.057
Card: 176

## Outcome

Runtime lands Contract 054's portable history-page surface.

Core adds `OperationShape` / `DriverRole` / `Capability` /
`CancellationScope::ProviderSessionHistory`. Runtime exposes plan, request,
plan-bound older cursor, page response, `Exact` / `AtLeast` / `Unknown`
totals, and `page_provider_session_history_window` for newest-first slices
over an ascending `SessionReplayItem` snapshot. `ProviderSessionHistoryDriver`
defaults to unsupported and returns no live session handle. Load and
reconciliation APIs stay behavior-compatible; `replay_complete` is unused.

The v0.3.0 candidate semantic API baseline was regenerated for the additive
core/runtime (and already-landed debug-observation) surface.

## Local Validation

- `effigy validate:focused swallowtail-runtime`: 174 passed
- `effigy package:api`: 28 packages at the regenerated v0.3.0 candidate
  baseline

## Boundaries

No Codex adapter emissions, guide inventory, native turn pagination, live
provider work, tag, or release. Card 177 owns the Codex synthetic proof.
