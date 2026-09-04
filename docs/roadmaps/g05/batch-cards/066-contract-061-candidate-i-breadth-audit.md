# 066 Contract 061 Candidate I Breadth Audit

Status: complete; evidence stop; two provider-operation rows require the deferred shared observation decision
Owner: Tom
Created: 2026-09-04
Updated: 2026-09-04
Milestone: `../009-contract-061-consumer-projection-realization.md`
Depends on: Contract 061; Batch 9.4 checkpoint; completed cards 022-024 and 031-033; current `main`

## Goal

Audit Batch 9.4 candidate I (DeepSeek, DeepSeek Harness) against current `main` under the
promotion rubric and return one honest disposition: promotable as one exact
package tranche, or stopped with the named blocker. No Rust changes.

## Candidate

Routes and census rows: `deepseek.continuation` 19; `deepseek-harness.jsonrpc` 11; `deepseek-harness.local-server` 17. Total 47 rows. The Batch 9.4 note classed
this candidate viable later because continuation, JSON-RPC, and local-server lifecycle need a distinct
multi-facade audit.

## Scope

1. Reconcile the exact census set to 47 rows without a filter or exception
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
   `docs/triage/YYYYMMDD-HHMMSS-contract-061-candidate-i-audit.md`
   holding the ledgers, facade map, withholding rules, rubric verdict, and
   one recommended disposition. Fill this card's `## Result`.

## Out Of Scope

Rust, manifest, contract, architecture, census, or Batch 9.4 note edits;
other candidates; provider contact; implementing the tranche; compiling an
implementation card (Chatterbox promotes from the note).

## Acceptance Criteria

- rows reconcile to 47 exactly
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

Stopped. Candidate I is not promotable on current `main` (`bab21839`).
The audit note is
[`docs/triage/20260904-140002-contract-061-candidate-i-audit.md`](../../../triage/20260904-140002-contract-061-candidate-i-audit.md).

- Rows reconcile to exactly 47 (19 `deepseek.continuation`, 11
  `deepseek-harness.jsonrpc`, 17 `deepseek-harness.local-server`) with no
  filter or exception list; candidate I owns zero of the nine no-control
  audit rows.
- Facade map: every census-cited prepared facade is named on current `main`
  with caller-supplied source identity (five on `deepseek.continuation`
  including `DeepSeekPreparedCatalogue`, two on `deepseek-harness.jsonrpc`,
  seven on `deepseek-harness.local-server`); neither adapter emits any
  `AdapterContribution` today.
- Ledger: 39 rows named against exact prepared or observed sources, 6
  matrix-posture rows construction-time withheld (01, 07, 08, 11, 19, 20),
  2 rows blocked.
- Blocker: census rows 44 `control.provider-session-catalogue` and 45
  `control.provider-session-history` are `post-open-observation-only` with
  observed state, but their sources are prepared provider-operation queries
  that open no session (`web/driver.rs:385-555`, `:558-676`). Publishing
  them needs `ActiveSessionObservation`, `PostOpenObservationOnly`, or
  `ConsumerRouteActiveSessionState` to widen from session-scoped semantics —
  the same stop-and-record gap as the Kimi gate, which pre-adjudicates the
  substitution. Rubric items 2 and 3 fail; no shared public type, fixed
  maximum, or composer rule is designed here.
- Census-source corrections (adapter lib.rs export claims, `ActivityEvent`,
  `ModelRoute` location, rows 46/47 effective outcomes) are recorded in the
  note as evidence only; the census CSV is untouched.
- Zero Rust changes. Changed files: this card and the one new triage note.
