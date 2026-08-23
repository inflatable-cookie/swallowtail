# 141 DeepSeek Structured-Run Thinking-Mode Acceptance

Status: complete
Owner: Tom
Created: 2026-08-23
Updated: 2026-08-23
Milestone: [g04.050 DeepSeek Structured-Run Thinking Mode](../050-deepseek-structured-run-thinking-mode.md)
Depends on: card 140

## Goal

Prove the exact admitted DeepSeek structured-run thinking-mode boundary,
preserve enabled reasoning and continuation, and publish route-local guidance
without claiming provider acceptance or effective mode.

## Scope

1. Add deterministic prepared-facade and protocol coverage for every Research
   197 deliver-now state, omitted/current behavior, and rejected boundary.
2. Prove exact selected-mode agreement across input, plan/evidence, driver, and
   request body. Prove disabled mode carries no portable reasoning selection or
   unqualified effort field.
3. Prove existing enabled `low|high|max` structured runs retain exact bytes,
   capability constraints, response parsing, cancellation, deadline, failure,
   and cleanup.
4. Prove direct session preparation remains enabled-only and that initial,
   tool-result, later-turn, and restored requests retain fixed effort and
   private `reasoning_content` replay.
5. Prove response drift, unknown fields/values, model/facade/plan/evidence
   mismatch, cache-policy mismatch, and disabled-session attempts follow
   Research 197's fail-closed disposition.
6. Update the DeepSeek prepared guide, Research 197, cards 139-141, g04.050,
   reserved route-local closeout, examples, and package-specific unreleased API
   baseline when applicable.
7. Record the exact architecture, route/feature matrix, programme, indexes,
   changelog, Contract 029, generation-boundary, and Next Task delta in the
   closeout and PR body. Do not edit those shared surfaces on the worker branch.

## Acceptance Criteria

- [x] every admitted state and rejected boundary has deterministic coverage
- [x] enabled and disabled dispatch truth remains exact and distinct
- [x] continuation/private-replay behavior is unchanged and enabled-only
- [x] default QA performs no credential, account, external request, provider
      prompt, or paid work
- [x] docs do not infer acceptance, effective mode, quality, latency, price,
      cache effect, or private reasoning from dispatch
- [x] closeout records PR/head truth without claiming merge
- [x] worker changes stay inside named code and route-local docs
- [x] named gates pass

## Validation

```sh
cargo fmt -p swallowtail-adapter-deepseek
effigy validate:focused swallowtail-adapter-deepseek
effigy package:verify-affected swallowtail-adapter-deepseek
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

- exact request, response, cache, lifecycle, cleanup, or continuation truth
  cannot be proved
- docs would infer provider acceptance, effective mode, private reasoning,
  quality, latency, or price
- another route/control family, currentness lane, contract, release, or
  generation rollout enters scope

## Out Of Scope

- live provider verification, publication, merge, shared front-door edits, or
  post-g04 generation planning

## Closeout

Card 141 added deterministic protocol and prepared-facade coverage for the
disabled structured-run request, ordinary no-private-reasoning response, and
private-reasoning drift rejection. Existing enabled effort, continuation,
restoration, cancellation, deadline, failure, and cleanup tests remain green.
The prepared guide and Research 197 now describe dispatch truth without
claiming provider acceptance or effective mode.

Named validation passed in the worker worktree: focused package validation,
affected-package verification, examples, route QA, Northstar QA, research/log/
roadmap indexes, next-action QA, package API, and diff checks. No credentials,
account state, live provider request, paid work, or merge was used. The worker
PR is [#49](https://github.com/inflatable-cookie/swallowtail/pull/49), opened
from `t3code/review-deepseek-thinking-handoff` at implementation commit
`ac0378d6d5ce7f1a2cae4463d6606362a6a1e4a6`; no merge was performed.
