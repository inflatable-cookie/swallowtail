# 069 Contract 061 Antigravity, Bedrock, And Cursor Package Completion

Status: ready
Owner: Tom
Created: 2026-09-04
Updated: 2026-09-04
Milestone: `../009-contract-061-consumer-projection-realization.md`
Depends on: completed card 064; the candidate C audit note; Contract 061; completed cards 022-024

## Goal

Complete candidate C's Contract 061 census dispositions: all 14
`antigravity.catalogue`, 18 `antigravity.headless`, 9 `bedrock.catalogue`,
10 `bedrock.runtime`, 13 `cursor-agent.acp`, 13 `cursor-agent.catalogue`,
and 17 `cursor-agent.headless` rows, as the exact 51 emitted / 43 withheld
ledger fixed by the
[candidate C audit](../../../triage/20260904-134500-contract-061-candidate-c-audit.md).

## Scope

1. Add the established `consumer_route_projection_contribution(source_id)`
   shape to the eight prepared facades the audit names across
   `swallowtail-adapter-antigravity`, `swallowtail-adapter-bedrock`, and
   `swallowtail-adapter-cursor`, including `AntigravityPreparedContinuation`,
   following the `codex.exec`, `zcode.app-server`, and card 024 precedents.
2. Use only each facade's exact `PreparedOperationEvidence`, activity
   profile, request input, and adapter-local prepared evidence. Preserve the
   caller-supplied source identity as `AdapterContribution`.
3. Disposition exactly 94 census tuples with the audit's per-route ledgers:
   `antigravity.catalogue` 2 E / 12 W, `antigravity.headless` 16 E / 2 W,
   `bedrock.catalogue` 2 E / 7 W, `bedrock.runtime` 8 E / 2 W,
   `cursor-agent.acp` 7 E / 6 W, `cursor-agent.catalogue` 2 E / 11 W,
   `cursor-agent.headless` 14 E / 3 W.
4. Keep the four `audit.no-public-route-specific-selectable-control` rows
   (`antigravity.catalogue`, `bedrock.catalogue`, `cursor-agent.acp`,
   `cursor-agent.catalogue`) as negative coverage. They never become a
   public control descriptor.
5. Withhold at construction, never through an emitted-row filter, under the
   audit's five rules: incompatible operation shape, documentation-only
   matrix features, negative-coverage audit rows, activity observation on
   catalogue operations that attach no activity profile, and conditional
   features not requested in the profile input.
6. Emit the twelve controls with the audit's exact value kinds, domains, and
   omission semantics: portable `ModelSelection`, `ReasoningSelection`, and
   `MaximumOutputTokens` where the census names them; bounded namespaced
   extensions for `structured-output`, `resource-access`, and `isolation` on
   `antigravity.headless` and for `fast`, `context-window`,
   `reasoning-effort`, and `read-mode` on `cursor-agent.headless`.
7. Emit `feature.reasoning-selection` and `feature.structured-output` on
   `antigravity.headless` only when the profile input requests them, and
   `feature.reasoning-selection` on `cursor-agent.headless` only when effort
   is configured, mirroring the `codex.exec` conditional row.
8. Emit `feature.activity-observation` descriptor-only and only from
   operations whose preflight plan carries an `ObservableActivityProfile`
   (`antigravity.headless` run and continuation, `bedrock.runtime`,
   `cursor-agent.acp`, `cursor-agent.headless`).
9. Keep catalogue, headless, continuation, and ACP shapes distinct; the
   existing cross-route and cross-operation mixture rejection applies.
10. Add one deterministic adapter-local ledger per route, asserting every
    exact tuple once with an emitted or withheld reason and no exception
    list.
11. Keep the runtime/testkit/core public baseline and Contracts 037, 047,
    057, and 061 unchanged. Respect the fixed maxima the audit tallied. Stop
    after one reviewable three-package PR.

## Out Of Scope

- any census row outside candidate C; candidates B, E, F, I, K, L
- a catalogue contribution seam, active-observation seam, acknowledgement
  result, per-turn mutation path, callback, or provider payload
- a new runtime, testkit, or core public type, maximum, failure, composer
  rule, registry, or downcast
- changes to Contracts 037, 047, 057, or 061
- provider contact, live probes, currentness (including parked Antigravity
  `1.1.24` and PR 182), watcher or skill work, papercut fixes, or
  generation closeout

## Acceptance Criteria

- [ ] seven independent ledgers reconcile exactly to 14, 18, 9, 10, 13, 13,
      and 17 rows with 51 emitted and 43 withheld in total
- [ ] every emitted row comes from its exact prepared operation and retains
      source, route, operation shape, lifecycle, value, omission,
      applicability, and evidence truth
- [ ] the four no-control audits remain negative coverage and produce no
      public control descriptor
- [ ] catalogue operations emit no activity row; headless, continuation,
      runtime, and ACP operations emit it descriptor-only
- [ ] conditional features appear only when requested or configured
- [ ] no runtime/testkit/core public API, contract, active observation,
      acknowledgement, execution, or mutation authority is added
- [ ] touched source stays below god-file thresholds and the repository
      scan does not exceed its accepted baseline

## Review Oracle

Invariant: candidate C publishes only truth already bound by one exact
prepared operation on one exact route and shape.

Counterexamples, each a fail:

- a no-control audit row emitted as a selectable control
- `feature.activity-observation` emitted from a catalogue operation
- a matrix-only feature emitted without a preflight binding
- a conditional feature emitted from an unrequested profile
- a headless row copied onto a catalogue or ACP shape, or the reverse
- an Antigravity continuation row that ignores the continuation facade's
  own prepared evidence
- ledgers reaching 94 through an exception list or borrowed identity

## Validation

- `cargo fmt -p swallowtail-adapter-antigravity -p swallowtail-adapter-bedrock -p swallowtail-adapter-cursor -- --check`
- `effigy validate:focused swallowtail-adapter-antigravity swallowtail-adapter-bedrock swallowtail-adapter-cursor`
- `effigy package:verify-affected swallowtail-adapter-antigravity swallowtail-adapter-bedrock swallowtail-adapter-cursor`
- `effigy package:api`
- `effigy qa:routes`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy --json scan god-files`
- `git diff --check`

No live probe or provider contact belongs to this card.

## Auto-Continuation

No. Stop after one reviewable PR for exact-head review.

## Stop Conditions

- any row needs a new runtime/testkit/core public type, source kind,
  authority posture, composer rule, fixed maximum, or contract amendment
- the exact per-route ledgers need an exception list, inferred support, or
  truth borrowed from another route or shape
- a no-control audit would become a public control, or activity cannot stay
  descriptor-only
- scope widens beyond the three named adapter packages or contacts a
  provider

## Evidence

- [candidate C audit](../../../triage/20260904-134500-contract-061-candidate-c-audit.md)
- [Batch 9.4 package expansion](../../../triage/2026-08-31-contract-061-batch-9-4-package-expansion.md)
- [Contract 061](../../../contracts/061-consumer-route-feature-and-control-projection.md)
- [reviewed census](../../../triage/2026-08-30-consumer-route-feature-and-option-projection-census.csv)
- [completed card 024](024-contract-061-deepagents-kiro-qoder-zcode-package-completion.md)
