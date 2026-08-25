# g04.061 Kimi Code ACP Plan Mode

Status: ready
Owner: Tom
Created: 2026-08-25
Updated: 2026-08-25
Depends on: per-route feature completion programme; Research 006, 086, 165, 179, 207
Vision tags: negotiated harness options, plan mode, route-local controls
Contract refs: 012, 017, 023, 029, 034, 037, 040, 041, 052
Research: 006, 086, 165, 179, 207, 208

## Problem

Production route `kimi-code.acp` already consumes the session's bounded ACP
`configOptions` snapshot to negotiate reasoning before readiness. Exact Kimi
Code `0.38.0` source also builds a `mode` select option with
`default|plan|auto|yolo`, maps `plan` to plan mode plus manual permission, and
returns the refreshed option snapshot after selection.

Swallowtail currently rejects every Kimi harness-mode request and does not
inspect or select that option. The existing `HarnessMode::Plan`,
`HarnessModeSelection`, and Contract 034 negotiation boundary can express the
portable request without exposing a generic provider configuration API.

The exact version floor and full confirmation behavior are not yet frozen.
The route must not expose `auto`, `yolo`, provider labels, or a read-only
isolation claim merely because those rows share the provider option.

## Generation Runway Goal

Qualify and bind `HarnessMode::Plan` on new `kimi-code.acp` sessions for the
exact version range that proves current-snapshot membership, one mode-selection
request, provider application, and exact effective confirmation. Preserve
omission, reasoning composition, model selection, access, lifecycle, and
ambient isolation truth.

## Goals

- [ ] freeze the first qualified Kimi Code version with the selected ACP mode
      option, `plan` row, set request, SDK application, and response truth
- [ ] distinguish requested, planned, dispatched, accepted, effective, and
      observed harness-mode state
- [ ] prove the exact `plan` mapping, including manual permission and no
      `auto|yolo` authority widening
- [ ] split the compatibility behavior revision if the mode path begins inside
      the maintained range
- [ ] promote Research 208 with an exact version/value deliver-now table or an
      honest empty set
- [ ] bind only `HarnessMode::Plan` through `SessionOptions`, the immutable
      plan, request, current snapshot, and confirmed response
- [ ] prove composition with every admitted Kimi reasoning selection without
      changing model, permission, resource, or first-prompt ordering
- [ ] keep load, resume, import, and recovery outside harness-mode mutation
- [ ] update route guidance and feature-matrix truth without presenting plan
      mode as process or filesystem containment

## Non-Goals

- provider `default`, `auto`, or `yolo` as public selections
- generic provider configuration, arbitrary option ids/values, display-label
  translation, aliases, fallback, or current-value inference
- permission bypass, automatic approval, tool-policy expansion, or callback
  authority
- `ProviderEnforced` or `HostEnforced` isolation; the route remains
  `AmbientHost`
- changing the selected model, reasoning vocabulary, provider access, working
  resource, retention, or session-management behavior
- plan-mode mutation on load, resume, import, or recovery attachment
- Kimi Code headless/local-server, Python `kimi-cli`, Kimi Platform, or another
  route family
- live OAuth mutation, login, provider prompt, paid inference, currentness,
  release, merge, generation rollover, or g04 closure

## Named Scope

The lane is restricted to route `kimi-code.acp`, driver
`swallowtail.kimi.acp`, axis `kimi-code.executable`, ACP v1 stdio, and the
existing delegated-membership access profile. Exact public source at qualified
`0.38.0` is a lead, not a range claim: card 170 must locate the first exact
qualified version carrying the complete mode-selection behavior and the
immediately preceding boundary.

The only candidate public value is `HarnessMode::Plan`. A new session may
prepare it only when its immutable plan contains `HarnessModeSelection` with
the exact `Plan` constraint. After provider-session allocation, the current
snapshot must contain one unambiguous `mode` select option in category `mode`
whose rows include exact value `plan`. The driver may send only the exact
qualified set request and must require a response or correlated update whose
valid refreshed option reports effective `currentValue = plan` before
readiness.

Provider rows `default`, `auto`, and `yolo` may coexist in an otherwise valid
snapshot. They never become public Swallowtail selections in this lane.
Unknown rows require an explicit Research 208 disposition. Missing, duplicate,
malformed, substituted, rejected, or drifted values fail closed without a
prompt or fallback.

Caller omission preserves the current new-session wire and sends no mode
selection. Plan mode may compose with reasoning only when each requested
option has its own exact plan constraint, current-snapshot membership, one
selection request, and effective confirmation before readiness. Research 208
must settle request ordering and failure cleanup without treating one option's
confirmation as proof of the other.

Load, resume, import, and recovery remain non-mutating. Provider-persisted plan
state may be observed only if an existing typed surface already supports that
truth; it does not authorize Swallowtail to redeclare the mode during
attachment.

## Execution Plan

### Batch 61.1 — Exact Mode Milestone Evidence

- [ ] Execute card 170.
- [ ] freeze exact source/artifact, version milestone, option construction,
      selection, application, confirmation, composition, and lifecycle truth
- [ ] promote Research 208 with a non-empty exact table or honest empty set

### Batch 61.2 — Conditional Plan-Mode Binding

- [ ] Execute card 171 only when Research 208 admits a non-empty set.
- [ ] bind only admitted versions and `HarnessMode::Plan` through the existing
      negotiated session-option surface

### Batch 61.3 — Route-Local Acceptance

- [ ] Execute card 172 only after card 171.
- [ ] prove mode/reasoning composition, rejection, confirmation, lifecycle,
      compatibility, API, guide, matrix, and closeout truth

## Acceptance Criteria

- [ ] only Research 208 deliver-now version/value rows prepare
- [ ] request, immutable plan, current snapshot, provider request, response,
      and effective plan mode agree before readiness
- [ ] `plan` maps to the exact provider behavior proved by source and does not
      grant `auto`, `yolo`, or another permission posture
- [ ] unsupported, absent, ambiguous, malformed, substituted, rejected, or
      drifted values fail closed without a prompt
- [ ] omission and existing reasoning-only wire behavior remain exact
- [ ] every admitted reasoning value composes without shared confirmation or
      fallback
- [ ] load, resume, import, and recovery gain no harness-mode mutation
- [ ] ambient isolation, access, model route, resource, and retention remain
      unchanged and independently visible
- [ ] no generic provider option, permission widening, sibling-route claim,
      or breaking public API appears
- [ ] default QA performs no install, login, OAuth mutation, provider prompt,
      external inference request, credential capture, or paid work
- [ ] new tests remain focused and do not worsen the inherited doctor baseline
- [ ] g04.061 closes only this route-local family; g04 remains active

## Lane Runway

- predecessor: g04.060 Kimi Code ACP catalogue-declared effort levels
- this milestone: exact Kimi ACP negotiated plan mode
- execution topology: one serial worker lane, cards 170-172
- generation boundary: g04 remains open; no closure or rollover is authorized

## Decision Gates

- Stop if exact source cannot identify the first complete mode-selection
  behavior or if the supported range cannot be represented honestly.
- Stop if `plan` is only a display row, is applied without manual permission,
  falls back, or lacks effective response confirmation before readiness.
- Stop if delivery requires public `auto|yolo`, generic configuration,
  permission widening, shared contract/runtime change, a live provider prompt,
  or a breaking public API.
- Stop if plan and reasoning cannot compose with independent exact
  confirmation and joined failure cleanup.

## Batch Cards

- [170-kimi-code-acp-plan-mode-evidence.md](batch-cards/170-kimi-code-acp-plan-mode-evidence.md)
- [171-kimi-code-acp-plan-mode-binding.md](batch-cards/171-kimi-code-acp-plan-mode-binding.md)
- [172-kimi-code-acp-plan-mode-acceptance.md](batch-cards/172-kimi-code-acp-plan-mode-acceptance.md)

## References

- [Per-Route Feature Completion Programme](./per-route-feature-completion.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Research 006 Kimi Code ACP Currentness And Persistent Sessions](../../research/006-kimi-code-acp-currentness-and-persistent-session-evidence.md)
- [Research 179 Kimi Code 0.38.0 Identity](../../research/179-kimi-code-0-38-0-identity.md)
- [Research 207 Kimi Code ACP Extended Effort](../../research/207-kimi-code-acp-extended-effort-evidence.md)
- [Research 208 Kimi Code ACP Plan Mode](../../research/208-kimi-code-acp-plan-mode-evidence.md)
- [Contract 012 Interactive Session Options](../../contracts/012-interactive-session-options-and-callback-exchange.md)
- [Contract 023 Harness Isolation](../../contracts/023-harness-operation-isolation-and-native-boundary.md)
- [Contract 029 Interface Version Qualification](../../contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 034 Negotiated Harness Session Options](../../contracts/034-negotiated-harness-session-options.md)
- [Kimi Code Prepared Integration](../../guides/kimi-prepared-integration.md)
- [Kimi Code exact `0.38.0` ACP mode source](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.38.0/packages/acp-adapter/src/modes.ts)
- [Kimi Code exact `0.38.0` ACP config-option source](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.38.0/packages/acp-adapter/src/config-options.ts)
