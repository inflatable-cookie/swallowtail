# 187 OpenCode HTTP Web Search Evidence

Status: ready
Owner: Tom
Created: 2026-08-26
Milestone: [g04.067 OpenCode HTTP Web Search](../067-opencode-http-web-search.md)
Depends on: Research 176; Contracts 008, 010, 013, 029, 033, 041

## Goal

Determine the exact OpenCode HTTP versions, providers/backends, permission
rules, operation profiles, and shared policy rows on which native `websearch`
can be admitted without ambient configuration authority, permission/network
conflation, or live-provider inference. Promote an honest empty set when any
required fact remains unproved.

## Work

1. Retrieve current official OpenCode permissions/tools documentation and
   exact `v1.18.20` source for `websearch`, tool registration, provider/backend
   selection, environment gates, session input, permission evaluation, prompt,
   events, replies, errors, and release identity. Record dates, revisions, and
   decisive hashes in Research 214.
2. Freeze the exact `websearch` permission name, query-pattern shape, rule
   ordering, last-match behavior, `allow|ask|deny` semantics, and callback
   request/reply vocabulary. Prove which cases stop before external work.
3. Freeze tool availability. Classify OpenCode/OpenCode Go, Exa, Parallel,
   selected model/provider, environment, API-key, hosted-service, checksum or
   fallback effects. Do not infer availability from registry presence.
4. Trace exact prompt execution and observations. Identify evidence for tool
   invocation, permission request, outbound provider call, result/error, SSE
   activity, terminal outcome, and abort. Split requested, admitted,
   dispatched, accepted, observed, and effective truth.
5. Audit production prepared structured-run and interactive-session inputs,
   policy/plan/evidence, session-create JSON, permission callbacks, validation,
   events, activity, usage, cancellation, deletion, cleanup, fixtures, guide,
   and public API. Name the smallest safe route-local delta.
6. Determine whether exact tool/backend availability can be bound from
   host-approved prepared evidence. Treat ambient attached-server config and
   environment as untrusted unless the existing route validates them exactly.
7. Classify callback-free and callback-enabled structured/interactive profiles
   separately. A wildcard `ask` fallback does not prequalify web search.
8. Prove omission keeps current session-create JSON and policy behavior.
   Determine the exact compatible `ExternalNetworkPolicy` /
   `ExternalSearchPolicy` pair and any required permission row/action.
9. Select any required adapter-private behavior or claim revision. Do not
   change the Contract 029 ceiling in this feature lane.
10. Promote Research 214 with an exact deliver-now table or explicit empty set.
    Update the milestone/card state and reserved closeout honestly.

## Acceptance Criteria

- [ ] exact official sources, tag, dates, revisions, and hashes are recorded
- [ ] version, provider/backend, tool-availability, permission, policy, and
      profile rows are explicit
- [ ] rule ordering, allow/ask/deny, callback, omission, unknown, fallback,
      and failure truth is settled
- [ ] requested/admitted/dispatched/accepted/observed/effective claims are split
- [ ] existing session JSON, plan/evidence, prompt, callback, event, and
      lifecycle seams are audited
- [ ] Research 214 contains a non-empty exact table or honest empty set
- [ ] no production code, public API, shared contract/runtime, currentness,
      release, merge, rollover, or g04 closure changes
- [ ] `effigy validate:focused swallowtail-adapter-opencode`, `effigy
      qa:northstar`, relevant indexes, and `git diff --check` pass

## Stop Conditions

- exact tagged source, backend availability, provider/model relationship,
  permission semantics, or failure behavior remains ambiguous
- delivery needs a live prompt, account/API-key inspection, ambient config,
  generic permission/tool settings, attached-server mutation, sibling-route
  promotion, shared authority, or a public lifecycle change

## Out Of Scope

- production binding, web fetch, task/subagent permission, arbitrary tools,
  server configuration, live external work, currentness, release, merge,
  rollover, or g04 closure
