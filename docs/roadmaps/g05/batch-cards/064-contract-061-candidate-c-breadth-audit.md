# 064 Contract 061 Candidate C Breadth Audit

Status: ready; candidate C audit complete; promotable as one exact package tranche; no Rust change
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

- [x] rows reconcile to 94 exactly
- [x] every facade and source identity is named or its absence proved
- [x] rubric items 1-6 each carry a verdict and evidence
- [x] one triage note exists and this card's result names its path
- [x] zero Rust changes

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

Candidate C breadth audit is complete. Exactly one triage note exists:
`docs/triage/20260904-134500-contract-061-candidate-c-audit.md`.

### Census Reconciliation

All 94 rows reconcile exactly across three adapter packages and seven route
shapes with zero exception lists or filters:
- `antigravity.catalogue`: 14 rows (13 features, 1 route audit)
- `antigravity.headless`: 18 rows (13 features, 5 controls)
- `bedrock.catalogue`: 9 rows (8 features, 1 route audit)
- `bedrock.runtime`: 10 rows (8 features, 2 controls)
- `cursor-agent.acp`: 13 rows (12 features, 1 route audit)
- `cursor-agent.catalogue`: 13 rows (12 features, 1 route audit)
- `cursor-agent.headless`: 17 rows (12 features, 5 controls)
Total: 78 features, 12 controls, 4 explicit no-control route audits = 94 rows.

The four no-control audits (`antigravity.catalogue`, `bedrock.catalogue`,
`cursor-agent.acp`, `cursor-agent.catalogue`) carry
`audit.no-public-route-specific-selectable-control` as negative coverage.

### Facades And Source Identity

All seven prepared facades exist on current `main`:
- `AntigravityPreparedCatalogue`: `crates/swallowtail-adapter-antigravity/src/prepared/catalogue.rs:37-43`
- `AntigravityPreparedHeadlessRun`: `crates/swallowtail-adapter-antigravity/src/prepared/run.rs:98-104`
- `BedrockPreparedCatalogue`: `crates/swallowtail-adapter-bedrock/src/prepared/catalogue.rs:186-192`
- `BedrockPreparedInferenceAttempt`: `crates/swallowtail-adapter-bedrock/src/prepared/runtime.rs:234-240`
- `CursorPreparedAcpSession`: `crates/swallowtail-adapter-cursor/src/prepared/acp.rs:33-38`
- `CursorPreparedCatalogue`: `crates/swallowtail-adapter-cursor/src/prepared/catalogue.rs:38-44`
- `CursorPreparedHeadlessRun`: `crates/swallowtail-adapter-cursor/src/prepared/headless.rs:144-151`

All use caller-supplied `ConsumerRouteProjectionSourceKind::AdapterContribution`.
Active-observation facades are proven absent on all seven routes; Candidate C
carries zero active post-open provider observation rows.

### Construction-Time Withholding Rules

Withholding rules are defined at construction for incompatible operation
shapes, documentation-only matrix features, negative-coverage route audits,
and activity observation on catalogue routes without activity profiles.

### Section 6a: Kimi Decision Reopen Finding

No row on `antigravity.catalogue`, `bedrock.catalogue`, or
`cursor-agent.catalogue` needs a provider-operation observation source kind,
lifecycle band, or view. The reopen trigger for the deferred Kimi decision is
not activated; candidate F remains unpromoted and card 034 remains planned and
not ready.

### Promotion Rubric Verdict

All six rubric items pass:
1. Exact census reconciliation and no-control negative coverage: PASS
2. Facade and source identity map plus withholding rules: PASS
3. Public baseline stability (no new runtime/core types, bounds respected): PASS
4. Deterministic adapter-local ledgers: PASS
5. Package boundary and focused validation (3 packages <= 4): PASS
6. Reviewable single-tranche scope: PASS

### Recommended Disposition

**Promotable as one exact package tranche** (`swallowtail-adapter-antigravity`,
`swallowtail-adapter-bedrock`, `swallowtail-adapter-cursor`).

Validation passes: `effigy qa:docs`, `effigy qa:northstar`, `git diff --check`.
Zero Rust changes.

## Evidence

- [triage note](../../../triage/20260904-134500-contract-061-candidate-c-audit.md)
- [Contract 061](../../../contracts/061-consumer-route-feature-and-control-projection.md)
- [reviewed census](../../../triage/2026-08-30-consumer-route-feature-and-option-projection-census.csv)
- [Batch 9.4 package expansion](../../../triage/2026-08-31-contract-061-batch-9-4-package-expansion.md)
- [observation deferral log](../../../logs/2026-09-04-contract-061-observation-deferral-and-breadth-audits.md)
