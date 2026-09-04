# 067 Contract 061 Candidate J Breadth Audit

Status: complete
Owner: Tom
Created: 2026-09-04
Updated: 2026-09-04
Milestone: `../009-contract-061-consumer-projection-realization.md`
Depends on: Contract 061; Batch 9.4 checkpoint; completed cards 022-024 and 031-033; current `main`

## Goal

Audit Batch 9.4 candidate J (llama.cpp, Ollama) against current `main` under the
promotion rubric and return one honest disposition: promotable as one exact
package tranche, or stopped with the named blocker. No Rust changes.

## Candidate

Routes and census rows: `llama-cpp.attached` 10; `llama-cpp.owned` 6; `ollama.attached` 19. Total 35 rows. The Batch 9.4 note classed
this candidate viable later because it has fewer rows than candidate H but more route-specific controls and
attached/owned prepared families whose applicability must stay distinct.

## Scope

1. Reconcile the exact census set to 35 rows without a filter or exception
   list, including explicit no-control rows as negative coverage.
2. Name every prepared or active-observation facade and its source-identity
   kind on current `main`, or prove its exact absence with a code reference.
3. Trace lifecycle, authority, applicability, and evidence-strength truth for
   every row. Documentation-only or prepared success is not observation.
4. State construction-time withholding rules for catalogue-only,
   incompatible-operation, documentation-only, and unobserved rows.
5. Answer rubric items 1-6 individually with evidence. Do not weaken a
   blocker to force a selection.
6. Where the candidate needs a new shared public type, fixed maximum,
   composer rule, or contract amendment, record the exact gap and stop; do
   not design it.
7. Write exactly one new triage note
   `docs/triage/YYYYMMDD-HHMMSS-contract-061-candidate-j-audit.md`
   holding the ledgers, facade map, withholding rules, rubric verdict, and
   one recommended disposition. Fill this card's `## Result`.

## Out Of Scope

Rust, manifest, contract, architecture, census, or Batch 9.4 note edits;
other candidates; provider contact; implementing the tranche; compiling an
implementation card (Chatterbox promotes from the note).

## Acceptance Criteria

- rows reconcile to 35 exactly
- every facade and source identity is named or its absence proved
- rubric items 1-6 each carry a verdict and evidence
- one triage note exists and this card's result names its path
- zero Rust changes

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Review Oracle

Invariant: the note is planning evidence, never coverage or authority.
Smallest counterexample: a row counted as proved, a facade asserted without a
code reference, or a rubric item closed by omission.

## Auto-Continuation

No. Stop after the note for coordinator closeout and Chatterbox reconciliation.

## Result

Candidate J (llama.cpp, Ollama) is promotable as one exact two-package
tranche. The audit reconciles all 35 census rows — `llama-cpp.attached` 10,
`llama-cpp.owned` 6, `ollama.attached` 19 — as 32 emitted and 3 withheld
construction-time negative-coverage rows, with every facade named and every
withholding absence code-proved on current `main` at `bab2183932`. No
blocker, no public-baseline gate, and no stop: no new shared public type,
fixed maximum, composer rule, or contract amendment is needed. The three
withheld rows are `llama-cpp.attached` `feature.cancellation-or-interruption`
(docs-only; no attached plan requires `Capability::Interruption`),
`llama-cpp.owned` `feature.activity-observation` (absence-proved; owned
prepared evidence retains no activity profile), and `ollama.attached`
interactive-session `control.reasoning-selection` (docs-only; no current
public session input exposes reasoning selection). The audit note records the
ledgers, facade map, withholding rules, shape-scope findings, and rubric
verdicts 1-6 all passing.

- Triage note: `docs/triage/20260904-134914-contract-061-candidate-j-audit.md`
- Validation: `effigy qa:docs` and `effigy qa:northstar` pass; `git diff
  --check` clean
- Changed files: exactly the triage note and this card's Result
- Zero Rust changes

This card's Status line and the batch-card index stay coordinator-owned
closeout surfaces; the coordinator re-buckets them after review and merge.
Chatterbox reconciles the note and may promote at most one implementation
card for the exact 35-row llama.cpp/Ollama tranche.
