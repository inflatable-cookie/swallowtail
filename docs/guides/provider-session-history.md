# Provider Session History Pages

Use this path when a chat UI needs a bound newest history window first, then
older pages on scroll-back, without treating that browse as load readiness or
reconciliation. New to the shared vocabulary? Read
[Key Concepts](key-concepts.md) first.

Authority: [Contract 054](../contracts/054-paged-provider-session-history.md).
Related: [Contract 017](../contracts/017-provider-owned-session-load-replay-and-host-containment.md)
(load complete-before-ready),
[Contract 048](../contracts/048-cross-process-active-operation-reconciliation.md)
(observe-only interrupted work),
[session import](provider-session-import.md),
[provider operation reconciliation](provider-operation-reconciliation.md).

## When To Use It

Ordinary apps may:

1. hold an exact durable `SessionResumeBinding` (from import or an earlier
   session)
2. `resume_session` when they need a live handle
3. call the history-page role to paint newest-first windows for scroll-back

Do not use history pages as:

- load readiness — `load_session` still finishes full qualified replay before
  the handle is ready
- reconciliation — interrupted-turn state stays on Contract 048;
  `replay_complete` means snapshot-fit-bounds, not “has older pages”
- consumer transcript authority — Swallowtail is not the chat store
  (Contracts 038 and 044)
- control flow — pages grant no turn start, interrupt, resume, archive,
  restore, delete, import, or callback answer

Absence of the capability means unsupported. Do not fall back to load.

## Page Shape

One successful page carries:

- ascending `SessionReplayItem` values for that window only (oldest in the
  window first) so older pages prepend without re-sorting
- `fetched_count`
- `has_older` plus an opaque plan-bound `older_cursor` when another older page
  may be requested
- total cardinality as `Exact(n)`, `AtLeast(n)`, or `Unknown`

Traversal is newest-window-first. Empty first pages are valid when the bound
session has no projectable history. Empty continuation pages, cursor/plan
mismatch, bound overflow, and dishonest totals fail closed.

## Consumer Sequence

1. Prepare the route that advertises `ProviderSessionHistory`.
2. Build a history plan for one exact binding, page item/byte bounds, snapshot
   ceiling, and optional deadline.
3. Request the first page with no cursor.
4. Render the window; if `has_older`, keep the opaque older cursor with the
   same plan.
5. Request later pages only with that plan-bound cursor.
6. Treat totals as UI chrome honesty, not proof of provider absence.

## Routes That Advertise History

| Route | Prepared entry | Snapshot source | Posture |
| --- | --- | --- | --- |
| `codex.app-server` | `prepare_session_history` / `CodexSessionHistoryInput` | bounded `thread/read(includeTurns: true)`, synthetic newest-first pages | ambient harness working resource |
| `opencode.http` | `prepare_session_history` / `OpenCodeSessionHistoryInput` | ascending `session_messages` walk via the same `load_replay` helper as reconciliation, then synthetic pages | ambient harness working resource; qualified server gate matches import/reconcile |
| `alibaba.conversations` (retained) | `prepare_session_history` / `AlibabaSessionHistoryInput` | ascending conversation items walk shared with load, then synthetic pages | resource-free; no live handle |

All three return `Exact` totals for the projected snapshot, fail closed on
snapshot overflow, and issue no control, import, load, resume, archive,
restore, or delete.

### Codex notes

Until native turn pagination is separately qualified, Codex pages are synthetic
over one bounded thread read. Version gate matches the thread-catalogue corpus.
Native Codex initial-turn pagination remains a later exact qualification.

### OpenCode notes

History reuses health + session get + message pagination without
`session/status` interrupted-state observation. Version gate matches catalogue,
import, and reconciliation: unqualified or unverified-newer servers do not
prepare history.

### Alibaba retained notes

History shares the ascending items walk with load but never returns a handle.
Load remains complete-before-ready. Resource-free bindings only.

## Routes That Stay Unsupported

These routes may load or attach sessions, but they do **not** advertise
`ProviderSessionHistory` today:

| Route | Why |
| --- | --- |
| `claude-agent.acp` | History today only rides `session/load`, which attaches a live control-capable handle. Contract 054 forbids wrapping load as history. |
| `kimi-code.acp` | Same class: attach-path `session/load` is not a control-free history wire. |
| `kimi-code.headless` | No retained session history surface. |
| `kimi-code.local-server` | Reconciliation returns empty replay; no transcript history wire. |

Do not derive history support from load, resume, catalogue, import, or
reconciliation. A control-free ACP history wire is a separate qualification.

This guide does not add a column to the main provider feature CSV. Support is
route-specific; see the route matrix rows for the three advertising routes.

## Validation

```sh
effigy validate:focused swallowtail-runtime
effigy validate:focused swallowtail-adapter-codex
effigy validate:focused swallowtail-adapter-opencode
effigy validate:focused swallowtail-adapter-alibaba-model-studio
effigy qa:docs
```

Live provider history paging remains an operator-gated probe, never
deterministic acceptance.
