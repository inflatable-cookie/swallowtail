# g04.059 Deep Agents ACP Model Selection

Status: stopped
Owner: Tom
Created: 2026-08-24
Updated: 2026-08-24
Depends on: per-route feature completion programme; Research 153, 157, 159
Vision tags: explicit selection, route-local controls, exact dispatch truth
Contract refs: 011, 023, 029, 033, 037, 041, 052
Research: 153, 157, 159, 206

## Problem

Production route `deepagents.acp` owns one exact
`deepagents-acp@0.1.25` child per session but starts it with no extra argv.
Official LangChain documentation advertises `--model <model>` and a
`provider:model` model string. The route therefore leaves a credible
server-start selection surface inaccessible while relying on an upstream
default and a host-owned Anthropic or OpenAI API key.

Current documentation is not exact-version proof. It does not settle the
`0.1.25` parser, provider vocabulary, default, fallback, authentication
agreement, or whether ACP confirms the effective model. Those facts must be
frozen before Swallowtail exposes a value.

## Generation Runway Goal

Qualify and, only when exact evidence permits, bind one typed route-local model
selection at `deepagents.acp` child start. Caller omission retains the current
no-extra-argv path. An explicit selection is immutable for the child/session
and cannot change access, working-resource, isolation, permission, tool, or
lifecycle authority.

## Goals

- [x] freeze exact `0.1.25` CLI parser, model grammar, default, provider
      dispatch, invalid-selection, fallback, and authentication evidence
- [x] distinguish current official documentation from exact package source
- [x] identify a bounded provider/model set that agrees with a named
      host-owned provider-key access profile, or stop with an empty set
- [x] distinguish requested, planned, dispatched, accepted, effective, and
      observed model truth
- [x] classify whether initialize, session creation, or events confirm the
      selected model and what exact no-fallback source proof can support
- [x] promote Research 206 with an exact deliver-now table or honest stop
- [ ] bind only admitted rows through typed prepared inputs, immutable
      plan/request agreement, and one exact `--model <value>` pair
      (blocked: Research 206 empty set)
- [ ] prove omission, invalid/mismatched selection, missing/wrong host key,
      cancellation, deadline, terminal failure, and cleanup truth
      (blocked: Research 206 empty set)

## Non-Goals

- a generic provider-settings map, arbitrary argv, or unconstrained string
- a live model catalogue, moving alias claim, quality claim, or provider
  fallback
- selecting `--skills`, `--memory`, `--workspace`, `--name`, `--debug`, or
  `--log-file`
- library embed, `npx`, registry package `0.1.7`, another Deep Agents route,
  or a new route family
- credential capture, lease materialization, API-key injection, login,
  provider prompt, external inference request, or paid work
- filesystem or descendant-process containment claims; the route remains
  `AmbientHost`
- permission, tool, MCP, resource-access, persistence, continuation, or
  restoration expansion
- currentness, release, merge, generation rollover, or g04 closure

## Named Scope

The lane is restricted to route `deepagents.acp`, driver
`swallowtail.deepagents.acp`, axis `deepagents-acp.package`, and exact
qualified npm package `0.1.25`. The current route is `QualifiedOnly`;
`UnverifiedNewer` does not inherit selection.

Card 164 must inspect the exact published tarball/source and freeze the full
`--model` parser path. It must record the precise value grammar, provider
prefixes accepted by that version, whether the model suffix is passed through,
the omission default, aliases, normalization, unknown-provider/model behavior,
and every fallback path. Current official docs are leads, not substitutes.

Model selection must agree with the prepared access profile before process
spawn. A value that selects Anthropic cannot be prepared from OpenAI-only
access evidence, or vice versa. Host-owned key bytes remain outside
Swallowtail. Missing or rejected provider credentials remain provider/auth
failure, never a signal to retry another model or provider.

Each prepared session owns one child, so an admitted selection is fixed at
server start and cannot vary per turn. There is no load/resume binding. Fresh
working-state restoration starts a new context-losing child and may carry the
same prepared selection only when the exact lifecycle and access evidence
remain valid.

Dispatch does not automatically prove effective selection. Research 206 must
freeze any exact ACP confirmation field, or explicitly bound a narrower
dispatch-only claim with exact source proof that the CLI cannot substitute or
fall back. If neither is possible, the deliver-now set is empty.

## Execution Plan

### Batch 59.1 — Exact Model Evidence

- [x] Execute card 164.
- [x] freeze source, grammar, provider, auth, fallback, confirmation, and
      lifecycle evidence
- [x] promote Research 206 with an exact deliver-now table or empty set

### Batch 59.2 — Conditional Prepared Binding

- [ ] Execute card 165 only when card 164 admits a non-empty deliver-now set.
      Blocked: Research 206 empty set.
- [ ] bind the smallest typed provider/model surface through preparation,
      plan, request, child command, and restoration agreement

### Batch 59.3 — Route-Local Acceptance

- [ ] Execute card 166 only after card 165. Blocked: card 165 not executed.
- [ ] prove exact argv, omission, mismatch, failure, lifecycle, and docs truth

## Acceptance Criteria

- [x] only Research 206 deliver-now rows prepare (vacuous: empty set; no
      binding)
- [ ] route, package, provider, model, access profile, plan, request, and argv
      agree before spawn (blocked)
- [ ] explicit selection emits exactly one `--model <provider:model>` pair and
      omission emits neither token (blocked; omission already emits neither)
- [ ] unknown, malformed, unsupported, or access-mismatched values fail before
      provider effects and never fall back (blocked)
- [ ] effective/observed model is claimed only where exact evidence supports it
      (vacuous: no claim added)
- [ ] one selection is immutable for the owned child/session and any admitted
      fresh replacement (blocked)
- [x] working resource, ambient isolation, host-owned credentials, ACP
      permissions, host callbacks, deadline, cancellation, and cleanup remain
      unchanged
- [x] stable diagnostics disclose no key, prompt, output, raw provider payload,
      account identity, endpoint, or host path
- [x] default QA performs no install, login, provider prompt, external
      inference request, credential capture, or paid work
- [x] g04.059 closes only this route-local family; g04 remains active

## Lane Runway

- predecessor: g04.058 Antigravity agent-profile evidence stop
- this milestone: Deep Agents ACP exact model-selection evidence and
  conditional prepared binding
- execution topology: one serial worker lane, cards 164-166
- generation boundary: g04 remains open; no closure or rollover is authorized

## Decision Gates

- Stop if exact `0.1.25` evidence cannot bound the model grammar and provider
  dispatch without private account state or a live provider turn.
- Stop if an invalid, unknown, unavailable, or unauthenticated selection can
  silently substitute a default, alias, provider, or model.
- Stop if provider/model selection cannot agree with one explicit host-owned
  access profile before spawn.
- Stop if delivery requires key materialization, a generic configuration API,
  shared contract/currentness movement, or a breaking public API.
- Stop if a dispatch-only claim cannot be stated honestly under the existing
  contracts and exact source evidence.

## Batch Cards

- [164-deepagents-acp-model-selection-evidence.md](batch-cards/164-deepagents-acp-model-selection-evidence.md)
- [165-deepagents-acp-model-selection-binding.md](batch-cards/165-deepagents-acp-model-selection-binding.md)
- [166-deepagents-acp-model-selection-acceptance.md](batch-cards/166-deepagents-acp-model-selection-acceptance.md)

## References

- [Per-Route Feature Completion Programme](./per-route-feature-completion.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Research 153 Secondary-Wave Source And Disposition](../../research/153-secondary-wave-source-and-disposition.md)
- [Research 157 Deep Agents ACP 0.1.25 Identity](../../research/157-deepagents-acp-0-1-25-identity.md)
- [Research 159 Post-Harness Expansion Currentness](../../research/159-post-harness-expansion-version-currentness-checkpoint.md)
- [Research 206 Deep Agents ACP Model Selection](../../research/206-deepagents-acp-model-selection-evidence.md)
- [Contract 029 Interface Version Qualification](../../contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 033 Harness Configuration Posture](../../contracts/033-harness-configuration-posture.md)
- [Contract 037 Prepared Consumer Integration](../../contracts/037-prepared-consumer-integration.md)
- [Deep Agents ACP Prepared Integration](../../guides/deepagents-acp-prepared-integration.md)
- [LangChain Deep Agents ACP](https://docs.langchain.com/oss/javascript/deepagents/acp)
- [LangChain Deep Agents Models](https://docs.langchain.com/oss/javascript/deepagents/models)
