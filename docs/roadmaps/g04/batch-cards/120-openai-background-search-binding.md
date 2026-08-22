# 120 OpenAI Background Search Binding

Status: planned; conditional on card 119
Owner: Tom
Created: 2026-08-22
Milestone: [g04.043 OpenAI Background Hosted Search](../043-openai-background-hosted-search.md)
Depends on: card 119; promoted Research 191 with a non-empty deliver-now set

## Goal

Bind only Research 191's deliver-now OpenAI background search subset through
typed prepared input, immutable plan/evidence, request policy, low-level driver,
exact Responses wire, and qualified activity projection.

## Scope

1. Add one optional typed external-search selection to
   `OpenAiBackgroundRunProfileInput`; preserve existing constructors and the
   exact absent path. Expose no raw tool, JSON, source, or search-option map.
2. Admit only the exact Research 191 model/facade/request combination. Keep the
   positive total-call maximum adapter-owned and fixed unless existing portable
   vocabulary already carries it exactly.
3. Bind `Capability::ExternalSearch`,
   `Capability::ProviderExternalNetwork`, `ExternalSearchPolicy::Enabled`, and
   `ExternalNetworkPolicy::HostApproved` through route, instance,
   requirements, plan, prepared evidence, request, and driver.
4. Encode only the exact qualified Responses fields, including the selected
   tool type, total-call maximum, tool choice, and source inclusion. Do not use
   `StructuredRunRequest::tools()` for a provider-owned tool.
5. Preserve exact model, output bound, reasoning, structured-output,
   background, stream, store, retention, reattachment, cancel, delete,
   detachment, and reconciliation behavior for every admitted combination.
6. Parse only Research 191's exact search stream/retrieve items. Project the
   qualified portable progress/activity lifecycle without turning sources,
   queries, or raw provider bodies into diagnostics.
7. Bind the feature to the exact facade point/private behavior revision
   admitted by Research 191. Do not retroactively widen the July point.
8. Reject missing capability, policy, bound, model, facade, plan, evidence,
   request, or driver agreement before endpoint or credential work. Preserve
   explicit provider rejection after dispatch.

## Acceptance Criteria

- [ ] only Research 191 deliver-now rows prepare
- [ ] typed input, plan, evidence, policy, driver, and wire agree exactly
- [ ] provider-owned search never enters the consumer-tool collection
- [ ] absent search preserves current request bytes and public behavior
- [ ] existing generation controls and lifecycle semantics remain independent
- [ ] selected/invoked/completed/source/output truth remains distinct
- [ ] malformed, foreign, duplicate, unbound, and drifted items fail closed
- [ ] all knowable mismatches reject before provider work

## Validation

```sh
cargo fmt -p swallowtail-adapter-openai
effigy validate:focused swallowtail-adapter-openai
effigy package:verify-affected swallowtail-adapter-openai
effigy package:api
effigy qa:northstar
git diff --check
```

Auto-continue to card 121 when exact preparation, request encoding, event
projection, absent-path, and zero-network rejection tests pass.

## Stop Conditions

- portable external-search semantics cannot represent the exact provider tool
- a positive use bound or provider-network posture cannot remain immutable
- existing background lifecycle or generation controls would weaken
- exact event/source parsing needs new unplanned public vocabulary
- compatibility needs a contract change, live provider proof, or breaking API

## Out Of Scope

- guide, matrices, architecture, programme, indexes, or shared closeout
- filters, location, image search, file search, MCP, functions, service tier,
  or sibling OpenAI routes
- live provider work
