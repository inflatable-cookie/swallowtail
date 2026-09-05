# 098 Contract 061 Alibaba, Anthropic, And xAI Package Completion

Status: ready
Owner: Tom
Created: 2026-09-05
Updated: 2026-09-05
Milestone: `../009-contract-061-consumer-projection-realization.md`
Depends on: completed card 096 and its audit note; Contract 061; completed cards 022-024, 031-034, 068-069, 074-075, 079

## Goal

Complete candidate B's exact 76-row Contract 061 package remainder
across `swallowtail-adapter-alibaba-model-studio`, `swallowtail-adapter-anthropic`, `swallowtail-adapter-xai`, as the ledger fixed by the
[per-turn authority audit](../../../triage/20260905-143430-contract-061-per-turn-authority-audit.md): 74 emitted and 2 withheld across 76 rows; the two withheld are `alibaba.conversations` `control.provider-state-policy` and `control.resume-session` (matrix-descriptor-only). The audit ruled that the
existing `ConsumerMediatedPerTurn` posture and projection vocabulary suffice;
no shared type, bound, or contract change is in scope.

## Scope

1. Add the established `consumer_route_projection_contribution(source_id)`
   shape to every prepared facade the audit names for `alibaba.conversations`, `anthropic.managed-agent`, `anthropic.messages`, and `xai.responses-websocket`, following the
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
4a. The single B per-turn row publishes only from its retained plan-borne
   mediation evidence as the audit anchors it; no other B row is per-turn.
5. Add one deterministic adapter-local ledger per route asserting every
   exact `(route_id, operation_shape, semantic_id)` once with an emitted or
   withheld reason and no exception list; prove source, applicability,
   lifecycle, and authority distinctions, mixed-assembly rejection, and
   negative coverage provider-free.
6. Keep the runtime, testkit, and core public baseline and Contracts 037,
   047, 057, and 061 unchanged. Regenerate the owned adapter API baseline
   files under `release-baselines/public-api-0.4.1/` additively. Stop after
   one reviewable PR.

## Out Of Scope

Any row outside candidate B; other candidates; a shared type, maximum,
composer rule, or contract amendment; provider contact.

## Acceptance Criteria

- [ ] per-route ledgers reconcile exactly to the audit totals
- [ ] every per-turn row carries `ConsumerMediatedPerTurn` from retained plan
      evidence and none claims prepared-session-start, acknowledged,
      effective, or rejected state
- [ ] descriptor-only rows never appear, and each fixture asserts why
- [ ] semantic API diff is additive only; god-file scan within baseline

## Validation

- `cargo fmt -p swallowtail-adapter-alibaba-model-studio -p swallowtail-adapter-anthropic -p swallowtail-adapter-xai -- --check`
- `effigy validate:focused swallowtail-adapter-alibaba-model-studio swallowtail-adapter-anthropic swallowtail-adapter-xai`
- `effigy package:verify-affected swallowtail-adapter-alibaba-model-studio swallowtail-adapter-anthropic swallowtail-adapter-xai`
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
