# 024 Contract 061 Deep Agents, Kiro, Qoder, And ZCode Package Completion

Status: ready
Owner: Tom
Created: 2026-08-31
Updated: 2026-08-31
Milestone: `../009-contract-061-consumer-projection-realization.md`
Depends on: completed card 023; post-card-023 Batch 9.4 reassessment

## Goal

Complete candidate H's Contract 061 census dispositions: all 9
`deepagents.acp`, 9 `kiro.acp`, 8 `qoder.headless`, and 12
`zcode.app-server` rows.

## Scope

1. Add the established
   `consumer_route_projection_contribution(source_id)` shape to
   `DeepAgentsPreparedSession`, `KiroPreparedSession`,
   `QoderHeadlessPreparedRun`, and `ZcodePreparedRun`.
2. Use only each facade's exact `PreparedOperationEvidence`, activity profile,
   request, and adapter-local prepared evidence. Preserve the caller-supplied
   `AdapterContribution` source identity.
3. Disposition exactly 38 census tuples: 9 `deepagents.acp`, 9 `kiro.acp`, 8
   `qoder.headless`, and 12 `zcode.app-server`.
4. Treat the Deep Agents, Kiro, and Qoder
   `audit.no-public-route-specific-selectable-control` rows as negative
   coverage. Do not turn common working-resource, transport, or preparation
   inputs into route-specific composer controls.
5. Emit ZCode model selection and app-server mode only from the exact prepared
   model route and `ZcodePreparedEvidence::mode()` binding.
6. Withhold model-catalogue, audit, documentation-only,
   incompatible-operation, or other rows whenever the exact prepared facade
   lacks their source truth. Withhold at construction, not through an
   emitted-row filter.
7. Keep activity observation descriptor-only. Preparation or successful start
   does not create provider-effective, rejected, pending, acknowledged, or
   active-session state.
8. Add one deterministic adapter-local ledger per route. Assert every exact
   `(route_id, operation_shape, semantic_id)` once with an emitted or withheld
   reason and no cross-route exception list.
9. Prove exact operation shape, source identity, applicability and access
   agreement, optional/absent row behavior, lifecycle, state posture, and no
   execution or mutation authority without contacting a provider.
10. Keep the runtime/testkit/core public baseline and Contracts 037, 047, 057,
    and 061 unchanged. Stop after one reviewable four-package PR.

## Out Of Scope

- any census row outside candidate H
- a catalogue contribution, active-observation seam, acknowledgement result,
  per-turn mutation path, callback, or provider payload
- a new runtime, testkit, or core public type, maximum, failure, composer rule,
  registry, enumeration seam, or downcast
- changes to Contracts 037, 047, 057, or 061
- provider contact, live probes, compatibility/currentness work, watcher or
  skill-visibility restart, PR 127, PR 130, papercuts, or generation closeout
- promotion of candidates B-G or I-L, or compilation of Batch 9.5

## Acceptance Criteria

- [ ] four independent ledgers reconcile exactly to 9, 9, 8, and 12 rows
- [ ] every emitted row comes from its exact prepared operation and retains
      source, route, operation, lifecycle, value, omission, applicability, and
      evidence truth
- [ ] the three no-route-specific-control audits remain negative coverage and
      produce no public control descriptor
- [ ] ZCode model selection and app-server mode agree with the exact prepared
      model route and mode binding
- [ ] catalogue-only, audit, documentation-only, and incompatible-operation
      rows are withheld at construction
- [ ] activity observation remains descriptor-only on all four routes
- [ ] cross-route, cross-operation, cross-access, cross-instance, and stale
      source assembly fail closed or replace according to the existing composer
- [ ] no runtime/testkit/core public API, contract, provider operation, active
      observation, acknowledgement, execution, or mutation authority is added
- [ ] touched source remains below the configured god-file thresholds and the
      repository scan does not exceed its accepted baseline

## Review Oracle

Invariant: candidate H publishes only truth already bound by one exact prepared
operation. Shared ACP or headless shape does not let one route borrow another's
row.

Counterexamples and required proof:

- a Deep Agents, Kiro, or Qoder audit row emitted as a selectable control —
  fail; it records the absence of one
- a model-catalogue row emitted from an operation plan without catalogue
  evidence — fail; withhold it
- a ZCode mode inferred from a descriptor or command default rather than the
  prepared evidence — fail; use the bound mode
- an interactive-session row copied to Qoder's structured run, or a Qoder row
  copied to either ACP session — fail; exact operation shape is required
- activity marked observed, effective, pending, or acknowledged because a
  prepared operation exists — fail; descriptor-only posture remains
- a changed access dimension or route accepted under a matching source ID —
  fail closed before any row is published
- the combined ledger reaches 38 by claiming another route's identity — fail;
  independent totals remain 9, 9, 8, and 12

## Validation

- `cargo fmt -p swallowtail-adapter-deepagents -p swallowtail-adapter-kiro -p swallowtail-adapter-qoder -p swallowtail-adapter-zcode -- --check`
- `effigy validate:focused swallowtail-adapter-deepagents swallowtail-adapter-kiro swallowtail-adapter-qoder swallowtail-adapter-zcode`
- `effigy package:verify-affected swallowtail-adapter-deepagents swallowtail-adapter-kiro swallowtail-adapter-qoder swallowtail-adapter-zcode`
- `effigy package:api`
- `effigy qa:routes`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy --json scan god-files`
- `git diff --check`

No live probe or provider contact belongs to this card.

## Auto-Continuation

No. Stop after one reviewable PR. The orchestrator must review the exact
38-row proof before another Batch 9.4 candidate can be promoted.

## Stop Conditions

- Stop if any row needs a new runtime/testkit/core public type, source kind,
  authority posture, composer rule, fixed maximum, or contract amendment.
- Stop if a route needs a catalogue contribution, active-observation seam,
  provider acknowledgement, per-turn mutation path, callback, or provider
  payload to emit a row.
- Stop if the exact 9/9/8/12 ledgers need an exception list, inferred support,
  or truth borrowed from another route.
- Stop if activity cannot remain descriptor-only or any no-control audit would
  become a public control.
- Stop if scope widens beyond the four named adapter packages or contacts a
  provider.

## Evidence

- [post-card-023 Batch 9.4 reassessment](../../../triage/2026-08-31-contract-061-batch-9-4-package-expansion.md)
- [Contract 061](../../../contracts/061-consumer-route-feature-and-control-projection.md)
- [reviewed census](../../../triage/2026-08-30-consumer-route-feature-and-option-projection-census.csv)
- [completed card 023](023-contract-061-codex-openai-package-completion.md)
