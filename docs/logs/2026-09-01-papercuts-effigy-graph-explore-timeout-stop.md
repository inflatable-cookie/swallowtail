# Papercuts Effigy graph-explore timeout ownership stop

Date: 2026-09-01
Branch: `worker/papercuts-effigy-graph-explore-timeout`
Base: `18a6907e75e55a6b181632a1da35a2fefd0824fe`
Host Effigy: `v0.12.1+local.47458a1`
Papercut: Effigy graph explore can rebuild silently without a useful timeout
  (2026-08-31)

## Outcome

Evidence-backed stop. No Swallowtail code, config, wrapper, or task change can
honestly close the papercut. Entry stays open.

## Ownership

| Claimed fix piece | Owner | Swallowtail lever |
| --- | --- | --- |
| Bounded graph-query wall clock | Effigy `run_graph` / `EFFIGY_GRAPH_TIMEOUT_MS` | none in `effigy.toml` |
| Structured timeout diagnostic | Effigy `effigy.graph.timeout.v1` | none |
| Rebuild / refresh progress while waiting | Effigy graph worker (missing) | none |
| Agent entrypoint | built-in `effigy graph explore` | skill already documents timeout; no local re-export |

Swallowtail surfaces checked and empty for this mechanism:

- root `effigy.toml` / `config/`: docs-policy indexes and QA tasks only; no
  graph query timeout or progress keys
- `scripts/`: no `graph explore` wrapper
- `[docs_policy.graph]` in the managed skill is Markdown-profile config, not
  code-graph query budgeting
- `AGENTS.md` forbids package scripts that merely re-export Effigy tasks

Effigy already closed unbounded unexplained graph *query* hangs in its own
PAPERCUTS (2026-08-27) with the timeout envelope. Explicit `graph index` and
`graph watch` remain unbounded by design. Emitting mid-rebuild progress still
requires changing Effigy's detached worker path in
`src/runner/graph_command.rs` (and related codegraph health/progress output).

## Proof on this worktree

1. Stale index: `effigy graph status --json` reported `freshness.stale = true`
   with thousands of stale paths and a present `graph.db`.
2. Silent rebuild: `EFFIGY_GRAPH_TIMEOUT_MS=15000 effigy graph explore … --json`
   wrote 0 stdout and 0 stderr bytes for the full 15s while `refresh.lock` was
   held; only the final timeout envelope appeared.
3. Structured fallback: the envelope carried
   `error.details.schema = "effigy.graph.timeout.v1"`, `timeout_ms`,
   `health.refresh_in_progress` / summary, and `next` actions (status,
   raise budget, pay cold build via `graph index`).
4. Short-budget falsification: `EFFIGY_GRAPH_TIMEOUT_MS=50` returned the same
   schema quickly with `ok: false` (process exit 1).

No Effigy checkout was patched. No provider contact.

## Residuals

- Papercut remains open until Effigy emits rebuild progress (and any further
  product tightening of explore/index UX) in a released binary Swallowtail
  consumes.
- Next open Swallowtail papercut after this one:
  live-probe temporary-workspace cleanup (2026-08-30).

## Validation

- Docs-only stop record: `effigy qa:docs:index:logs` and `effigy qa:docs:links`
  after the log and PAPERCUTS serial edits.
