# 097 Contract 061 OpenCode And Pi Package Completion

Status: ready
Owner: Tom
Created: 2026-09-05
Updated: 2026-09-05
Milestone: `../009-contract-061-consumer-projection-realization.md`
Depends on: completed card 096 and its audit note; Contract 061; completed cards 022-024, 031-034, 068-069, 074-075, 079

## Goal

Complete candidate L's exact 69-row Contract 061 package remainder
across `swallowtail-adapter-opencode`, `swallowtail-adapter-pi`, as the ledger fixed by the
[per-turn authority audit](../../../triage/20260905-143430-contract-061-per-turn-authority-audit.md): `opencode.http` 33 emitted / 2 withheld, `pi.rpc` 15 / 0, `pi.sdk-sidecar` 16 / 3; 64 emitted and 5 withheld across 69 rows (corrected 2026-09-05 after review: the audit's 19/0 for the sidecar was a stated target, and the exact session plan carries no catalogue role, no usage-reporting requirement, and no activity-profile requirement, so `feature.model-catalogue`, `feature.usage-evidence`, and `feature.activity-observation` are withheld at construction with anchored reasons; widening the production plan to reach the target is forbidden by this card's oracle). The audit ruled that the
existing `ConsumerMediatedPerTurn` posture and projection vocabulary suffice;
no shared type, bound, or contract change is in scope.

## Scope

1. Add the established `consumer_route_projection_contribution(source_id)`
   shape to every prepared facade the audit names for `opencode.http`, `pi.rpc`, and `pi.sdk-sidecar`, following the
   `codex.exec`, `zcode.app-server`, card 024, and card 068 precedents.
2. Publish per-turn rows only with `ConsumerRouteMutationAuthority::ConsumerMediatedPerTurn`
   and only from retained plan-borne evidence: a bounded
   `Capability::Attachments` requirement, or an
   `ambient_harness_with_consumer_mediated_requests` policy with the exact
   extension namespaces on the immutable `PreflightPlan`. Never from a
   successful local call or a prepared plan alone; `admit_lifecycle_authority`
   already rejects that and the ledger must prove it.
3. Emit `feature.attachments` selection-summary rows only from a facade
   prepared with attachments; never widen them from the feature matrix.
4. Withhold every `matrix-descriptor-only` row at construction as negative
   coverage.
4a. `opencode.http` `feature.permission-exchange` uses a bounded `Namespaced`
   extension over the `opencode/permission` namespace; do not fold it into
   `QuestionExchange`, which is a separate row on the same route with its
   own namespace. `control.reasoning-selection` and
   `control.provider-turn-reference` on `opencode.http` stay withheld; the
   reconciliation path already rejects every turn reference.
4b. Callbacks excluded under active-turn detachment
   (`operations/integration.rs`) must not publish a per-turn row.
5. Add one deterministic adapter-local ledger per route asserting every
   exact `(route_id, operation_shape, semantic_id)` once with an emitted or
   withheld reason and no exception list, and bind it to real
   contributions in both directions on the codex and llama-cpp precedent:
   drive each prepared fixture and assert that every ledger-claimed row is
   published by the named facade and every withheld row is absent. A
   ledger that only describes itself does not satisfy this item.
6. Keep the runtime, testkit, and core public baseline and Contracts 037,
   047, 057, and 061 unchanged. Regenerate the owned adapter API baseline
   files under `release-baselines/public-api-0.4.1/` additively. Stop after
   one reviewable PR.

## Out Of Scope

Any row outside candidate L; other candidates; a shared type, maximum,
composer rule, or contract amendment; provider contact.

## Acceptance Criteria

- [ ] per-route ledgers reconcile exactly to the audit totals
- [ ] every per-turn row carries `ConsumerMediatedPerTurn` from retained plan
      evidence and none claims prepared-session-start, acknowledged,
      effective, or rejected state
- [ ] descriptor-only rows never appear, and each fixture asserts why
- [ ] semantic API diff is additive only; god-file scan within baseline

## Validation

- `cargo fmt -p swallowtail-adapter-opencode -p swallowtail-adapter-pi -- --check`
- `effigy validate:focused swallowtail-adapter-opencode swallowtail-adapter-pi`
- `effigy package:verify-affected swallowtail-adapter-opencode swallowtail-adapter-pi`
- `effigy package:api`
- `effigy qa:routes`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy --json scan god-files`
- `git diff --check`

## Review Oracle

Invariant: a per-turn row is truth only because the consumer mediated it on
the plan. Smallest counterexample: a per-turn row emitted from a facade
prepared without the mediating requirement, a matrix-only row emitted, or a
per-turn row carrying provider-effective state.

## Auto-Continuation

No. Stop after one reviewable PR for exact-head review.
