# 237 OpenAI Realtime Reasoning-Effort Acceptance

Status: done
Owner: Tom
Created: 2026-08-27
Updated: 2026-08-27
Milestone: [g04.084 OpenAI Realtime Reasoning Effort](../084-openai-realtime-reasoning-effort.md)
Depends on: card 236

## Goal

Prove exact OpenAI Realtime session-scoped reasoning dispatch,
acknowledgement, omission, restoration, and unchanged lifecycle behavior, then
produce one review-ready route-local closeout.

## Scope

1. Add deterministic prepared-facade, protocol, and driver coverage for every
   admitted and rejected Research 236 value.
2. Assert exact model, new facade/private behavior point, capability,
   constraint, plan, evidence, request, session-update bytes, and matching
   session-updated value.
3. Prove explicit missing, malformed, foreign, and mismatched acknowledgement
   fails the open and joins connection work before credential release.
4. Assert omission retains the historical no-`reasoning` session-update bytes
   and existing session acknowledgement behavior without a default claim.
5. Prove every admitted reasoning value composes independently with output
   maximum omission and the positive 1..=4,096 range.
6. Prove fresh working-state restoration preserves selection while remaining
   context-losing `SessionReplaced`, not reconnect, resume, or rollover.
7. Preserve manual PCM formats, two-turn bound, response cancellation,
   transcript/audio events, usage, rate observations, provider failures,
   deadline, disconnect, invalidation, and joined cleanup.
8. Update the Realtime integration guide, feature/route matrices, example if it
   materially clarifies the public builder, package API baseline, changelog,
   Research 236, cards 236-237, g04.084, and the reserved route-local log.
   Record shared programme/index/Next Task deltas for orchestrator closeout;
   do not edit those shared planning surfaces on the worker branch.

## Acceptance Criteria

- [x] every admitted and rejected value has deterministic coverage
- [x] explicit selection is not usable without exact acknowledgement
- [x] omission, output maximum, restoration, and lifecycle proofs stay exact
- [x] default QA uses no credential, account, socket, provider request, media
      operation, or paid work
- [x] docs distinguish requested, planned, dispatched, and acknowledged from
      effective or observed reasoning
- [x] per-response override and reasoning-token inference stay withheld
- [x] worker changes stay within OpenAI code, route-local docs/evidence,
      matrices, changelog, and package API baseline
- [x] named gates pass

## Validation

```sh
cargo fmt -p swallowtail-adapter-openai
effigy validate:focused swallowtail-adapter-openai
effigy package:verify-affected swallowtail-adapter-openai
effigy check:examples
effigy qa:routes
effigy qa:northstar
effigy package:api
git diff --check
```

Auto-continuation: No.

## Stop Conditions

- exact dispatch or acknowledgement cannot be proved without live provider work
- omission, output maximum, or restoration behavior changes
- cleanup no longer joins before credential release
- another route, timing, model, control family, contract, currentness lane, or
  release enters scope

## Out Of Scope

Live provider verification, effective reasoning, per-response override,
reasoning-token claims, publication, merge, shared front-door closeout, or later
feature selection.

## Evidence

Route-local docs, matrices, changelog, API baseline, Research 236, milestone,
and log updated on the worker branch. Shared inventory/programme/index/Next Task
deltas are recorded for orchestrator closeout only.
