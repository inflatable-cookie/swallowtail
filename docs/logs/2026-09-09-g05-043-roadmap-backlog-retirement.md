# 2026-09-09 g05.043 Roadmap Backlog Retirement

Status: complete; PR #302 merged at `8ae707d5`
Task: `docs/roadmaps/g05/043-roadmap-backlog-retirement.md`
Handoff: `docs/handoffs/20260909-151758-roadmap-backlog-retirement.md`
Base: clean synchronized `main` at `2e11aca8` (planning head `2e11aca81df148526968f1a0af4cd7332c55b29e`).

## What Changed

Applied the frozen nine-item disposition manifest and removed the duplicate
`docs/roadmaps/backlog/` intake layer. Roadmaps retain promoted executable
tasks; triage retains unresolved or deferred candidates without becoming
execution authority.

Disposition audit (former backlog item → destination):

- Provider-Session Management Binding Persistence → new deferred triage note
  `docs/triage/20260909-151758-provider-session-management-binding-persistence.md`
  (full scope, constraints, source g02 card 060, promotion gate: matrix
  checkpoint plus a concrete consumer need; operator owner; next check at the
  Contract 029 checkpoint). Promotion still needs explicit operator selection.
- Aider / Hosted Interactive OAuth / Kiro / OpenHands stubs → already parked;
  removed; meaning lives in
  `docs/triage/2026-08-21-deferred-route-surfaces.md` plus Research 143/153-155
  and historical g03/g04 evidence.
- Gemini CLI Range Requalification → implemented; removed; standing Contract
  029 lane plus compacted g04 roll-up and qualified-range evidence.
- Grok Build Maintained ACP Range → implemented; removed; compacted g01-g03
  evidence, Contracts 015/029, Research 070/085.
- Pi RPC Session Continuity → implemented with the RPC fresh-only boundary
  retained; removed the backlog stub and the fully promoted resolved triage
  duplicate `docs/triage/2026-08-21-pi-continuity-workarounds.md`. Meaning is
  preserved in `pi.sdk-sidecar` (`crates/swallowtail-adapter-pi/src/sidecar.rs`),
  Contracts 017/019/029, Research 180-181, and the compacted g04 roll-up;
  `pi.rpc` remains fresh-only.
- Python Kimi CLI Headless Route → declined/no current need; removed without a
  placeholder; Research 068 and maintained native Kimi Code routes retain the
  decision evidence.

Repaired before deletion: the two live path links into the backlog tree now
point at the surviving canonical note
(`2026-08-19-route-readiness-facade.md` and
`2026-08-21-new-route-candidates.md` → `2026-08-21-deferred-route-surfaces.md`).
Removed live scaffolding: `effigy.toml` roadmaps_backlog index policy, its QA
task and list entry, and the stale `backlog/**` exclude;
`docs/roadmaps/README.md` backlog index bullet.

Deleted: all ten files under `docs/roadmaps/backlog/` and the directory
itself; no aliases, moved-to stubs, or empty directory remain.

## Current State

`find docs -type d -name backlog -print` returns nothing. No current
executable surface, starter template, agent instruction, or checker requires
a roadmap backlog. The new persistence note is deferred intake with no
ordering or dispatch authority. No product task was created, promoted, or
reordered.

## Validation

- `git diff --check`: clean.
- Backlog directory inventory: empty (above).
- Live `roadmaps/backlog` path inventory: no hits outside retained historical
  surfaces (below) and the new triage note's provenance line.
- `effigy qa:docs`: pass (links, all index checks incl. roadmaps/g05, status
  drift + fixtures, number collision + fixtures, release-version identity,
  next-action, forbidden, consumer front door, guide coverage,
  version-expects).
- `effigy qa:northstar`: pass (spine, agent-contract, readme, docs-front-door,
  headings).

## Retained Historical Exceptions

Unchanged per the retirement procedure (provenance, not live authority, no
current broken link): `docs/roadmaps/archive/*` roll-ups and preservation
manifest, historical `docs/logs/*`, closed handoffs without broken current
links, PAPERCUTS observations, and generic non-roadmap uses of “backlog” in
research, feature-ranking code, and quoted evidence.

## Post-Closeout Correction

Chatterbox's 2026-09-09 follow-up removed residual duplicate-intake wording
from current task, roadmap, contract, generation, long-term-plan, and triage
surfaces. It also repointed the closed Pi handoff from the removed resolved
triage note to Research 181. This corrected the two exceptions that should not
have been retained by the reviewed cleanup; product, frontier, candidate, and
release state did not change.

## Review / Merge

PR #302 (`ns-b7c6d92e-a326-4796-84fe-976777d835e6` → `main`) was
independently accepted at exact head
`52423f055e7579a91b5b005d17662c04bf80c271`; review comment `5603707134`
reported no blocking findings and carried the Northstar `ready_to_merge`
marker. The queue merged it into `main` as
`8ae707d5c367d24c566a8ad26c800cc159b2a44e` and synchronized the integration
checkout at that exact SHA. No validation failure was deferred.

## Unchanged Frontier

Candidate `49c9e3b2`, tree `1a9db127`, and g05.036 are untouched. No tag,
release, consumer pin, candidate, generation rollover, or product scope
change. The exact-SHA `v0.4.4` tag decision remains operator-owned.
