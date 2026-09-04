# 064 Contract 061 Candidate C Breadth Audit

Status: complete; candidate C audit complete; promotable as one exact package tranche; no Rust change
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
- `antigravity.catalogue`: 14 rows (13 features, 1 route audit; 2 emitted, 12 withheld)
- `antigravity.headless`: 18 rows (13 features, 5 controls; 16 emitted, 2 withheld across run and continuation)
- `bedrock.catalogue`: 9 rows (8 features, 1 route audit; 2 emitted, 7 withheld)
- `bedrock.runtime`: 10 rows (8 features, 2 controls; 8 emitted, 2 withheld)
- `cursor-agent.acp`: 13 rows (12 features, 1 route audit; 7 emitted, 6 withheld)
- `cursor-agent.catalogue`: 13 rows (12 features, 1 route audit; 2 emitted, 11 withheld)
- `cursor-agent.headless`: 17 rows (12 features, 5 controls; 14 emitted, 3 withheld)
Total: 78 features, 12 controls, 4 explicit no-control route audits = 94 rows
(51 emitted, 43 withheld).

The four no-control audits (`antigravity.catalogue`, `bedrock.catalogue`,
`cursor-agent.acp`, `cursor-agent.catalogue`) carry
`audit.no-public-route-specific-selectable-control` as negative coverage.

### Facades And Source Identity Truth

Eight prepared facades exist on current `main`:
- `AntigravityPreparedCatalogue`: `crates/swallowtail-adapter-antigravity/src/prepared/catalogue.rs:37-43` (`StructuredRun` / `ModelCatalog`)
- `AntigravityPreparedHeadlessRun`: `crates/swallowtail-adapter-antigravity/src/prepared/run.rs:98-104` (`StructuredRun` / `StructuredRun`)
- `AntigravityPreparedContinuation`: `crates/swallowtail-adapter-antigravity/src/prepared/session.rs:48-52` (`InteractiveSession` / `InteractiveSession`)
- `BedrockPreparedCatalogue`: `crates/swallowtail-adapter-bedrock/src/prepared/catalogue.rs:186-192` (`StructuredRun` / `ModelCatalog`)
- `BedrockPreparedInferenceAttempt`: `crates/swallowtail-adapter-bedrock/src/prepared/runtime.rs:234-240` (`StructuredRun` / `StructuredRun`)
- `CursorPreparedAcpSession`: `crates/swallowtail-adapter-cursor/src/prepared/acp.rs:33-38` (`InteractiveSession` / `InteractiveSession`)
- `CursorPreparedCatalogue`: `crates/swallowtail-adapter-cursor/src/prepared/catalogue.rs:38-44` (`StructuredRun` / `ModelCatalog`)
- `CursorPreparedHeadlessRun`: `crates/swallowtail-adapter-cursor/src/prepared/headless.rs:144-151` (`StructuredRun` / `StructuredRun`)

On `antigravity.headless`, `AntigravityPreparedHeadlessRun` and
`AntigravityPreparedContinuation` jointly cover the route's structured-run and
interactive continuation capabilities without cross-route leakage.

**Source-identity truth:** Code search over `crates/swallowtail-adapter-antigravity`,
`crates/swallowtail-adapter-bedrock`, and `crates/swallowtail-adapter-cursor`
confirms zero occurrences of `consumer_route_projection_contribution`,
`AdapterContribution`, or `ConsumerRouteProjectionSourceKind`. None of the
three crates currently implements projection contribution on `main`. The
intended established kind for implementation is
`ConsumerRouteProjectionSourceKind::AdapterContribution`, matching completed
packages (`QoderHeadlessPreparedRun`, etc.).

**Active-observation absence proof:** Resolving line references confirm active
observation is absent across all seven routes:
- `antigravity.catalogue`: `AntigravityPreparedCatalogue::list_models` (`catalogue.rs:110-118`)
- `antigravity.headless`: `AntigravityPreparedHeadlessRun::start_run` (`run.rs:226-234`); `AntigravityPreparedContinuation::open_session` (`session.rs:147-156`)
- `bedrock.catalogue`: `BedrockPreparedCatalogue::list_models` (`catalogue.rs:220-228`)
- `bedrock.runtime`: `BedrockPreparedInferenceAttempt::start_run` (`runtime.rs:268-276`)
- `cursor-agent.acp`: `CursorPreparedAcpSession::open_session` (`acp.rs:123-132`)
- `cursor-agent.catalogue`: `CursorPreparedCatalogue::list_models` (`catalogue.rs:114-122`)
- `cursor-agent.headless`: `CursorPreparedHeadlessRun::start_run` (`headless.rs:282-290`)

### Non-Uniform Controls And Withholding Rules

The 12 controls are non-uniform across value domain, omission, and applicability:
- `control.model-selection`: `exact-model-route` domain, required in route selection
- `control.reasoning-selection`: `bounded-enum` (`low|medium|high`), optional effort
- `control.maximum-output-tokens`: `bounded-integer` (`1..=i32::MAX`), required in Bedrock runtime profile
- `control.structured-output`: `structured-declaration`, optional schema
- `control.resource-access`: `access-policy`, required by Antigravity run profile
- `control.isolation`: `isolation-policy`, required by Antigravity run profile
- `control.fast`, `control.context-window`, `control.reasoning-effort`, `control.read-mode`: `bounded-enum` parameters on Cursor headless

Withholding rules enforce construction-time omission for incompatible
operation shapes, documentation-only matrix rows, negative-coverage route
audits, and activity observation on catalogue routes.

### View Occupancy Reconciled With Emit Set

Fixed library maxima are respected across all views:
- `SelectionSummary`: max 8 emitted rows per contribution (limit 32)
- `SessionStart`: max 4 controls per contribution (limit 16)
- `ActiveSession`: exactly 1 descriptor-only row (`feature.activity-observation`) for operations with an activity profile, 0 for catalogue operations (limit 8)
- `NamespacedExtensions`: max 4 on Cursor headless (limit 16)
- `SourceIdentities`: 1 per contribution (limit 16)

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
3. Public baseline stability and reconciled view occupancy: PASS
4. Deterministic 94-row emit/withhold ledgers (51 emitted, 43 withheld): PASS
5. Package boundary and focused validation (3 packages <= 4): PASS
6. Reviewable single-tranche scope: PASS

### Recommended Disposition

**Promotable as one exact package tranche** (`swallowtail-adapter-antigravity`,
`swallowtail-adapter-bedrock`, `swallowtail-adapter-cursor`).

Validation passes: `effigy qa:docs`, `effigy qa:northstar`, `git diff --check`.
Zero Rust changes.

## Evidence

- triage note `docs/triage/20260904-134500-contract-061-candidate-c-audit.md`, pruned after card 069 closed; preserved in Git history
- [Contract 061](../../../contracts/061-consumer-route-feature-and-control-projection.md)
- [reviewed census](../../../triage/2026-08-30-consumer-route-feature-and-option-projection-census.csv)
- [Batch 9.4 package expansion](../../../triage/2026-08-31-contract-061-batch-9-4-package-expansion.md)
- [observation deferral log](../../../logs/2026-09-04-contract-061-observation-deferral-and-breadth-audits.md)
