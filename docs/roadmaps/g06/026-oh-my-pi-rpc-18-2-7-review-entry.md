# g06.026 Oh My Pi RPC 18.2.7 Review Entry

Owner: Tom
Created: 2026-09-24
Depends on: Contract 029; completed g06.020 pilot; external PR #357
Vision tags: route currentness, external PR, review entry

## Outcome

Admit external PR #357 through Queue review entry, raising `oh-my-pi.rpc` from
`18.1.22` toward current official (`18.2.7` at the PR).

## Why It Matters

The g06.020 pilot (#358) landed cleanly, so Tom approved the rest of the Grok
Bot suite on 2026-09-24. This PR has been open since 2026-09-21.

## Ready-State Rubric

- [x] PR #357 is open, non-draft and same-origin at head `bf1def43`.
- [x] Integration collisions are known: the PR names its record Research 331,
      which `main` already uses, and it conflicts with `main` in shared
      indexes, `CHANGELOG.md` and matrices.
- [x] Scope, acceptance, validation, evidence and stop conditions are explicit.

## Decisions

- Renumber the PR's research record to **Research 345**, reserved for this
  PR so parallel suite lanes cannot collide, and repair every reference.
- Planning integration merges `main` into the branch and resolves conflicts
  additively. Any later conflict from a sibling suite PR landing first is
  resolved the same way, keeping every `main` row.
- Extend to current official when each added hop is a compatible extension.
  A selected-surface change is recorded exactly and lands as the compatible
  prefix; Contract 029 No Terminal Stop then makes Chatterbox compile the
  adaptation.
- Author declaration: Tom attested on 2026-09-24 that every Grok Bot suite PR
  was written by Grok 4.6 or Grok 4.7, recorded as the model set
  `xai/grok-4.6`, `xai/grok-4.7`; GitHub `betterthanclay`; Cursor cloud agent.
  Operator-declared, not runtime-observed.

## Dispatch manifest

- **State:** ready; review entry; reviewer first, no initial worker.
- **Completion:** independent exact-head review accepts the head with Research
  345, indexes and matrices reconciled against current `main`.
- **Owned mutable paths on revision:** the PR's existing changed paths; the
  renumbered research record; conflict resolution in shared indexes,
  `CHANGELOG.md` and matrices; fixtures and identity tests for any added hop.
- **Reserved closeout surfaces:** lifecycle task record and generated
  projections; submitted-handoff deletion belongs to the repository hook.
- **Excluded:** this card's prose; other families and suite PRs; contracts;
  Next Task and generation runway; release, tag, publication; any login,
  install, binary execution or live session.
- **Worker evidence:** the authenticated run result and the research record.
- **Escalation:** operator via Chatterbox for scope; Queue coordinator for
  mechanical blockers.

## Work

1. Reviewer: verify identity, the compatible-extension claim, the renumber,
   and the conflict resolution.
2. Revision worker, if requested: renumber, reconcile, extend to current
   official if compatible, and rerun validation.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| No record collides | The record keeps 331 or takes another lane's number | Research 345; every reference updated |
| Indexes lose nothing | A `main` row disappears in conflict resolution | diff against `main` shows additions only |
| Extension is proven | A hop is claimed from a version bump | mapped-surface evidence per hop |
| Family stays separate | Another family's claim moves | no other adapter in the diff |

## Stop conditions

Stop and ask if the renumber or conflicts touch another PR's scope, or if
identity shows a change needing a new driver or facade revision.

## Evidence

The PR's research record and fixtures. Focused and affected-package validation
for `swallowtail-adapter-oh-my-pi`, route and docs QA.

## Next Task

Chatterbox reconciles the claim after closeout.
