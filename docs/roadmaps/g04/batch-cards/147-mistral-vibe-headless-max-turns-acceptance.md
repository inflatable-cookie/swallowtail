# 147 Mistral Vibe Headless Maximum-Turn Acceptance

Status: complete
Owner: Tom
Created: 2026-08-23
Updated: 2026-08-23
Milestone: [g04.052 Mistral Vibe Headless Maximum Turns](../052-mistral-vibe-headless-max-turns.md)
Depends on: card 146

## Goal

Prove the exact admitted Mistral Vibe maximum-turn boundary and publish
route-local guidance without overstating counter, terminal, provider,
effective-work, quality, latency, cost, or billing truth.

## Scope

1. Add deterministic command, prepared-facade, driver, stream, terminal, and
   cleanup coverage for every Research 199 deliver-now state, omission, and
   rejected boundary.
2. Prove exact selected-value agreement across input, immutable plan/evidence,
   driver, and command.
3. Prove caller omission retains exact current `--max-turns 8` argv and
   behavior while upstream unbounded flag omission remains impossible.
4. Prove exact at-limit/over-limit exit, stderr, stream, partial-event,
   terminal, cancellation, deadline, failure, and cleanup truth from Research
   199. Earlier public events must not turn a native limit failure into success.
5. Prove prompt transport, streaming output, plan agent, trust, workdir, local
   access, required host services, deadline, and one-child lifecycle do not
   change.
6. Update the Mistral Vibe prepared guide, Research 199, cards 145-147,
   g04.052, reserved route-local closeout, examples, fixtures, and package-
   specific unreleased API baseline when applicable.
7. Record the exact architecture, Contract 029, route/feature matrix,
   programme, indexes, changelog, and sole Next Task delta in the closeout and
   PR body. Do not edit those shared surfaces on the worker branch.

## Acceptance Criteria

- [x] every admitted state and rejected boundary has deterministic coverage
- [x] omission and selected dispatch truth remain exact and distinct
- [x] counter and terminal claims do not exceed Research 199 evidence
- [x] default QA performs no install, login, setup, credential, catalogue,
      external request, provider prompt, or paid work
- [x] docs do not infer provider acceptance, effective work, quality, latency,
      cost, billing, or output-token enforcement from turn-limit dispatch
- [x] closeout records PR/head truth without claiming merge
- [x] worker changes stay inside named code and route-local docs
- [x] named gates pass

## Validation

```sh
cargo fmt -p swallowtail-adapter-mistral-vibe
effigy validate:focused swallowtail-adapter-mistral-vibe
effigy package:verify-affected swallowtail-adapter-mistral-vibe
effigy check:examples
effigy qa:routes
effigy qa:northstar
effigy qa:docs:index:research
effigy qa:docs:index:logs
effigy qa:docs:index:roadmaps
effigy qa:docs:index:roadmaps:g04
effigy qa:docs:index:roadmaps:batch-cards
effigy qa:docs:next-action:roadmaps
effigy package:api
git diff --check
```

Auto-continuation: No.

## Stop Conditions

- exact command, counter, terminal, lifecycle, cancellation, or cleanup truth
  cannot be proved deterministically
- docs would infer provider acceptance, effective work, quality, latency, cost,
  billing, or output-token enforcement
- another route/control family, currentness lane, contract, release, or
  generation rollover enters scope

## Out Of Scope

- live provider verification, shared front-door edits, publication, merge, or
  g04 closure
