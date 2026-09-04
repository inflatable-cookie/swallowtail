# 065 Contract 061 Candidate E Breadth Audit

Status: ready
Owner: Tom
Created: 2026-09-04
Updated: 2026-09-04
Milestone: `../009-contract-061-consumer-projection-realization.md`
Depends on: Contract 061; Batch 9.4 checkpoint; completed cards 022-024 and 031-033; current `main`

## Goal

Audit Batch 9.4 candidate E (Gemini, Grok) against current `main` under the
promotion rubric and return one honest disposition: promotable as one exact
package tranche, or stopped with the named blocker. No Rust changes.

## Candidate

Routes and census rows: `gemini-cli.acp` 14; `gemini-cli.headless` 13; `gemini.live` 16; `grok-build.acp` 13. Total 56 rows. The Batch 9.4 note classed
this candidate viable later because ACP, headless, and live applicability need a three-family proof that keeps
each applicability distinct. Gemini requalification stays deferred; this audit
reads current code and changes no claim.

## Scope

1. Reconcile the exact census set to 56 rows without a filter or exception
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
   `docs/triage/YYYYMMDD-HHMMSS-contract-061-candidate-e-audit.md`
   holding the ledgers, facade map, withholding rules, rubric verdict, and
   one recommended disposition. Fill this card's `## Result`.

## Out Of Scope

Rust, manifest, contract, architecture, census, or Batch 9.4 note edits;
other candidates; provider contact; implementing the tranche; compiling an
implementation card (Chatterbox promotes from the note).

## Acceptance Criteria

- rows reconcile to 56 exactly
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

Audit complete. Candidate E reconciles to exactly 56 rows across 4 route IDs
and 2 adapter packages (`swallowtail-adapter-gemini` and `swallowtail-adapter-grok`)
with zero filters or exceptions. All 5 prepared facades (`GeminiPreparedSession`,
`GeminiHeadlessPreparedRun`, `GeminiPreparedLiveSession`, `GrokPreparedSession`,
`GrokPreparedRun`) are verified with exact code references on current `main`.
Active-observation facades are proved absent on current `main`. Applicability
dimensions across ACP, headless, and live families remain distinct and non-overlapping.
Both ACP drivers already retain `NegotiatedSessionModelOptions` on their session
handles without discarding confirmations. Rubric items 1-6 all pass with evidence.
Recommended disposition: promotable as one exact package tranche with adapter-owned
projected-open seams following Card 032.

Triage note: `docs/triage/20260904-134659-contract-061-candidate-e-audit.md`.
