# 228 Pi SDK Sidecar Reasoning Selection Evidence

Status: reserved
Owner: Tom
Created: 2026-08-27
Card: g04.081 / 225

## Question

Which exact `pi.sdk-sidecar` provider/model/value/lifecycle rows can expose
portable `ReasoningSelection` without allowing Pi 0.84.2 to clamp, substitute,
default, or restore a different thinking level?

## Method And Boundary

Freeze exact `@earendil-works/pi-coding-agent@0.84.2` documentation, tagged
source, public types, and deterministic local sidecar specimens. Use no
provider prompt, API call, credential, account inspection, package install, or
ambient configuration mutation.

The selected route remains only `pi.sdk-sidecar` with exact Node `22.23.2`,
current source-tagged sidecar, current private wire unless evidence requires a
revision, maintainer-supported delegated harness auth, explicit provider/model,
host-leased working resource, and durable provider-session preservation.

## Evidence To Freeze

- exact `ThinkingLevel` vocabulary and model capability representation
- `clampThinkingLevel` behavior for every input/model class
- static provider/model membership available before process work
- explicit-option precedence over stored/default thinking state
- new/load/resume/replacement/restoration factory behavior
- bootstrap and state snapshot confirmation semantics
- setup/rebind `thinking_level_changed` event ordering
- omission bytes and default/stored-state posture
- current Rust, sidecar, fixture, guide, matrix, and API seams

## Candidate Seam

The source-tagged sidecar already accepts optional bootstrap `thinkingLevel`,
passes it to `createAgentSessionFromServices`, and reports
`session.thinkingLevel`. Rust currently sends no field and validates no
reported value. The compatibility fixture already contains `medium`; it is
wire-shape evidence only.

Official 0.84.2 SDK source states that construction clamps the selected value
to model capabilities. Therefore exact forwarding cannot qualify by itself.
Research must prove a closed pre-effect membership table and compare the
reported effective state with the requested value before readiness.

## Deliver-Now Table

| Provider | Model | Requested mode | Lifecycle | Exact confirmation | Disposition |
| --- | --- | --- | --- | --- | --- |
| pending | pending | pending | pending | pending | pending |

## Required Dispositions

- portable vs adapter-local vocabulary
- exact package/model/value gate
- requested, dispatched, accepted, effective, and observed states
- new, load, resume, replacement, and fresh restoration
- wire/behavior/source-tag revision
- omission, default, stored-state, clamp, substitution, and model fallback
- public API and compatibility baseline

## Promotion Gate

Promote a non-empty row only when exact static provider/model/value membership
permits rejection before effects and `session.thinkingLevel` confirms the same
value before readiness for the claimed lifecycle. Otherwise promote an honest
empty set and block cards 226-227.

## Sources

- [Research 181 Pi SDK Sidecar Route Qualification](./181-pi-sdk-sidecar-route-qualification.md)
- [Pi 0.84.2 SDK guide](https://github.com/earendil-works/pi/blob/v0.84.2/packages/coding-agent/docs/sdk.md)
- [Pi 0.84.2 SDK construction](https://github.com/earendil-works/pi/blob/v0.84.2/packages/coding-agent/src/core/sdk.ts)
- [Pi 0.84.2 AgentSession](https://github.com/earendil-works/pi/blob/v0.84.2/packages/coding-agent/src/core/agent-session.ts)
- [Contract 012 Interactive Session Options](../contracts/012-interactive-session-options-and-callback-exchange.md)
- [Contract 040 Generation Controls](../contracts/040-generation-control-application-and-enforcement.md)
