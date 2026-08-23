# 137 OpenAI Background Service-Tier Binding

Status: blocked; card 136 evidence stop
Owner: Tom
Created: 2026-08-23
Updated: 2026-08-23
Milestone: [g04.049 OpenAI Background Service Tier](../049-openai-background-service-tier.md)
Depends on: card 136; promoted Research 196 with a non-empty deliver-now set

## Goal

Bind only Research 196's exact OpenAI Background service-tier values and
profiles through adapter-local typed prepared state, immutable evidence,
driver validation, exact request encoding, and qualified response parsing.

## Scope

1. Add one optional typed service-tier selection to only the Research 196
   admitted background profiles. Preserve current constructors and exact
   omission behavior.
2. Keep the control inside `swallowtail-adapter-openai`. Do not add a portable
   capability, shared generation-control field, generic provider settings map,
   or sibling-route behavior.
3. Admit only Research 196's canonical values. Reject aliases, unknown strings,
   unsupported values, and profile combinations before endpoint, credential,
   or provider work. Do not default, clamp, translate, substitute, or infer.
4. Bind the exact selection through configured input, preflight plan/evidence,
   bound driver state, and request encoding. Reject model/facade/input/plan/
   evidence/driver drift before effects when knowable.
5. Preserve exact prior create-request bytes when omitted. Do not serialize
   `auto` merely to reproduce the provider's omission default unless Research
   196 independently admits that explicit selection.
6. Parse the returned service tier only to Research 196's admitted observation
   boundary. Unknown future values fail closed where the route claims exact
   observation; absence and request/response mismatch follow the promoted
   disposition.
7. Preserve one reattachment, retrieval, cancellation, deletion, provider
   failure, and joined cleanup. Support controlled detachment/reconciliation
   only if Research 196 proves the necessary selected/resolved truth survives.
8. Compose with absent and every admitted reasoning value plus absent and
   selected structured output. Preserve output bounds and retention policy.
9. Advance only the exact facade/private behavior/claim/model-route revisions
   Research 196 selects. Retain prior facade points as superseded proof.

## Acceptance Criteria

- [ ] only Research 196 deliver-now values and profiles prepare
- [ ] input, plan/evidence, driver, request bytes, and admitted observations
      agree exactly
- [ ] omission preserves the prior create request
- [ ] aliases, unknown values, unqualified access, and drift reject before
      effects where knowable
- [ ] reasoning, structured output, reattachment, and lifecycle behavior remain
      unchanged
- [ ] no shared runtime, portable capability, sibling route, project setting,
      retry, fallback, cost, latency, or capacity claim enters the API

## Validation

```sh
cargo fmt -p swallowtail-adapter-openai
effigy validate:focused swallowtail-adapter-openai
effigy package:verify-affected swallowtail-adapter-openai
effigy package:api
effigy qa:northstar
git diff --check
```

Auto-continue to card 138 when exact preparation, request, response,
composition, rejection, and every admitted lifecycle profile pass.

## Stop Conditions

- adapter-local prepared state cannot express the admitted exact set
- requested and returned tier truth become conflated
- implementation changes another OpenAI route or control
- implementation needs a portable capability, shared checkpoint, live proof,
  unplanned contract change, or breaking API

## Out Of Scope

- shared docs/indexes, other controls/routes/models, live provider work,
  release, or merge

## Closeout

Not executed. Research 196 admits no deliver-now value or profile. Official
docs freeze the complete enum, Fast aliasing, Ultrafast access control, and
requested-versus-returned drift, but this route cannot prove access,
observation, or durable selected/returned truth. There is no typed input,
plan constraint, prepared evidence, driver binding, request field, or
response parser to implement.
