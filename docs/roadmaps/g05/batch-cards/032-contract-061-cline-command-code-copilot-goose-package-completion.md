# 032 Contract 061 Cline, Command Code, Copilot CLI, And Goose Package Completion

Status: ready
Owner: Tom
Created: 2026-09-01
Updated: 2026-09-01
Milestone: `../009-contract-061-consumer-projection-realization.md`
Depends on: completed card 031; accepted Cline active-observation
public-baseline gate

## Goal

Complete candidate G's exact 48-row Contract 061 package remainder and expose
exact `cline.acp` Plan acknowledgement plus bounded negotiated model options
through the accepted additive adapter-owned projected-open seam.

## Scope

1. Add the established
   `consumer_route_projection_contribution(source_id)` method to
   `ClinePreparedSession`, `ClineHeadlessPreparedRun`,
   `CommandCodePreparedRun`, `CommandCodePreparedSession`,
   `CopilotCliPreparedSession`, and `GoosePreparedSession`.
2. Add `ClineProjectionOpenFuture`, `ClineProjectionOpenOutcome`, and
   `ClineProjectionOpenFailure` with the exact signatures and accessors fixed
   by the
   [public-baseline gate](../../../triage/2026-09-01-contract-061-cline-active-observation-public-baseline-gate.md).
3. Add `ClinePreparedSession::open_session_with_projection` with distinct
   prepared and active-session source IDs. Keep
   `ClinePreparedSession::open_session` source-compatible with the same
   failure and cleanup lifecycle. Its existing generic handle accessor may
   expose only the newly approved exact model snapshot. Serve both methods
   from one private open lifecycle.
4. Retain exact Cline Plan confirmation privately: omitted, effective
   `"plan"`, exact rejected `"act"`, or runtime failure. Return `Rejected`
   only for the exact unique `["plan", "act"]` mismatch; preserve the existing
   diagnostic and cleanup in both public open paths.
5. Parse an optional exact Cline `model` config option into
   `NegotiatedSessionModelOptions`, retain it on the generic session handle,
   and expose the same snapshot from the projected-open outcome. Absent model
   evidence remains `None`. Invalid evidence remains `None` on the preserved
   open path, while projected open closes the session and returns the exact
   negotiated-model-options runtime failure with no contribution.
6. Publish Cline post-open Plan and model-option rows only through bounded
   namespaced identities qualified by exact `cline.acp` route and
   protocol-facade revision. Keep model observation non-selectable, observed,
   and without mutation or catalogue authority.
7. Disposition exactly 48 census tuples with no filter or exception list: 11
   `cline.acp`, 8 `cline.headless`, 11 `command-code.headless`, 9
   `copilot-cli.acp`, and 9 `goose.acp`. Name each tuple once with an emitted
   or construction-time-withheld reason.
8. Prove these maximal ledger totals independently: Cline ACP 9/2,
   Cline headless 7/1, Command Code 10/1, Copilot CLI 6/3, and Goose 6/3
   emitted/withheld.
9. Withhold model-catalogue on all five routes. Withhold persistence on Cline
   ACP, Copilot CLI, and Goose from their exact `Prohibited` prepared policy.
   Keep the Copilot CLI and Goose no-control audits as negative coverage and
   emit no out-of-census Command Code capability.
10. Keep prepared activity descriptor-only. Preserve Cline ACP and headless
    harness-mode lifecycle separately. Preserve Command Code structured-run
    and interactive-session model-selection applicability separately.
11. Add deterministic provider-free fixtures for Cline omitted, effective,
    rejected, missing, malformed, duplicate, absent-model, exact-model,
    malformed-model, and equal-source cases. Prove the preserved and additive
    open paths share Plan failure codes and cleanup.
12. Add independent ledger, route/operation/access/instance/revision mixture,
    support, availability, lifecycle, omission, posture, and no-control proofs
    for all five routes. Stop after one reviewable four-package PR.

## Out Of Scope

- `kimi-code.acp`, candidate F, or `EffectiveReasoningSetup`
- candidates B, C, E, I-L or any Batch 9.5 work
- runtime/testkit/core public API, contracts, census, compatibility, or route
  claim changes
- model mutation, model catalogue discovery, session creation for discovery,
  generic active-observation payloads, callbacks, registries, runtime route
  enumeration, or adapter downcasts
- provider contact, live probes, currentness, watcher, skill-discovery,
  papercut, or generation-closeout work

## Acceptance Criteria

- [ ] independent ledgers reconcile exactly to 11, 8, 11, 9, and 9 rows,
      with each `(route_id, operation_shape, semantic_id)` named once and no
      exception list
- [ ] maximal dispositions equal 9/2, 7/1, 10/1, 6/3, and 6/3
      emitted/withheld; minimal and maximal prepared/open fixtures prove every
      optional Cline row is genuinely absent or present
- [ ] every emitted row retains exact source, route, operation, lifecycle,
      value, omission, applicability, access, evidence, support, availability,
      actor posture, and mutation posture
- [ ] prepared Cline Plan is requested/prepared/pending on ACP and
      requested/prepared on headless; neither prepared facade claims
      provider-effective or rejected state
- [ ] exact matching ACP Plan confirmation opens a session and emits
      provider-effective `"plan"`; exact admitted `"act"` returns `Rejected`
      with no session
- [ ] missing, malformed, duplicate, transport, setup, or ambiguous Plan
      confirmation returns `Runtime` with no rejected contribution
- [ ] exact model options survive on both the generic handle and outcome
      accessor; absent data stays absent; invalid data preserves legacy-open
      success with no snapshot and fails projected open after cleanup with
      `swallowtail.negotiated_model_options.invalid`
- [ ] model-option projection is observed, not selectable, acknowledged,
      provider-effective, mutable, or catalogue authority
- [ ] rejected Plan never carries a model-option row; omitted Plan carries no
      acknowledgement row; an unused active source is absent
- [ ] existing and projected Cline open methods preserve Plan failure code,
      handle shape, and cleanup outcome through one private lifecycle
- [ ] model catalogue, prohibited persistence, no-control audits, and any
      out-of-census capability are withheld at construction
- [ ] Command Code model selection is exact for both structured-run and
      interactive-session facades without flattening their applicability
- [ ] matching-source cross-route and cross-operation mixtures reject in both
      directions; cross-instance, stale-revision, and each exact access drift
      fail closed or stop before a row can form
- [ ] only the four candidate G adapter semantic API baselines change; shared
      APIs, contracts, census, provider claims, and compatibility claims remain
      unchanged
- [ ] touched source stays below configured god-file thresholds and the scan
      does not exceed the accepted 387-finding repository baseline

## Review Oracle

Invariant: candidate G publishes 48 exact route tuples, while only an exact
Cline open observation may add Plan acknowledgement or model-option state.

Counterexamples and required proof:

- mark prepared Cline Plan provider-effective — fail lifecycle/state proof
- treat a static mismatch diagnostic as exact rejected `"act"` without the
  complete unique provider domain — fail as runtime with no contribution
- infer model options from session existence or matrix posture — fail; exact
  wire evidence is required
- accept malformed, duplicate, ambiguous, unadvertised, or unbounded model
  config on projected open — fail with the exact runtime code after cleanup;
  the preserved path stays successful with no snapshot
- attach selectability, mutation authority, catalogue identity, or
  provider-effective state to model observation — fail exact posture proof
- retain model observation on rejected Plan — fail; no open session exists
- reuse the prepared source ID for active observation — fail before provider
  work
- attach Cline active rows to headless, Command Code, Copilot CLI, or Goose —
  fail route/operation applicability
- emit persistence from documentation on a `Prohibited` prepared session —
  fail exact provider-state policy proof
- turn a Copilot CLI or Goose no-control audit into a row — fail negative
  coverage
- copy Command Code structured-run model selection onto its session, or the
  inverse — fail exact operation applicability
- make preserved and projected Cline opens disagree on diagnostic or cleanup
  for the same Plan fixture — fail shared-lifecycle proof
- reach 48 through a filtered superset, exception list, duplicated semantic
  identity, or borrowed route identity — fail ledger reconciliation

The ledger source is generated from the reviewed CSV. Observed tuple mapping
is maintained independently of ledger disposition so a wrong route,
operation-shape label, or semantic identity cannot agree with itself.

## Validation

- `cargo fmt -p swallowtail-adapter-cline -p swallowtail-adapter-command-code -p swallowtail-adapter-copilot-cli -p swallowtail-adapter-goose -- --check`
- `effigy validate:focused swallowtail-adapter-cline swallowtail-adapter-command-code swallowtail-adapter-copilot-cli swallowtail-adapter-goose`
- `effigy package:verify-affected swallowtail-adapter-cline swallowtail-adapter-command-code swallowtail-adapter-copilot-cli swallowtail-adapter-goose`
- `effigy package:api`
- `effigy qa:routes`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy --json scan god-files`
- `git diff --check`

No live probe or provider contact belongs to this card.

## Auto-Continuation

No. Stop after one reviewable PR. The orchestrator must review the exact
48-row proof, four adapter API baselines, and Cline failure preservation before
another Batch 9.4 candidate is reassessed or promoted.

## Stop Conditions

- Stop if the card needs a runtime/testkit/core public type, source kind,
  composer rule, fixed maximum, failure kind, or contract amendment.
- Stop if exact Cline Plan rejection requires accepting an unadvertised or
  ambiguous value, or if exact model options require a raw provider payload.
- Stop if optional model observation changes preserved-open success, failure
  code, handle type, or cleanup beyond the approved exact snapshot on its
  existing generic accessor.
- Stop if prepared and active Cline truth cannot remain independently sourced.
- Stop if any 11/8/11/9/9 ledger needs an exception list, inferred support,
  or truth borrowed from another route or operation.
- Stop if scope widens to Kimi, another candidate, Batch 9.5, shared public
  API, provider contact, or another product track.

## Evidence

- [Cline active-observation public-baseline gate](../../../triage/2026-09-01-contract-061-cline-active-observation-public-baseline-gate.md)
- [Batch 9.4 package expansion](../../../triage/2026-08-31-contract-061-batch-9-4-package-expansion.md)
- [completed card 031](031-contract-061-claude-agent-package-and-acknowledgement.md)
- [Contract 061](../../../contracts/061-consumer-route-feature-and-control-projection.md)
- [reviewed census](../../../triage/2026-08-30-consumer-route-feature-and-option-projection-census.csv)
- [Batch 9.1 public baseline](../../../triage/2026-08-31-contract-061-batch-9-1-public-baseline-gate.md)
