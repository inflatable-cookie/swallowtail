# g04.058 Antigravity Headless Agent Profile Selection

Status: planned
Owner: Tom
Created: 2026-08-24
Updated: 2026-08-24
Depends on: per-route feature completion programme; Research 079, 080, 177
Vision tags: explicit selection, route-local controls, exact confirmation
Contract refs: 011, 023, 029, 033, 037, 040, 041, 052
Research: 079, 080, 177, 205

## Problem

Production route `antigravity.headless` binds an exact caller-selected model,
optional reasoning effort and structured output, resource access, isolation,
and exact-id continuation. It does not expose Antigravity's separate agent
profile selection even though exact qualified CLI help has long exposed
`--agent` and `agy agents`.

Current official headless documentation for CLI `1.1.17` names `--agent`,
directs callers to `agy agents`, and says a selected agent appears in the
stream-JSON `init.agent` field. That is a credible dispatch-and-confirmation
surface, not yet a Swallowtail claim. Agent names may be built-in, custom,
account-visible, settings-backed, or absent; agent choice may also affect
instructions and tools. Swallowtail must freeze identity, selection,
confirmation, authority, failure, and continuation truth before binding it.

## Generation Runway Goal

Qualify and, only when exact evidence permits, bind one typed route-local agent
profile selection on `antigravity.headless` structured runs and exact-id
continuation. Caller omission retains current argv, plan, model, effort,
schema, sandbox, permission, stream, and lifecycle behavior.

## Goals

- [ ] freeze current official and exact `1.1.9..=1.1.17` evidence for
      `agy agents`, `--agent`, invalid selection, and `init.agent`
- [ ] define the smallest safe profile-id domain and whether values are
      installation/account observations or qualified portable inputs
- [ ] distinguish requested, planned, dispatched, accepted, effective, and
      observed profile truth
- [ ] prove whether selected profiles compose with explicit model, effort,
      schema, read-plan mode, provider sandbox, and ambient permissions
- [ ] classify one-run and exact-id continuation lifetimes separately
- [ ] promote Research 205 with an exact deliver-now table or honest stop
- [ ] bind only admitted rows through typed prepared inputs, immutable plan and
      request agreement, exact `--agent`, and `init.agent` confirmation
- [ ] prove omission, invalid profile, missing/mismatched confirmation,
      cancellation, deadline, terminal failure, and cleanup truth

## Non-Goals

- a generic provider-settings map or arbitrary string passthrough
- creating, editing, importing, deleting, or persisting agent profiles
- translating agent display labels, descriptions, files, prompts, or tools
- treating `agy agents` as a model catalogue or addable-route surface
- changing model, reasoning, schema, resource access, isolation, permission,
  tool, subagent, background-task, or conversation authority
- `--dangerously-skip-permissions`, ambient latest-session continuation, or
  silent fallback to a default agent
- Antigravity 2.0, SDK personas, IDE agents, Gemini CLI, or another route
- login, account mutation, provider prompt, currentness, release, merge,
  generation rollover, or g04 closure

## Named Scope

The lane is restricted to route `antigravity.headless`, driver
`swallowtail.antigravity.headless`, axis `antigravity-cli.release`, exact
qualified CLI `1.1.9..=1.1.17`, and permitted later stable
`UnverifiedNewer` inheritance. The current personal Google subscription access
profile remains unchanged.

Card 161 must freeze the exact plain-text `agy agents` shape, id grammar,
ordering, duplicates, empty output, custom/account visibility, and whether
listing is safe without a provider prompt. It must also freeze `--agent`
parser behavior, one invalid-selection result, and exact `init.agent`
confirmation. Documentation alone does not qualify values.

For structured runs, selection must be operation-private. For continuation,
one selected profile must be immutable on the prepared session and dispatched
on every first, resumed, and fresh-replacement child only if exact evidence
proves that composition. A conversation id does not imply that a profile was
restored. Missing or mismatched `init.agent` fails closed.

Selected profiles do not grant extra Swallowtail authority. Existing
`ResourceAccess`, `HarnessIsolation`, model, effort, schema, host deadline,
provider `request-review` posture, and no-permission-callback behavior remain
authoritative. If a profile cannot compose without widening those boundaries,
the deliver-now set is empty.

## Execution Plan

### Batch 58.1 — Exact Profile Evidence

- [ ] Execute card 161.
- [ ] freeze listing, selection, confirmation, authority, version, failure, and
      lifecycle evidence
- [ ] promote Research 205 with an exact deliver-now table or empty set

### Batch 58.2 — Conditional Prepared Binding

- [ ] Execute card 162 only when card 161 admits a non-empty deliver-now set.
- [ ] bind one typed profile id through prepared run/session inputs, immutable
      evidence, request agreement, command construction, and stream validation

### Batch 58.3 — Route-Local Acceptance

- [ ] Execute card 163 only after card 162.
- [ ] prove exact argv and confirmation on every admitted operation/version
      row while preserving omission, failure, and cleanup truth

## Acceptance Criteria

- [ ] only Research 205 deliver-now rows prepare
- [ ] route, model, profile id, access, isolation, effort, schema, plan,
      request, dispatch, and confirmation agree before successful completion
- [ ] explicit selection emits exactly one `--agent <id>` and omission emits
      neither token
- [ ] `init.agent` exactly confirms every selected child before output is
      accepted; absent or foreign values fail closed
- [ ] invalid, missing, duplicate, stale, or ambiguous profiles do not fall
      back to an ambient/default profile
- [ ] structured-run and continuation lifetimes are claimed only where proved
- [ ] read-plan mode, provider sandbox selection, request-review permission,
      model, effort, schema, host deadline, cancellation, and cleanup remain
      unchanged
- [ ] no raw agent definition, instructions, tool list, provider payload,
      account identity, or path enters a stable diagnostic
- [ ] default QA performs no install, login, account mutation, provider prompt,
      external inference request, credential capture, or paid work
- [ ] g04.058 closes only this route-local family; g04 remains active

## Lane Runway

- predecessor: g04.057 Grok Build ACP reasoning-selection evidence stop
- this milestone: Antigravity headless agent-profile evidence and conditional
  prepared binding
- execution topology: one serial worker lane, cards 161-163
- generation boundary: g04 remains open; no closure or rollover is authorized

## Decision Gates

- Stop if exact evidence cannot freeze a bounded profile-id shape and one
  selected `init.agent` confirmation path.
- Stop if an invalid or unavailable selection falls back silently.
- Stop if profile selection widens resource, permission, tool, isolation,
  provider-session, or account authority beyond the immutable plan.
- Stop if continuation cannot reassert and confirm one immutable profile on
  every child without changing conversation semantics.
- Stop if delivery needs profile-file parsing, account mutation, a generic
  configuration API, contract/currentness movement, or a breaking public API.

## Batch Cards

- [161-antigravity-headless-agent-profile-evidence.md](batch-cards/161-antigravity-headless-agent-profile-evidence.md) — ready
- [162-antigravity-headless-agent-profile-binding.md](batch-cards/162-antigravity-headless-agent-profile-binding.md) — conditional
- [163-antigravity-headless-agent-profile-acceptance.md](batch-cards/163-antigravity-headless-agent-profile-acceptance.md) — conditional

## References

- [Per-Route Feature Completion Programme](./per-route-feature-completion.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Research 079 Antigravity Headless Stream Qualification](../../research/079-antigravity-cli-1-1-9-headless-stream-qualification.md)
- [Research 080 Antigravity Exact Conversation Continuation](../../research/080-antigravity-cli-1-1-9-exact-conversation-continuation.md)
- [Research 177 Antigravity 1.1.17 Identity](../../research/177-antigravity-1-1-17-identity.md)
- [Research 205 Antigravity Agent-Profile Evidence](../../research/205-antigravity-headless-agent-profile-evidence.md)
- [Contract 029 Interface Version Qualification](../../contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 033 Harness Configuration Posture](../../contracts/033-harness-configuration-posture.md)
- [Contract 037 Prepared Consumer Integration](../../contracts/037-prepared-consumer-integration.md)
- [Contract 040 Generation Controls](../../contracts/040-generation-control-application-and-enforcement.md)
- [Antigravity Prepared Integration](../../guides/antigravity-prepared-integration.md)
- [Antigravity Headless Mode](https://antigravity.google/docs/cli/headless/)
