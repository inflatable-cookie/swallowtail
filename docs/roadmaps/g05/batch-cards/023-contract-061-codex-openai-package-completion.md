# 023 Contract 061 Codex And OpenAI Package Completion

Status: complete; exact 59-row tranche merged through PR 133 at `58be7122`
Owner: Tom
Created: 2026-08-31
Updated: 2026-08-31
Milestone: `../009-contract-061-consumer-projection-realization.md`
Depends on: completed card 022; Batch 9.4 package-expansion checkpoint

## Goal

Complete the remaining Contract 061 census dispositions owned by
`swallowtail-adapter-codex` and `swallowtail-adapter-openai`: all 35
`codex.exec` rows and all 24 `openai.background` rows.

## Outcome

PR 133 merged exact reviewed head `fbb4b118` through merge commit
`58be7122`. `CodexPreparedExec` and `OpenAiPreparedBackgroundRun` now publish
only exact prepared-operation truth. The independent ledgers reconcile the
full 35-row and 24-row census tuples, including explicit construction-time
withholding. No runtime, testkit, core, contract, provider-operation, or
active-observation surface changed.

## Scope

1. Add the established
   `consumer_route_projection_contribution(source_id)` shape to
   `CodexPreparedExec` and `OpenAiPreparedBackgroundRun`. Each façade emits
   only the truth admitted by its exact `PreparedOperationEvidence` and
   supplied `AdapterContribution` source identity.
2. Preserve the existing runtime-owned records, fixed maxima, pure composer,
   exact access-state agreement, source `(id, kind)` admission, and
   consumer-mediated per-turn authority without changing their public shape.
3. Disposition exactly the 35 `codex.exec` census rows across model-catalogue,
   structured-run, interactive-session, route-observation, route-capability,
   session-lifecycle, and session-management operation shapes.
4. Disposition exactly the 24 `openai.background` rows across model-catalogue,
   structured-run, route-observation, route-capability, and session-lifecycle
   operation shapes.
5. Withhold catalogue, route-wide, post-open, lifecycle, or other rows at
   construction whenever the named prepared façade lacks exact
   operation-local authority. Do not emit and filter them or borrow truth from
   `codex.app-server`, `openai.realtime`, another OpenAI endpoint, or a docs
   matrix.
6. Do not adapt `OpenAiPreparedModels` or
   `OpenAiPreparedBackgroundReconciliation` into background structured-run
   evidence. Their route or prepared-evidence family does not authorize a new
   composer input or active-observation seam in this card.
7. Keep requested, prepared, provider-effective-unobserved, and
   effective-unobserved states exact. In particular, background preparation,
   successful run creation, reconciliation support, or route documentation
   cannot invent provider-effective or acknowledgement truth.
8. Add deterministic adapter-local ledgers for exactly 35 Codex Exec and 24
   OpenAI Background rows. Assert each exact `(route_id, operation_shape,
   semantic_id)` identity once with an emitted or withheld disposition, and
   build the emitted set directly without an out-of-tranche exception list.
9. Add provider-free fixtures proving exact prepared-operation withholding,
   changed source identity replacement, access/applicability disagreement,
   lifecycle/state preservation, and absence of execution or mutation
   authority.
10. Keep all runtime/testkit/core public APIs and all contracts unchanged. Stop
   after one reviewable two-package PR for orchestrator review.

## Out Of Scope

- any census row outside `codex.exec` and `openai.background`
- a new runtime or testkit public type, fixed maximum, failure kind, composer
  rule, registry, enumeration seam, callback, downcast, or provider payload
- an OpenAI background active-observation or acknowledgement result type
- changes to `swallowtail-core` or Contracts 037, 047, 057, or 061
- provider contact, live probes, compatibility/currentness work, watcher or
  skill-visibility work, PR 127, PR 130, or generation closeout
- promotion or implementation of candidates B-L or the Batch 9.5 all-route
  audit

## Acceptance Criteria

- [x] the Codex ledger dispositions exactly 35 `codex.exec` rows and no other
      route
- [x] the OpenAI ledger dispositions exactly 24 `openai.background` rows and
      no other route
- [x] every emitted row comes from one exact prepared operation and retains
      its supplied source `(id, kind)`, route, operation, lifecycle, value,
      omission, applicability, and evidence truth
- [x] documentation-only, incompatible-operation, route-wide, and unobserved
      rows are withheld at construction rather than emitted then filtered
- [x] prepared or successful execution/reconciliation truth does not become
      provider-effective, rejected, or acknowledged truth
- [x] exact access dimensions and configured/prepared agreement continue to
      fail closed without aggregate flattening
- [x] the existing 36-row Codex App Server and 15-row OpenAI Realtime proofs
      remain unchanged and exact
- [x] no runtime/testkit/core public API, contract, provider operation, or
      execution/mutation authority is added
- [x] touched source remains below the configured god-file thresholds and the
      repository scan does not exceed its accepted baseline

## Review Oracle

Invariant: completing a package remainder adds only exact prepared-operation
truth; package ownership and documentation never widen a row's evidence.

Counterexamples and required proof:

- a `codex.exec` row copied from the app-server contribution because both live
  in one crate — fail; exact route and operation evidence are required
- an OpenAI Models or Realtime fact projected as `openai.background` because
  all three share an adapter — fail; withhold it
- the separate background reconciliation prepared family presented as
  structured-run evidence, observed recovery, or provider acknowledgement —
  fail; preserve the current composer input boundary and withhold the row
- a route-wide or matrix row emitted and removed from the ledger afterward —
  fail; construction must withhold it
- a changed source ID or access dimension merged into an existing snapshot —
  replace for the former and reject the mixed snapshot for the latter
- either ledger totals 59 only by claiming the other package's rows — fail;
  the independent totals remain 35 and 24

## Validation

- `cargo fmt -p swallowtail-adapter-codex -p swallowtail-adapter-openai -- --check`
- `effigy validate:focused swallowtail-adapter-codex swallowtail-adapter-openai`
- `effigy package:verify-affected swallowtail-adapter-codex swallowtail-adapter-openai`
- `effigy package:api`
- `effigy qa:routes`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy --json scan god-files`
- `git diff --check`

No live probe or provider contact belongs to this card.

## Auto-Continuation

No. Stop after one reviewable PR. The orchestrator must review the exact
59-row proof before another Batch 9.4 candidate can be promoted.

## Stop Conditions

- Stop if any remaining row needs a new runtime/testkit/core public type,
  composer rule, fixed maximum, source kind, authority posture, or contract
  amendment.
- Stop if either adapter needs a registry, route enumeration, callback,
  downcast, generic provider data, or truth borrowed from another route.
- Stop if the exact 35/24 ledgers cannot be constructed without an exception
  list or documentation-only support claim.
- Stop if provider-effective, rejected, acknowledgement, or active-observation
  state would have to be inferred.
- Stop if scope widens beyond the two named adapter packages or touches a
  provider.

## Evidence

- [PR 133](https://github.com/inflatable-cookie/swallowtail/pull/133) — exact
  head `fbb4b118`; merged at `58be7122`
- [exact-head orchestrator review](https://github.com/inflatable-cookie/swallowtail/pull/133#issuecomment-5479727867)
- [card 023 closeout](../../../logs/2026-08-31-g05-009-card-023-package-completion.md)
- [Batch 9.4 package expansion](../../../triage/2026-08-31-contract-061-batch-9-4-package-expansion.md)
- [Contract 061](../../../contracts/061-consumer-route-feature-and-control-projection.md)
- [reviewed census](../../../triage/2026-08-30-consumer-route-feature-and-option-projection-census.csv)
- [completed card 022](022-contract-061-composer-and-two-route-vertical.md)
