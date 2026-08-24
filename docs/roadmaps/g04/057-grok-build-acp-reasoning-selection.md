# g04.057 Grok Build ACP Reasoning Selection

Status: stopped
Owner: Tom
Created: 2026-08-24
Updated: 2026-08-24
Depends on: per-route feature completion programme; Research 130 and 163
Vision tags: explicit selection, negotiated harness options, route-local controls
Contract refs: 011, 017, 023, 029, 034, 037, 040, 041, 052
Research: 130, 163, 204

## Problem

Production route `grok-build.acp` binds one exact model per executable behavior
and exposes structured-run and interactive-session shapes over ACP v1. Existing
exact handshakes record bounded effort sets, but consumers cannot select one:
interactive preparation rejects non-empty `SessionOptions`, and structured-run
preparation exposes no reasoning input.

Current official ACP docs and the stdio example open a session with `cwd` and
empty MCP servers. They do not document an ACP effort field. A planning triage
note attributed open/resume effort selection to the 1.0.x changelog; the frozen
changelog pages do not contain that sentence. Exact 1.0.5 binaries apply an
open-time `_meta.reasoningEffort` hint with fail-open ignore; 1.0.4 has the
parser without that new-session path. Neither is Contract 034 confirmation.
Swallowtail must freeze the exact option snapshot, private option id and values,
selection request, effective confirmation, lifetime, and version/model mapping
before enabling portable `ReasoningSelection`.

## Generation Runway Goal

Qualify and, only when exact evidence permits, bind model-qualified reasoning
selection for new `grok-build.acp` sessions and the operation-private session
used by a structured run. Empty options and omitted run reasoning retain the
current wire and behavior.

## Goals

- [x] freeze current official and exact package/source/handshake evidence for
      ACP effort advertisement, selection, confirmation, defaults, and lifetime
- [x] classify deprecated `0.2.114..=0.2.117`, maintained `1.0.4..=1.0.5`,
      permitted later stable points, and incompatible gaps separately
- [x] bind every candidate value to the exact route-fixed model
- [x] distinguish advertised, selectable, requested, accepted, effective, and
      observed reasoning truth
- [x] promote Research 204 with an exact deliver-now table or honest stop
- [ ] map only admitted values through portable `ReasoningSelection`, immutable
      plan/evidence, request options/policy, and adapter-private ACP values
- [ ] require exact effective confirmation before returning a ready session or
      sending the structured run's first prompt
- [x] preserve omission, working-resource, access, callback, cancellation,
      provider-retention, attachment, and cleanup behavior
- [ ] prove malformed, ambiguous, rejected, missing-confirmation, drift, and
      post-allocation failures without a live prompt

## Non-Goals

- arbitrary provider configuration, raw option snapshots, or public string maps
- model switching, model catalogue expansion, or provider default inference
- reasoning changes after readiness or per-turn effort mutation
- selection on load, resume, or attachment recovery without separate exact proof
- `--effort` child argv, headless `--max-turns`, web-search, plan, subagent,
  sandbox, permission, allow/deny, or approval controls
- hosted xAI Responses or Realtime reasoning
- usage, cost, output bounds, structured output, attachments, consumer tools,
  permission/question response, or provider-session management
- credential/account work, authenticated prompt, release, currentness,
  publication, merge, generation rollover, or g04 closure

## Named Scope

The lane is restricted to route `grok-build.acp`, driver
`swallowtail.grok-build.acp`, axis `grok-build.executable`, ACP v1 stdio, and
the existing delegated subscription access profile. Exact deprecated versions
`0.2.114..=0.2.117` bind `grok-4.5`; maintained `1.0.4` and `1.0.5` bind
`grok-4.6` but differ on new-session effort application. Mid-gap
`0.2.118..=0.2.121` and unprobed `1.0.0..=1.0.3` remain incompatible.

The initial candidates come only from frozen exact handshakes:

- `grok-4.5`: `low`, `medium`, `high`
- `grok-4.6`: `low`, `medium`, `high`, `xhigh`

Those rows are not prequalified. Card 158 must prove the ACP configuration
surface required by Contract 034. `off`, `minimal`, `max`, aliases, nearest-
value fallback, and provider defaults stay withheld unless exact evidence
admits them.

The existing structured-run route creates one operation-private provider
session before its single prompt, so an exact new-session mapping may compose
with both public shapes. It must not be copied onto attachment recovery:
Contract 034 does not authorize mutation of a previously persisted session.

## Execution Plan

### Batch 57.1 — Exact ACP Option Evidence

- [x] Execute card 158.
- [x] freeze exact snapshot, request, confirmation, value, model, version,
      omission, failure, and lifetime evidence
- [x] promote Research 204 with an exact deliver-now table or empty set

### Batch 57.2 — Conditional Prepared Binding

- [ ] Execute card 159 only when card 158 admits a non-empty deliver-now set.
- [ ] bind optional reasoning through prepared run/session inputs, immutable
      plan/evidence, request agreement, and adapter-private ACP negotiation

### Batch 57.3 — Route-Local Acceptance

- [ ] Execute card 160 only after card 159.
- [ ] prove exact negotiation, omission, drift, first-prompt ordering, failure,
      and cleanup truth for every admitted route/version/model row

## Acceptance Criteria

- [ ] only Research 204 deliver-now version/model/value rows prepare
- [ ] route-fixed model, portable selection, immutable plan/evidence, request,
      driver mapping, ACP snapshot, selection, and confirmation agree
- [ ] omission preserves exact current wire and lifecycle behavior
- [ ] readiness and first prompt occur only after exact effective confirmation
- [ ] unsupported values and known mismatches reject before provider work
- [ ] post-allocation drift returns no ready handle and joins owned work while
      preserving provider-session retention truth
- [ ] load/resume/attachment recovery are not mutated by inference
- [ ] no raw provider option, model switch, default substitution, or sibling-
      route reasoning claim enters the public API
- [ ] default QA performs no install, login, account inspection, provider prompt,
      external inference request, credential capture, or paid work
- [ ] new tests are split into focused modules and do not increase the doctor
      baseline above 378 findings / 46 errors
- [ ] g04.057 closes only this route-local family; g04 remains active

## Lane Runway

- predecessor: g04.056 llama.cpp owned context size
- this milestone: Grok Build ACP reasoning-selection evidence and conditional
  portable binding
- execution topology: one serial worker lane, cards 158-160
- generation boundary: g04 remains open; no closure or rollover is authorized

## Decision Gates

- Stop if exact evidence does not expose one bounded option snapshot, one
  selectable exact value, and effective confirmation.
- Stop if effort is provider/model-defaulted, clamped, aliased, or silently
  substituted.
- Stop if selection cannot complete after `session/new` and before readiness or
  the structured run's first prompt.
- Stop if delivery requires a generic provider settings map, model switch,
  contract/currentness change, load/resume mutation, or breaking public API.
- Stop if deterministic failure cleanup cannot preserve durable provider-state
  truth after session allocation.

## Batch Cards

- [158-grok-build-acp-reasoning-selection-evidence.md](batch-cards/158-grok-build-acp-reasoning-selection-evidence.md) — complete
- [159-grok-build-acp-reasoning-selection-binding.md](batch-cards/159-grok-build-acp-reasoning-selection-binding.md) — blocked
- [160-grok-build-acp-reasoning-selection-acceptance.md](batch-cards/160-grok-build-acp-reasoning-selection-acceptance.md) — blocked

## References

- [Per-Route Feature Completion Programme](./per-route-feature-completion.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Research 130 Grok 1.0.4 Milestone Handshake](../../research/130-grok-1-0-4-milestone-handshake.md)
- [Research 163 Grok 1.0.5 Identity](../../research/163-grok-1-0-5-identity.md)
- [Research 204 Grok Build ACP Reasoning-Selection Evidence](../../research/204-grok-build-acp-reasoning-selection-evidence.md)
- [Contract 029 Interface Version Qualification](../../contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 034 Negotiated Harness Session Options](../../contracts/034-negotiated-harness-session-options.md)
- [Contract 037 Prepared Consumer Integration](../../contracts/037-prepared-consumer-integration.md)
- [Contract 040 Generation Controls](../../contracts/040-generation-control-application-and-enforcement.md)
- [Contract 052 Consumer And Operator Documentation](../../contracts/052-consumer-and-operator-integration-documentation.md)
- [Grok Build Prepared Integration](../../guides/grok-build-prepared-integration.md)
- [Grok Build CLI Reference](https://docs.x.ai/build/cli/reference)
