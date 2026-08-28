# g05.002 Effective Harness Skill Visibility Proof

Status: ready
Owner: Tom
Created: 2026-08-28
Depends on: completed g05.001; Contract 058
Vision tags: harness skills, selected-session truth, consumer integration
Contract refs: 020, 029, 032, 033, 037, 041, 047, 058
Research: 256 reserved
Planning state: card 004 ready; cards 005-006 planned

## Problem

Contract 058 requires the complete effective skill roster seen by one exact
selected harness context. Research 255 found no qualified route. Qoder
headless `1.1.25` is the only current lead with explicit `skills` and
`plugins` collections alongside run, model, and session identity, but only
empty prompt-bearing fixtures exist.

## Goal

Decide whether Qoder supplies a prompt-free, complete, positive-or-empty
effective roster including deliberately installed global and project skills.
Bind it only if exact evidence closes every Contract 058 gate.

## Execution Plan

### Batch 2.1 — Exact Evidence

- [ ] execute ready card 004
- [ ] freeze init timing, roster source, completeness, provenance, freshness,
      and prompt/auth/mutation behavior
- [ ] return a closed positive row or honest empty set in Research 256

### Batch 2.2 — Conditional Binding

- [ ] execute card 005 only after a non-empty deliver-now disposition
- [ ] expose the bounded observation without widening the Qoder run
- [ ] execute card 006 for route, guide, and consumer acceptance

## Acceptance Criteria

- [ ] global and project skills are included when Qoder actually admits them
- [ ] file presence and package membership never substitute for run visibility
- [ ] no model prompt, paid work, ambient scan, or host mutation is required
- [ ] empty, unavailable, incomplete, and complete rosters remain distinct
- [ ] no production capability lands after an honest empty set

## Stop Conditions

- the roster exists only after model inference starts
- positive membership requires login, paid work, or ambient host mutation
- `skills` or `plugins` is partial, lazy, unbounded, or not selected-run truth
- a project or global row requires Swallowtail or adapter scanning instead of
  the exact harness roster surface
- exact `1.1.25` source cannot bind the field semantics

## Batch Cards

- [004 Qoder Effective Skill Roster Evidence](batch-cards/004-qoder-effective-skill-roster-evidence.md)
- [005 Qoder Effective Skill Visibility Binding](batch-cards/005-qoder-effective-skill-visibility-binding.md)
- [006 Qoder Skill Visibility Acceptance](batch-cards/006-qoder-skill-visibility-acceptance.md)

## References

- [Contract 058 Effective Harness Skill Visibility](../../contracts/058-effective-harness-skill-visibility.md)
- [Research 255 Production Harness Census](../../research/255-production-harness-skill-and-watcher-surface-census.md)
- [Research 256 Qoder Effective Skill Roster](../../research/256-qoder-effective-skill-roster-evidence.md)
