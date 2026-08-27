# 223 xAI Responses WebSocket Web Search Binding

Status: blocked; Research 227 empty deliver-now set
Owner: Tom
Created: 2026-08-27
Updated: 2026-08-27
Milestone: [g04.080 xAI Responses WebSocket Web Search](../080-xai-responses-websocket-web-search.md)
Depends on: card 222; promoted Research 227 with a non-empty deliver-now set

## Goal

Bind only Research 227's exact web-search rows through explicit prepared
search intent, immutable evidence, capability/policy agreement, canonical xAI
request bytes, and bounded provider-owned response projection.

## Scope

1. Expose only `ExternalSearchPolicy::Enabled` on exact admitted run/session
   profiles. Do not expose raw tool JSON, arbitrary provider names, filters,
   include lists, or generic tool configuration.
2. Keep omission as exact `ExternalSearchPolicy::Disabled` and `tools: []`.
3. Bind the exact model/profile, provider tool, fixed positive use bound, and
   any required route-private facade revision in prepared evidence and plan
   capabilities. Reject unsupported or mismatched rows before endpoint or
   credential work.
4. Encode exactly one canonical `web_search` tool shape admitted by Research
   227. Do not enable X search, image search, code execution, file search,
   MCP, functions, or consumer tools.
5. Retain `ExternalNetworkPolicy::Denied`. Search runs inside xAI; no host
   network grant, browser, fetch, callback, or tool-result port is created.
6. Decode only the bounded provider-owned search-call, citation, usage, cost,
   failure, and terminal subset Research 227 admits. Raw provider payloads and
   search results remain private.
7. Keep structured-run and serial-session membership independent. Reapply one
   immutable admitted selection on each eligible continuation/restoration turn
   only when exact evidence permits it.
8. Preserve `store=false`, model/reasoning/output controls, private
   continuation, working-state replacement, cancellation, deadline, socket
   invalidation, terminal mapping, billed cost, and credential-last cleanup.

## Acceptance Criteria

- [ ] only exact Research 227 model/profile rows prepare enabled web search
- [ ] request, plan, prepared evidence, policy, driver, and wire agree
- [ ] omission retains exact `tools: []` bytes
- [ ] host networking stays denied and no consumer tool authority appears
- [ ] unsupported, stale, mismatched, or wider rows reject before effects
- [ ] docs claim no stronger invocation, citation, usage, billing, or outcome
      truth than exact evidence supports
- [ ] reasoning, output limits, continuation, restoration, terminal, and
      lifecycle do not widen

## Validation

```sh
cargo fmt -p swallowtail-adapter-xai
effigy validate:focused swallowtail-adapter-xai
effigy package:verify-affected swallowtail-adapter-xai
effigy package:api
effigy qa:northstar
git diff --check
```

Auto-continue to card 224 only when exact preparation, policy/plan agreement,
canonical request encoding, bounded decoding, omission, rejection, and
lifecycle proof pass.

## Stop Conditions

- immutable search selection cannot agree across prepared and low-level paths
- search requires raw options, host networking, account inspection, or live
  provider acceptance to be safe
- response projection cannot separate provider search from consumer tools or
  preserve bounded citation/terminal truth

## Out Of Scope

- other xAI tools, sibling routes, portable tool/search expansion, live
  provider work, currentness, release, merge, rollover, or g04 closure
