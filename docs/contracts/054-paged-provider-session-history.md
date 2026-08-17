# 054 Paged Provider Session History

Status: accepted
Owner: Tom
Updated: 2026-08-08

## Purpose

Give consumers a read-only, newest-first page of provider-owned session
history with explicit pagination metadata, without treating that page as load
readiness, reconciliation observation, or consumer transcript truth.

## Boundary

Provider-session history pages are independent of:

- `load_session` complete-before-ready replay (Contract 017)
- `resume_session` and recovery attachment (Contract 017 / 050)
- provider-session catalogue and import (Contract 046)
- cross-process reconciliation snapshots (Contract 048)
- provider archive, restore, delete, and native close (Contract 038)
- consumer thread and transcript persistence (Contracts 038 and 044)

A history page grants no turn start, steering, callback answer, import, load,
resume, archive, restore, delete, or management authority. Observation of
interrupted-turn or interrupted-run state remains Contract 048 only.

## Shared Substrate

History pages reuse the portable `SessionReplayItem` projection used by load
replay and reconciliation snapshots. They also reuse the catalogue-style
discipline of:

- immutable plan agreement for scope, bounds, and deadline
- opaque, plan-bound continuation cursors
- finite item and byte bounds

Adapters may walk provider wires in ascending or provider-native order
internally. The consumer-facing page API is newest-first: the first page is
the newest bound window; later pages move strictly toward older history.

Load and reconciliation may call the same projection and bound helpers. They
must not expose this page role as a substitute for their own contracts.

## Plan And Request

Side-effect-free preparation fixes at least:

- integration family, driver, transport, and protocol facade
- configured instance and exact target revision
- authoritative execution host
- access profile and safe provenance
- exact interface versions when claimed
- exact durable session binding or handle-scoped authority when qualified
- exact working-resource posture matching the binding
- page item and byte bounds
- optional deadline and cancellation posture
- required host services

The request carries one request id, the plan agreement, an optional older
cursor from a prior page of the same plan, and cancellation. A cursor from
another plan, binding, route, host, or resource fails closed before provider
work.

## Page Response

One successful page contains:

- ordered `SessionReplayItem` values for that page only, in ascending replay
  order within the page (oldest item in the window first) so consumers can
  prepend older pages above newer ones without re-sorting
- `fetched_count` for the returned item count
- older-page availability (`has_older`)
- optional opaque `older_cursor` when `has_older` is true
- total history cardinality as one of:
  - `Exact(n)` when the provider or a complete bounded snapshot proves it
  - `AtLeast(n)` when only a lower bound is known
  - `Unknown` when no honest total exists

Traversal is newest-window-first: the first request without a cursor returns
the newest bound window; each older cursor moves toward earlier history.

Empty first pages are valid when the bound session has no projectable
history. Empty continuation pages, cursor non-progress, duplicate item ids
that break ordering, bound overflow, foreign session identity, and plan drift
fail the page and return no cursor authority for further traversal.

Partial provider failures during a page fail the page. They do not invent a
usable handle, do not mutate provider state, and do not delete consumer
transcript rows.

## Completeness Vocabulary

Do not overload reconciliation’s `replay_complete`:

| Flag | Meaning |
| --- | --- |
| History page `has_older` | Another older page may be requested with the returned cursor |
| History total `Exact` / `AtLeast` / `Unknown` | Cardinality honesty for UI chrome |
| Reconciliation `replay_complete` | Whether one replacement snapshot fit agreed item/byte bounds |
| Load readiness | Full qualified replay finished before a live handle is returned |

An incomplete history traversal never authorizes load readiness and never
proves provider-history absence for consumer merge policy.

## Route Capability

Routes advertise history-page support explicitly. Absence means unsupported,
not “use load instead.” A route may implement:

- provider-native pagination projected into portable pages
- synthetic pages over one bounded provider snapshot when the wire returns
  only a full history within Swallowtail bounds

Synthetic paging must still fail closed when the snapshot exceeds the agreed
page or operation bounds. It must not silently truncate without `has_older` /
total honesty appropriate to the evidence.

## Route Mappings

Routes that advertise history today:

| Route | Snapshot | Notes |
| --- | --- | --- |
| `codex.app-server` | bounded `thread/read(includeTurns: true)` | first proof; synthetic newest-first pages; ambient working resource |
| `opencode.http` | ascending `session_messages` via shared load-replay helper | synthetic pages; ambient working resource; same qualified-server gate as import/reconcile |
| `alibaba.conversations` (retained) | ascending conversation items walk shared with load | synthetic pages; resource-free; no live handle |
| `deepseek-harness.local-server` | direct unary `session.history` pages using `beforeSeq` | pinned RC6 corpus proves control-free paging; complete bounded walks yield `Exact(n)`; live smoke remains operator-gated |

Shared rules for those mappings:

- portable pages are sliced newest-first from the projected replay items
- older cursors are opaque and plan-bound
- overflow fails closed under the route’s existing replay ceilings plus plan
  page/snapshot bounds
- the operation issues no turn start, interrupt, resume, archive, restore,
  delete, import, or load-handle grant

Native Codex initial-turn pagination remains a later exact qualification and
does not inherit from the synthetic Codex mapping.

ACP routes whose only history wire is control-granting `session/load`
(`claude-agent.acp`, `kimi-code.acp`) stay unsupported until a control-free
history wire is qualified. `kimi-code.local-server` likewise stays unsupported
until a control-free transcript or messages API is qualified — session
lifecycle and WS activity catch-up are not that wire (Research 115).

## Conformance

Portable and route tests must cover:

- first newest page and older continuation
- cursor/plan mismatch and foreign session rejection
- bound overflow and empty history
- totals Exact / AtLeast / Unknown as applicable
- no control, import, load, resume, or management side effect
- joined cleanup that preserves the provider session
- clear separation from load readiness and reconciliation snapshots

## Acceptance

- consumers can render a newest bound window without receiving the entire
  history as one load-replay phase
- load remains complete-before-ready
- reconciliation remains observe-only with replacement-snapshot semantics
- cursors cannot cross plan, binding, route, host, or resource boundaries
- totals remain honest when counts are unknown
- Codex, OpenCode, and Alibaba retained fixtures prove synthetic newest-first
  paging under existing bounds where the route can page without a live handle
- DeepSeek Web fixtures prove a control-free unary history walk with no resume
  or live handle; its live qualification remains a separate gated selector
