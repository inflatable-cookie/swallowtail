# 122 OpenAI Background Reasoning Vocabulary Correction

Status: complete
Owner: Tom
Created: 2026-08-23
Milestone: [g04.044 OpenAI Background Reasoning Vocabulary Correction](../044-openai-background-reasoning-vocabulary-correction.md)
Depends on: Research 191; g04.043 evidence stop

Completed on worker branch `t3code/openai-background-reasoning-correction` in
commit `a8cad66`.

## Goal

Correct the exact GPT-5.6 background reasoning mapping to the six values proved
by Research 191 and bind the correction to a new exact opaque facade point.

## Scope

1. Remove `minimal` only from the `openai.background` exact-model preparation
   validator. Do not change provider-neutral `ReasoningMode` construction or
   another adapter's value set.
2. Admit exactly `none`, `low`, `medium`, `high`, `xhigh`, and `max`. Preserve
   the absent reasoning path.
3. Reject `minimal` and every other unsupported value during preflight with the
   existing safe `swallowtail.openai.preparation.reasoning_unsupported`
   diagnostic before endpoint, credential, request, or provider work.
4. Do not translate `minimal` to `none`, select a provider default, clamp,
   retry, fall back, or choose another route or model.
5. Publish a new exact opaque `openai.responses-background-facade` point and
   private behavior revision. Update the configured instance, claim, plan,
   fixtures, and exact facade assertions together; do not silently rewrite the
   July point.
6. Preserve exact agreement among prepared input, capability constraint, plan,
   prepared evidence, request policy, configured driver, and wire for every
   admitted value.
7. Preserve output bounds, structured output, background execution, temporary
   retention, streaming, one reattachment, cancellation, deletion,
   detachment, and reconciliation behavior.

## Acceptance Criteria

- [x] the six Research 191 values prepare on exact GPT-5.6
- [x] `minimal` and foreign values reject before effects
- [x] absent reasoning retains current request behavior
- [x] the new facade point and behavior revision bind every relevant surface
- [x] stale facade, plan, evidence, policy, and driver combinations fail closed
- [x] no public generic-value or fallback surface is added
- [x] other routes and global reasoning syntax are unchanged

## Validation

```sh
cargo fmt -p swallowtail-adapter-openai
effigy validate:focused swallowtail-adapter-openai
effigy package:verify-affected swallowtail-adapter-openai
effigy package:api
effigy qa:northstar
git diff --check
```

Card 122 passed its six-value mapping, explicit `minimal` rejection, facade
revision, absent path, and zero-effect gates. Card 123 acceptance and
route-local closeout follow on the same worker branch.

## Stop Conditions

- exact-model evidence is no longer the Research 191 six-value set
- rejection occurs after endpoint, credential, request, or provider work
- the correction requires global vocabulary or sibling-route changes
- the old and corrected opaque facade truths cannot remain distinct
- implementation requires a contract change or release action

## Out Of Scope

- OpenAI web search or any other new route capability
- model/currentness expansion, live provider work, release, or publication
- shared architecture, matrices, programme/front doors/indexes, changelog,
  release notes, workspace versions, or merge work
