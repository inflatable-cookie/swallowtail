# 114 Paged Provider Session History

Status: promoted
Owner: Tom
Date: 2026-08-08

## Question

How should Swallowtail expose newest-first, consumer-driven history pages for
provider-owned sessions without collapsing load, reconciliation, or consumer
transcript ownership into one operation?

## Method

Evidence inspected on 2026-08-08:

- Contracts 017, 025, 038, 044, 046, 048
- Runtime `SessionReplayItem`, `LoadedSession`, catalogue cursors/bounds, and
  `bound_provider_session_replay_tail`
- Codex app-server catalogue/import/reconciliation: `thread/read` with
  `includeTurns: true`, full-turn projection, and bounds
  (`MAXIMUM_REPLAY_TURNS` / items / bytes)
- Research 093: `thread/resume.initialTurnsPage` appears at Codex `0.139.0`
  and was left additive/unselected for catalogue/import
- Contract 025 Alibaba retained load: ascending pages with `has_more` and
  last-item cursor, complete replay before readiness

No live provider, credential, or paid operation was used.

## Problem

Consumers that paint chat UIs need a bound newest page first, then older pages
on scroll-back, with metadata for how many items arrived and how much history
remains. Today:

| Surface | Behavior |
| --- | --- |
| `load_session` | Adapter walks provider pages (when any) until complete; returns full bounded replay or fails. Partial replay is not readiness. |
| Reconciliation | Optional bounded **replacement snapshot** plus `replay_complete` meaning “fit agreed bounds,” not scroll state. No handle, no control. |
| Catalogue | Paged **session candidates**, not message history. |
| Consumer transcript | Downstream (038/044). Swallowtail is not the chat store. |

Codex still materializes the full turn list from one `thread/read` for load,
import revalidation, and reconciliation. Native initial-turn pagination exists
on resume from `0.139.0` but is not a qualified history-read surface yet.

## Recommendation

### Unify substrate, not operations

Share one portable history-page vocabulary across load (internal walk),
reconciliation (tail snapshot), and a new read role:

- `SessionReplayItem` projection
- item and byte bounds
- opaque, plan-bound cursor
- page direction (`NewestFirst` for the consumer API; ascending walks remain
  adapter-private where the wire requires them)
- page metadata: fetched count, older/newer availability, total as
  `Exact` / `AtLeast` / `Unknown`

Do **not** fold UX paging into reconciliation or weaken load’s
complete-before-ready rule.

### New operation

Add a read-only **provider-session history page** role/request:

1. Requires an exact durable binding (or an already attached handle when a
   route qualifies handle-scoped reads).
2. First page returns the newest bound window plus metadata.
3. Later pages accept only the opaque older cursor from a prior page of the
   same plan.
4. Grants no turn start, resume, load, import, archive, delete, or callback
   authority.
5. Does not claim interrupted-turn state (048 stays separate).
6. Does not become consumer transcript truth (038/044 stay separate).

### Codex proof posture

First proof on `codex.app-server`:

- expose the portable page API over projected `SessionReplayItem`s from a
  bounded `thread/read(includeTurns: true)`
- synthesize newest-first pages and opaque older cursors when the wire returns
  a full turn list within existing replay bounds
- fail closed when history exceeds those bounds (same ceiling as today’s load
  projection), unless a later card qualifies native turn pagination
- keep ordinary `load_session` all-or-nothing; history pages may accompany
  `resume_session` or run against a binding without forcing full load replay
  into the UI

Native Codex turn pagination (`initialTurnsPage` and successors) remains a
later qualification once exact request/response evidence is frozen for a
history-read path.

### Why this shape

- Matches catalogue’s proven cursor/bounds pattern (046) without overloading
  candidate discovery.
- Preserves 017 load readiness and 048 observe-only authority.
- Gives Nucleus and other consumers a portable page DTO before every route
  has wire-native reverse pagination.
- Lets adapters that already page on the wire (Alibaba items, OpenCode
  messages, Anthropic managed events) project into the same DTO later.

## Tradeoffs

| Choice | Cost | Benefit |
| --- | --- | --- |
| Separate history role vs paging `load_session` | another role | load stays attach+prove; UI can resume then browse |
| Synthetic Codex pages vs wait for native cursors | first fetch may still be heavy | portable API and UX land now; native paging upgrades later |
| Shared item type with reconciliation | careful completeness vocabulary | one projection path; less adapter churn |
| Totals as Exact/AtLeast/Unknown | consumers must handle Unknown | honest across harnesses that omit counts |

## Open Questions Settled For Promotion

1. **Unify with reconciliation?** Substrate yes; operation no.
2. **Direction?** Consumer API is newest-first pages toward older history.
3. **First proof route?** Codex app-server with synthetic pages under existing
   replay bounds.
4. **Consumer transcript?** Remains Nucleus/other app ownership.

## Promotion Targets

- Contract 054 (paged provider-session history)
- Amendments to Contracts 017 and 048
- Architecture note
- Spec 005 as provisional planning history
- Roadmap g03.057 and ready cards for runtime DTO/role, Codex proof, guide

## Validation Needs

- runtime page/cursor/metadata fixtures
- Codex fixture: newest page, older page, bound overflow, no control side
  effects
- docs inventory for the new feature and Codex route note
