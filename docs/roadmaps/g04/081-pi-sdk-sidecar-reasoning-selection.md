# g04.081 Pi SDK Sidecar Reasoning Selection

Status: stopped after evidence
Owner: Tom
Created: 2026-08-27
Depends on: g04.033; g04.080 closeout; per-route feature completion programme
Vision tags: explicit behavior, SDK sidecar, persistent sessions
Contract refs: 008, 012, 017, 019, 029, 034, 040, 052
Research: 181, 228

## Problem

Production route `pi.sdk-sidecar` exposes exact provider/model selection,
persistent new/load/resume, typed replay, reasoning activity, and a
source-tagged Node sidecar over `@earendil-works/pi-coding-agent@0.84.2`.
Its feature-matrix `reasoning_selection` cell remains `No`.

The private sidecar wire already accepts optional `thinkingLevel`, passes it
to `createAgentSessionFromServices`, and reports `session.thinkingLevel` in
bootstrap and state snapshots. The Rust preparation and driver deliberately
omit and ignore that seam. Pi clamps unsupported values to model capability,
so forwarding a string is not enough: exact model/value membership and
effective state agreement must be proved before any portable claim.

## Generation Runway Goal

Qualify and, only when exact evidence permits, bind immutable portable
`ReasoningSelection` on `pi.sdk-sidecar` across new, load, resume, and fresh
context-losing restoration. Reject any value that Pi would silently clamp or
substitute. Preserve omission, durable provider state, replay, resource and
credential gates, attachments, cancellation, and joined cleanup.

## Goals

- [x] freeze exact `0.84.2` thinking-level vocabulary, model membership,
      clamping, persistence, replacement, and state-confirmation truth
- [x] classify new, load, resume, and fresh restoration independently
- [x] promote Research 228 with a non-empty exact deliver-now table or honest
      empty set
- [ ] conditionally bind only exact model/value/lifecycle rows whose selected
      value is confirmed by `session.thinkingLevel` before readiness
- [ ] preserve selection in immutable request, plan, prepared evidence,
      bootstrap, attachment, and restoration agreement
- [x] keep omission byte-equivalent and make no claim about default depth

## Non-Goals

- changing reasoning after readiness, cycling levels, model switching, raw Pi
  settings, or a generic provider-option map
- inferring effective reasoning from displayed thought, token use, or output
- accepting clamped, defaulted, aliased, or model-family-inferred values
- changing `pi.rpc`, qualifying a newer Pi SDK package, provider prompting,
  credential use, account inspection, release, merge, rollover, or g04 closure

## Named Scope

The lane is restricted to route `pi.sdk-sidecar`, driver
`swallowtail.pi.sdk-sidecar`, exact package
`@earendil-works/pi-coding-agent@0.84.2`, exact Node `22.23.2`, current
source-tagged sidecar, and private wire `swallowtail-pi-sdk-jsonl-v1` unless
Research 228 proves a behavior or wire revision is required.

Card 225 must freeze the exact public SDK types and tagged source behind
`ThinkingLevel`, `clampThinkingLevel`, `createAgentSessionFromServices`,
`AgentSessionRuntime`, session persistence, and `session.thinkingLevel`. It
must identify a closed set of exact provider/model/value rows that preparation
can reject before process or credential work. Catalogue boolean reasoning
metadata, examples, emitted reasoning, and the currently permissive private
wire are leads only.

New session, load, resume, and fresh restoration stay distinct. For load and
resume, the caller re-declares the selected mode under Contract 012; the
sidecar must apply it through the runtime replacement factory and confirm the
effective value before the attached session becomes ready. Stored selection
must not silently win, and a clamp or substitution is a mismatch. Empty
options retain existing stored/default Pi behavior without claiming a
portable selection.

## Execution Plan

### Batch 81.1 — Exact SDK Evidence

- [x] Execute card 225.
- [x] freeze exact model/value/lifecycle and clamp/substitution truth
- [x] promote Research 228 with an exact table or honest empty set

### Batch 81.2 — Conditional Selection Binding

- [ ] Execute card 226 only when Research 228 admits a non-empty exact row.
- [ ] bind only admitted portable reasoning rows through preparation, plan,
      sidecar bootstrap, attachment, and effective-state confirmation

### Batch 81.3 — Route-Local Acceptance

- [ ] Execute card 227 after card 226.
- [ ] prove exact selection, omission, rejection, lifecycle, replay, and
      cleanup truth

## Acceptance Criteria

- [ ] only exact Research 228 provider/model/value/lifecycle rows prepare
- [ ] request, plan, prepared evidence, sidecar command, and reported state
      agree before readiness
- [ ] unsupported values, model drift, clamp, substitution, stored-state
      conflict, and malformed state reject before provider work or readiness
- [x] omission retains exact prior behavior and claims no selected/default
      mode
- [ ] load, resume, and fresh restoration preserve Contract 012/017 meaning
- [x] replay, attachments, cancellation, resource/credential cleanup, and
      durable provider-state posture do not widen
- [x] default QA uses no provider request, credential, account, package
      install, or ambient configuration mutation

## Lane Runway

- predecessor: g04.080 xAI WebSocket web-search evidence stop
- this milestone: exact Pi SDK-sidecar reasoning-selection evidence and
  conditional delivery — stopped after card 225 with Research 228 empty set
- execution topology: one serial worker lane, cards 225-227
- generation boundary: g04 remains open; no closure or rollover is authorized

## Decision Gates

- Stop after card 225 if exact model/value membership cannot be made static
  enough for preparation-time rejection. **Fired.**
- Stop if Pi can clamp or substitute a requested value without a distinct
  effective state that the sidecar can compare before readiness.
- Stop if load/resume cannot reapply and confirm the caller-declared mode
  without mutating an unrelated durable session.
- Stop if delivery needs live provider work, credential inspection, ambient
  settings authority, arbitrary wire options, or a shared-contract change.

## Batch Cards

- [225-pi-sdk-sidecar-reasoning-selection-evidence.md](batch-cards/225-pi-sdk-sidecar-reasoning-selection-evidence.md)
- [226-pi-sdk-sidecar-reasoning-selection-binding.md](batch-cards/226-pi-sdk-sidecar-reasoning-selection-binding.md)
- [227-pi-sdk-sidecar-reasoning-selection-acceptance.md](batch-cards/227-pi-sdk-sidecar-reasoning-selection-acceptance.md)

## References

- [Per-Route Feature Completion Programme](./per-route-feature-completion.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Research 181 Pi SDK Sidecar Route Qualification](../../research/181-pi-sdk-sidecar-route-qualification.md)
- [Research 228 Pi SDK Sidecar Reasoning Selection](../../research/228-pi-sdk-sidecar-reasoning-selection-evidence.md)
- [Pi SDK Sidecar Prepared Integration](../../guides/pi-sdk-sidecar-prepared-integration.md)
- [Contract 012 Interactive Session Options](../../contracts/012-interactive-session-options-and-callback-exchange.md)
- [Contract 040 Generation Controls](../../contracts/040-generation-control-application-and-enforcement.md)
- [Pi 0.84.2 SDK guide](https://github.com/earendil-works/pi/blob/v0.84.2/packages/coding-agent/docs/sdk.md)
- [Pi 0.84.2 SDK construction](https://github.com/earendil-works/pi/blob/v0.84.2/packages/coding-agent/src/core/sdk.ts)
