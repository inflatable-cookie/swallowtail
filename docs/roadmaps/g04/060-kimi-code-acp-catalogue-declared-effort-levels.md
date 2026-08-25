# g04.060 Kimi Code ACP Catalogue-Declared Effort Levels

Status: ready
Owner: Tom
Created: 2026-08-25
Updated: 2026-08-25
Depends on: per-route feature completion programme; Research 006, 086, 165, 179
Vision tags: negotiated harness options, exact effort truth, route-local controls
Contract refs: 011, 023, 029, 034, 037, 040, 041, 052
Research: 006, 086, 165, 179, 207

## Problem

Production route `kimi-code.acp` already negotiates one reasoning selection
from the session's `thinking` config option. Swallowtail accepts
`off|on|low|medium|high`, sends one `session/set_config_option`, and requires
the returned config snapshot to confirm the effective value before readiness.

Exact Kimi Code `0.38.0` source builds that option from the current model's
catalogue-declared `support_efforts`. Its exact tests and model projection admit
declared `xhigh` and `max` rows. Swallowtail's route-local parser rejects those
rows as malformed before checking the caller selection. This leaves an exact,
negotiated per-route feature inaccessible.

The qualified range starts before `0.38.0`. The source milestone that first
made arbitrary model-declared effort rows part of the ACP snapshot is not yet
frozen. Swallowtail must not widen every maintained version or trust an
unbounded provider string without that evidence.

## Generation Runway Goal

Qualify and bind catalogue-declared `xhigh` and `max` reasoning on new
`kimi-code.acp` sessions for the exact version subrange that proves the full
snapshot, selection, and confirmation path. Keep the existing
`off|on|low|medium|high` behavior, model negotiation, load/resume rejection,
access posture, and lifecycle unchanged.

## Goals

- [ ] freeze the exact first qualified Kimi Code version whose ACP `thinking`
      option projects model `support_efforts` without clamping to `high`
- [ ] freeze exact `xhigh` and `max` snapshot, selection, provider dispatch,
      confirmation, fallback, model-change, and lifecycle behavior
- [ ] distinguish catalogue declaration, requested, dispatched, accepted,
      effective, and observed effort truth
- [ ] split the compatibility behavior revision if older qualified versions do
      not support the extended rows
- [ ] promote Research 207 with an exact version/model/value deliver-now table
      or honest stop
- [ ] admit only `xhigh` and `max` when the current exact session snapshot
      advertises the requested value
- [ ] require the returned exact config snapshot to retain the option shape and
      confirm the selected effective value before returning a session
- [ ] prove omission, legacy values, boolean `on|off`, unsupported values,
      malformed/ambiguous snapshots, drift, cancellation, and cleanup truth
- [ ] update route guidance and feature-inventory disposition without changing
      the existing matrix capability cell

## Non-Goals

- arbitrary effort strings, display labels, aliases, nearest-value fallback,
  provider defaults, or a generic configuration map
- model selection, model catalogue publication, model-quality claims, or
  inference from a model name
- reasoning selection on load, resume, import, or attachment recovery; those
  lifecycles continue to reject redeclaration
- Kimi Code headless, local-server, Python `kimi-cli`, Kimi Platform, or
  another route family
- plan mode, YOLO/AFK, permissions, questions, tools, search, output bounds,
  context controls, subagents, or filesystem authority
- live OAuth mutation, login, provider prompt, paid inference, currentness,
  release, merge, generation rollover, or g04 closure

## Named Scope

The lane is restricted to route `kimi-code.acp`, driver
`swallowtail.kimi.acp`, axis `kimi-code.executable`, ACP v1 stdio, and the
existing membership OAuth access profile. Exact `0.28.1` remains the deprecated
boolean reasoning point. Exact `0.29.0..=0.38.0` is currently one maintained
declared-effort segment; card 167 must determine whether extended levels require
a later behavior milestone inside that range.

The only new candidate values are portable `ReasoningMode("xhigh")` and
`ReasoningMode("max")`. They are eligible only when the exact session-open
snapshot contains one unambiguous `thinking` select option in category
`thought_level`, its rows include the requested exact value, and the
`session/set_config_option` response repeats a valid option whose
`currentValue` equals that value. A model's display name, alias, provider type,
or presumed capability cannot substitute for the snapshot.

The route may continue to accept `off|on|low|medium|high` exactly as today.
Unknown advertised values must not silently become public selections. Research
207 must decide whether a snapshot containing a foreign value can retain the
known subset safely or must fail as malformed; production behavior follows that
exact disposition.

Selection applies only while opening a new session. Existing load and resume
paths reject a non-empty reasoning option before host effects and remain
unchanged. Provider-session import yields a binding; it does not authorize
reasoning mutation during later load or resume.

## Execution Plan

### Batch 60.1 — Exact Effort Milestone Evidence

- [ ] Execute card 167.
- [ ] freeze exact source/artifact, version milestone, option construction,
      value, selection, confirmation, fallback, and lifecycle evidence
- [ ] promote Research 207 with a non-empty exact table or empty set

### Batch 60.2 — Conditional Parser And Compatibility Binding

- [ ] Execute card 168 only when Research 207 admits a non-empty set.
- [ ] bind only admitted versions and `xhigh|max` through the existing
      negotiated reasoning surface and exact config-option exchange

### Batch 60.3 — Route-Local Acceptance

- [ ] Execute card 169 only after card 168.
- [ ] prove extended and legacy values, rejection, confirmation, lifecycle,
      compatibility, API, guide, matrix-note, and closeout truth

## Acceptance Criteria

- [ ] only Research 207 deliver-now version/value rows prepare
- [ ] exact executable behavior, session snapshot, caller selection, request,
      confirmation, and effective reasoning agree before readiness
- [ ] `xhigh` and `max` are accepted only when advertised by the current model
- [ ] unsupported, absent, ambiguous, malformed, substituted, or drifted values
      fail closed without a prompt
- [ ] omission and existing `off|on|low|medium|high` wire behavior remain exact
- [ ] load, resume, import, and attachment recovery gain no reasoning mutation
- [ ] boolean and always-thinking option shapes remain truthful for admitted
      versions
- [ ] no model-name inference, arbitrary provider option, default substitution,
      sibling-route claim, access widening, or public raw config map appears
- [ ] default QA performs no install, login, OAuth mutation, provider prompt,
      external inference request, credential capture, or paid work
- [ ] new tests remain focused and do not worsen the inherited doctor baseline
- [ ] g04.060 closes only this route-local family; g04 remains active

## Lane Runway

- predecessor: g04.059 Deep Agents ACP model-selection evidence stop
- this milestone: exact Kimi ACP catalogue-declared extended effort levels
- execution topology: one serial worker lane, cards 167-169
- generation boundary: g04 remains open; no closure or rollover is authorized

## Decision Gates

- Stop if exact source cannot identify the first qualified snapshot behavior or
  if the supported range cannot be split honestly under Contract 029.
- Stop if `xhigh` or `max` can be accepted without exact current-model
  advertisement or can be clamped, aliased, substituted, or confirmed only by
  display state.
- Stop if the response lacks effective confirmation before session readiness.
- Stop if delivery requires model selection, raw configuration exposure,
  shared contract/runtime change, OAuth mutation, a live provider prompt, or a
  breaking public API.

## Batch Cards

- [167-kimi-code-acp-extended-effort-evidence.md](batch-cards/167-kimi-code-acp-extended-effort-evidence.md)
- [168-kimi-code-acp-extended-effort-binding.md](batch-cards/168-kimi-code-acp-extended-effort-binding.md)
- [169-kimi-code-acp-extended-effort-acceptance.md](batch-cards/169-kimi-code-acp-extended-effort-acceptance.md)

## References

- [Per-Route Feature Completion Programme](./per-route-feature-completion.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Research 006 Kimi Code ACP Currentness And Persistent Sessions](../../research/006-kimi-code-acp-currentness-and-persistent-session-evidence.md)
- [Research 086 Kimi Code 0.31.1 Range Checkpoint](../../research/086-kimi-code-0-31-1-range-checkpoint.md)
- [Research 165 Kimi Code 0.37.2 Identity](../../research/165-kimi-code-0-37-2-identity.md)
- [Research 179 Kimi Code 0.38.0 Identity](../../research/179-kimi-code-0-38-0-identity.md)
- [Research 207 Kimi Code ACP Extended Effort Evidence](../../research/207-kimi-code-acp-extended-effort-evidence.md)
- [Contract 029 Interface Version Qualification](../../contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 034 Negotiated Harness Session Options](../../contracts/034-negotiated-harness-session-options.md)
- [Contract 040 Generation-Control Application](../../contracts/040-generation-control-application-and-enforcement.md)
- [Kimi Code Prepared Integration](../../guides/kimi-prepared-integration.md)
- [Kimi Code exact `0.38.0` ACP config-option source](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.38.0/packages/acp-adapter/src/config-options.ts)
- [Kimi Code exact `0.38.0` ACP model-catalogue source](https://github.com/MoonshotAI/kimi-code/blob/%40moonshot-ai%2Fkimi-code%400.38.0/packages/acp-adapter/src/model-catalog.ts)
