# g04.067 OpenCode HTTP Web Search

Status: ready
Owner: Tom
Created: 2026-08-26
Depends on: g04.029; per-route feature completion programme
Vision tags: explicit authority, provider truth, attached-route containment
Contract refs: 008, 009, 010, 011, 013, 029, 033, 037, 041, 044, 052
Research: 176, 214

## Problem

Production route `opencode.http` owns exact session creation, prompt, SSE,
permission-callback, structured-run, and interactive-session seams through an
operator-managed server. It currently rejects external search and sends a
deny-first permission ruleset with no `websearch` row.

Exact OpenCode `1.18.20` source contains a native `websearch` tool and a
dedicated `websearch` permission. Tool availability can still depend on the
selected provider, OpenCode service, environment, and hosted search backend.
Permission syntax alone therefore does not prove a runnable or host-authorized
search profile.

## Generation Runway Goal

Qualify and, only when exact evidence permits, bind the smallest OpenCode HTTP
web-search subset through existing shared search/network policy and exact
session permissions. Do not infer tool availability, provider access, callback
approval, network authority, or successful search from one another.

## Goals

- [ ] freeze exact `1.18.20` web-search tool, registry, permission, session,
      provider-selection, environment, request, event, and failure evidence
- [ ] identify exact structured-run and interactive-session rows whose tool
      availability and authority can be bound without ambient inference
- [ ] distinguish search requested, permission admitted, provider request
      dispatched, provider accepted, result observed, and result effective
- [ ] promote Research 214 with a deliver-now table or honest empty set
- [ ] conditionally admit only `ExternalNetworkPolicy::HostApproved` plus
      `ExternalSearchPolicy::Enabled` on Research 214 rows
- [ ] preserve disabled-search request bytes and deny-first session posture
- [ ] prove exact permission ordering, callback behavior, lifecycle, and docs

## Non-Goals

- web fetch, arbitrary URL access, browser control, or consumer-side search
- generic OpenCode tool selection or permission editor
- `task` subagents, agent teams, arbitrary shell/network widening, or writes
- provider/model selection, hosted-search provider selection, environment or
  server configuration, credential injection, or attached-server ownership
- search-result quality, freshness, billing, entitlement, provider acceptance,
  or effective use by the model
- another OpenCode transport, sibling route, currentness, release, merge,
  generation rollover, or g04 closure

## Named Scope

The lane is restricted to route `opencode.http`, driver
`swallowtail.opencode.http`, axis `opencode.server`, exact qualified ceiling
`1.18.20`, and only structured-run or interactive-session rows admitted by
Research 214. No version floor, provider/model, profile, permission action, or
behavior revision is prequalified here.

Card 187 must freeze exact release-tag `websearch` registration and execution,
provider/backend selection, environment gates, session-create permission
evaluation, prompt dispatch, permission events/replies, search/result events,
failure behavior, and the production route's prepared/driver seams. Current
official docs corroborate the tag; moving main does not qualify delivery.

The evidence table must separate tool visibility from permission and both from
network authority. `allow`, `ask`, and `deny` are not interchangeable. A
callback-capable session does not authorize hosted search by itself. An
attached server's ambient configuration or environment is not host-approved
evidence unless Swallowtail can bind and validate the exact required fact.

If delivery proceeds, explicit search must require the existing compatible
shared policy pair and an exact session permission row selected by Research
214. Omission must keep the current policy, session-create JSON, tool posture,
and callback behavior byte-equivalent. Unknown providers, missing tool/backend
availability, drift, and incompatible policies must reject before session or
prompt effects when knowable.

An empty Research 214 deliver-now set is an honest stop.

## Execution Plan

### Batch 67.1 — Exact Web-Search Evidence

- [ ] Execute card 187.
- [ ] freeze exact tool, provider/backend, permission, session, event, policy,
      and failure truth
- [ ] promote Research 214 with a non-empty table or honest empty set

### Batch 67.2 — Conditional Route-Local Binding

- [ ] Execute card 188 only when Research 214 admits a non-empty set.
- [ ] bind only exact operation-policy, profile, permission, and version rows

### Batch 67.3 — Route-Local Acceptance

- [ ] Execute card 189 only after card 188.
- [ ] prove dispatch, omission, rejection, callback composition, lifecycle,
      docs, and API truth

## Acceptance Criteria

- [ ] only Research 214 deliver-now rows prepare
- [ ] operation policy, plan/evidence, session permissions, driver, and prompt
      path agree exactly
- [ ] omission preserves prior request bytes and claims no search authority
- [ ] incompatible policy pairs, profiles, versions, permissions, providers,
      and knowable availability drift reject before effects
- [ ] callback approval never substitutes for network/search authority
- [ ] existing model, reasoning, schema, image, resource, retention, activity,
      usage, cancellation, terminal, delete, and joined-cleanup truth remains
      exact
- [ ] docs claim no provider acceptance, result quality, billing, entitlement,
      or model use beyond frozen evidence
- [ ] default QA performs no credential, login, provider prompt, hosted search,
      external network, or paid work

## Lane Runway

- predecessor: g04.066 Codex Exec model verbosity
- this milestone: OpenCode HTTP web-search evidence and conditional binding
- execution topology: one serial worker lane, cards 187-189
- generation boundary: g04 remains open; no closure or rollover is authorized

## Decision Gates

- Stop if exact tool/backend availability, permission evaluation, selected
  profile, or failure behavior cannot be bound without ambient inference.
- Stop if delivery depends on server/user environment, an account probe, live
  provider prompt, paid search, or uncontrolled provider/model selection.
- Stop if permission approval and network/search authority cannot remain
  separate or disabled-search request bytes cannot remain stable.
- Stop if delivery needs a generic tool/permission surface, shared contract
  change, attached-server configuration, sibling promotion, currentness
  movement, or a breaking lifecycle.

## Batch Cards

- [187-opencode-http-web-search-evidence.md](batch-cards/187-opencode-http-web-search-evidence.md)
- [188-opencode-http-web-search-binding.md](batch-cards/188-opencode-http-web-search-binding.md)
- [189-opencode-http-web-search-acceptance.md](batch-cards/189-opencode-http-web-search-acceptance.md)

## References

- [Per-Route Feature Completion Programme](./per-route-feature-completion.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Research 176 OpenCode HTTP 1.18.20 Identity](../../research/176-opencode-http-1-18-20-identity.md)
- [Research 214 OpenCode HTTP Web Search](../../research/214-opencode-http-web-search-evidence.md)
- [Contract 029 Interface Version Qualification](../../contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 033 Harness Configuration Posture](../../contracts/033-harness-configuration-posture.md)
- [Contract 037 Prepared Consumer Integration](../../contracts/037-prepared-consumer-integration.md)
- [Contract 041 Input, Callback, And Provider-Tool Admission](../../contracts/041-input-callback-and-provider-tool-admission.md)
- [OpenCode Prepared Integration](../../guides/opencode-attached-prepared-integration.md)
- [OpenCode permissions](https://opencode.ai/docs/permissions/)
- [OpenCode tools](https://opencode.ai/docs/tools/)
- [OpenCode v1.18.20 websearch source](https://github.com/anomalyco/opencode/blob/v1.18.20/packages/opencode/src/tool/websearch.ts)
