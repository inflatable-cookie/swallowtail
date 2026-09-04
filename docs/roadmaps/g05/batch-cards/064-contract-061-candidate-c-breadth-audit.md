# 064 Contract 061 Candidate C Breadth Audit

Status: ready
Owner: Tom
Created: 2026-09-04
Updated: 2026-09-04
Milestone: `../009-contract-061-consumer-projection-realization.md`
Depends on: Contract 061; Batch 9.4 checkpoint; completed cards 022-024 and 031-033; current `main`

## Goal

Audit Batch 9.4 candidate C (Antigravity, Bedrock, Cursor) against current `main` under the
promotion rubric and return one honest disposition: promotable as one exact
package tranche, or stopped with the named blocker. No Rust changes.

## Candidate

Routes and census rows: `antigravity.catalogue` 14; `antigravity.headless` 18; `bedrock.catalogue` 9; `bedrock.runtime` 10; `cursor-agent.acp` 13; `cursor-agent.catalogue` 13; `cursor-agent.headless` 17. Total 94 rows. The Batch 9.4 note classed
this candidate viable later because its 94 rows and seven route shapes form a larger negative-coverage tranche
than candidate H; four explicit no-control route audits are expected.

## Scope

1. Reconcile the exact census set to 94 rows without a filter or exception
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
6a. Report explicitly whether any row on `antigravity.catalogue`,
   `bedrock.catalogue`, or `cursor-agent.catalogue` needs a
   provider-operation observation source kind, lifecycle band, or view that
   current `swallowtail-runtime` defines only as post-open session
   semantics. This finding is the reopen trigger for the deferred Kimi
   decision recorded in the Kimi active-observation gate note; state it in
   its own section.
7. Write exactly one new triage note
   `docs/triage/YYYYMMDD-HHMMSS-contract-061-candidate-c-audit.md`
   holding the ledgers, facade map, withholding rules, rubric verdict, and
   one recommended disposition. Fill this card's `## Result`.

## Out Of Scope

Rust, manifest, contract, architecture, census, or Batch 9.4 note edits;
other candidates; provider contact; implementing the tranche; compiling an
implementation card (Chatterbox promotes from the note).

## Acceptance Criteria

- rows reconcile to 94 exactly
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
