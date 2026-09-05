# 099 Contract 061 Mistral Vibe, Muse, Oh My Pi, And Qwen Package Completion

Status: ready
Owner: Tom
Created: 2026-09-05
Updated: 2026-09-05
Milestone: `../009-contract-061-consumer-projection-realization.md`
Depends on: completed card 096 and its audit note; Contract 061; completed cards 022-024, 031-034, 068-069, 074-075, 079

## Goal

Complete candidate K's exact 52-row Contract 061 package remainder
across `swallowtail-adapter-mistral-vibe`, `swallowtail-adapter-muse`, `swallowtail-adapter-oh-my-pi`, `swallowtail-adapter-qwen`, as the ledger fixed by the
[per-turn authority audit](../../../triage/20260905-143430-contract-061-per-turn-authority-audit.md): 52 emitted and 0 withheld across 52 rows; no documentation-only rows. The audit ruled that the
existing `ConsumerMediatedPerTurn` posture and projection vocabulary suffice;
no shared type, bound, or contract change is in scope.

## Scope

1. Add the established `consumer_route_projection_contribution(source_id)`
   shape to every prepared facade the audit names for `mistral-vibe.headless`, `muse-code.headless`, `oh-my-pi.rpc`, and `qwen.headless`, following the
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
4a. The single K per-turn row is the `oh-my-pi.rpc` attachment row and
   publishes only from a facade whose plan carries the bounded
   `Capability::Attachments` requirement (`prepared/instance.rs`).
4b. This card sits exactly at the four-package focused-validation maximum;
   do not add a fifth package for any reason.
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

Any row outside candidate K; other candidates; a shared type, maximum,
composer rule, or contract amendment; provider contact.

## Acceptance Criteria

- [ ] per-route ledgers reconcile exactly to the audit totals
- [ ] every per-turn row carries `ConsumerMediatedPerTurn` from retained plan
      evidence and none claims prepared-session-start, acknowledged,
      effective, or rejected state
- [ ] descriptor-only rows never appear, and each fixture asserts why
- [ ] semantic API diff is additive only; god-file scan within baseline

## Validation

- `cargo fmt -p swallowtail-adapter-mistral-vibe -p swallowtail-adapter-muse -p swallowtail-adapter-oh-my-pi -p swallowtail-adapter-qwen -- --check`
- `effigy validate:focused swallowtail-adapter-mistral-vibe swallowtail-adapter-muse swallowtail-adapter-oh-my-pi swallowtail-adapter-qwen`
- `effigy package:verify-affected swallowtail-adapter-mistral-vibe swallowtail-adapter-muse swallowtail-adapter-oh-my-pi swallowtail-adapter-qwen`
- `effigy package:api`
- `effigy qa:routes`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy --json scan god-files`
- `git diff --check`

## Result

Implemented candidate K across the four owned adapters. The adapter-local
ledgers reconcile exactly to 8/10/18/16 rows for Mistral Vibe, Muse Code, Oh My
Pi, and Qwen: 52 emitted and 0 withheld. Added prepared-facade contributions,
plan-backed controls, mixed-applicability rejection, and provider-free negative
coverage. The only per-turn row is Oh My Pi interactive
`control.attachments`, sourced from retained bounded `Capability::Attachments`
evidence and carrying `ConsumerMediatedPerTurn`; it makes no
provider-effective claim.

Validation passed: the exact four-package fmt check; focused validation and
affected-package verification for the exact four packages; package API;
`qa:routes`; `qa:docs`; `qa:northstar`; the JSON god-file scan (`ok: true`);
and `git diff --check`. The API baselines are additive only. No shared runtime,
testkit, core, contract, census, audit, other-adapter, roadmap-closeout, or
version-claim surface changed.

PR: [#235](https://github.com/inflatable-cookie/swallowtail/pull/235)

## Review Oracle

Invariant: a per-turn row is truth only because the consumer mediated it on
the plan. Smallest counterexample: a per-turn row emitted from a facade
prepared without the mediating requirement, a matrix-only row emitted, or a
per-turn row carrying provider-effective state.

## Auto-Continuation

No. Stop after one reviewable PR for exact-head review.
