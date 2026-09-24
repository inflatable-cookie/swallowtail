# g06.020 Grok Build ACP 1.0.40 Review-Entry Pilot

Owner: Tom
Created: 2026-09-24
Depends on: Contract 029; Research 314, 316; external PR #358
Vision tags: route currentness, Grok Build ACP, external PR, review entry

## Outcome

Admit external PR #358 through Queue review entry as the bounded pilot for the
second external suite (#354–#359), raising the `grok-build.acp`
`AllowUnverified` executable window from `1.0.30` toward current official.

## Why It Matters

Six Grok Bot currentness PRs have sat open since 2026-09-21. Tom approved a
pilot-first sequence on 2026-09-24. Grok Build ACP goes first because it sits
closest to the MCP work: its registered-tool courier and Longhorn carrier path
must stay untouched by a window raise.

## Ready-State Rubric

- [x] PR #358 is open, non-draft, same-origin, and green on every CI job at
      head `2eda6ab0`.
- [x] Integration collisions are known: the PR names its record Research 331,
      which `main` already uses, and it conflicts in the research index, logs
      index, and feature matrix CSV.
- [x] Official moved after the PR: npm `latest` is `1.0.41` at planning.
- [x] Scope, acceptance, validation, evidence and stop conditions are explicit.

## Decisions

- Renumber the PR's research record to the next free number and repair every
  reference to it. Resolve the three index and matrix conflicts against
  current `main` without dropping any `main` row.
- Extend to current official (`1.0.41` at planning) when that hop is a
  compatible extension. A selected-surface change is recorded exactly and
  handled under Contract 029 No Terminal Stop: the PR lands the compatible
  prefix and Chatterbox compiles the adaptation.
- Keep the exact `1.0.30` catalogue pin, the `1.0.4`/`1.0.5` registered-tool
  courier, claim id `grok-build.acp.executable-window-2`, and the behavior
  revision unless identity proves a change.
- Author declaration: Tom attested on 2026-09-24 that Grok 4.7 wrote #358,
  recorded as `xai/grok-4.7`; GitHub `betterthanclay`; Cursor cloud agent per
  the PR footer. It is operator-declared, not runtime-observed, and does not
  extend to #354–#357 or #359.

## Dispatch manifest

- **State:** ready; review-entry pilot; reviewer first, no initial worker.
- **Completion:** independent exact-head review accepts the head with the
  research number, indexes and matrix reconciled against current `main`.
- **Owned mutable paths on revision:** the PR's existing changed paths; the
  renumbered research record; index and matrix conflict resolution; fixtures
  and identity tests for any added hop.
- **Reserved closeout surfaces:** lifecycle task record and generated
  projections; submitted-handoff deletion belongs to the repository hook.
- **Excluded:** this card's prose; the catalogue pin and registered-tool
  courier; other PRs in the suite; contracts; Next Task and generation
  runway; release, tag, publication; any login, install, binary execution or
  live session.
- **Worker evidence:** the authenticated run result and the research record.
- **Escalation:** operator via Chatterbox for scope; Queue coordinator for
  mechanical blockers.

## Work

1. Reviewer: verify identity, the compatible-extension claim, and the
   collision and conflict repairs listed above.
2. Revision worker, if requested: renumber, reconcile, extend to current
   official if compatible, and rerun validation.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| No record collides | Two research files share a number | unique number; every reference updated |
| Indexes lose nothing | A `main` index or matrix row disappears in conflict resolution | diff against `main` shows additions only |
| Courier and catalogue stay closed | A `1.0.4`/`1.0.5` or catalogue pin moves | no such change |
| Extension is proven | A hop is claimed from a version bump | mapped-surface evidence per hop |

## Stop conditions

Stop and ask if the renumber or conflicts touch another PR's scope, or if
identity shows a change needing a new driver or facade revision.

## Evidence

The PR's research record and fixtures. Focused and affected-package validation
for `swallowtail-adapter-grok`, route and docs QA.

## Next Task

Chatterbox reconciles the pilot, then sequences #354–#357 and #359 on a fresh
base with any author-attestation question put to Tom.
