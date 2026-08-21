# 033 Pi SDK Sidecar Route

Status: ready
Owner: Tom
Created: 2026-08-21
Depends on: completed g04.024; Research 181; amended Contracts 019 and 029
Vision tags: provider breadth, persistent sessions, explicit selection
Contract refs: 005, 009, 010, 011, 017, 019, 023, 029, 037, 050, 057
Planning state: ready for worker dispatch
Research: 053, 180, 181

## Problem

`pi.rpc` is a useful production route, but its public wire cannot attach a
stored session to the host-leased working resource required by Contract 017.
Pi's official TypeScript SDK exposes the missing attachment and typed state
surfaces. Swallowtail needs a separate, honestly identified SDK-backed Node
sidecar route rather than weakening attachment or pretending the SDK embeds in
Rust.

## Generation Runway Goal

Realize a fuller Pi route with exact SDK, sidecar, process, resource, and
session truth while retaining the existing RPC route during proof.

## Goals

- [ ] freeze a source-tagged sidecar and strict bounded protocol over Pi's
      public SDK
- [ ] add a separate `pi.sdk-sidecar` driver with exact runtime and package
      qualification
- [ ] preserve the useful fresh-session Pi surface with explicit configuration
- [ ] realize persistent new, load-with-replay, and replay-free resume under
      Contract 017
- [ ] expose the route through prepared and connection-lifecycle surfaces
- [ ] compare the proved SDK route with `pi.rpc` and record explicit
      coexistence or deprecation

## Non-Goals

- changing or patching Pi's RPC wire
- calling the sidecar SDK-native
- direct parsing, copying, rewriting, or trusting Pi session JSONL
- hidden prompt-based context reconstruction
- interrupted-turn, pending-callback, or active-operation recovery
- provider-managed containment, arbitrary shell, or write-tool authority
- package installation, provider login, paid inference, or live account work
- implicit route substitution or removal of `pi.rpc` before evidence

## Named Scope

The first route binds:

- route `pi.sdk-sidecar`
- driver `swallowtail.pi.sdk-sidecar`
- wire `swallowtail-pi-sdk-jsonl-v1`
- exact source-tagged sidecar revision
- exact approved Node runtime satisfying Pi's `>=22.19.0` requirement
- exact `@earendil-works/pi-coding-agent@0.84.2`
- qualified-only one-point SDK claim, separate from the RPC package claim
- explicit provider, model, cwd, credential, provider-state, and read-only tool
  authority
- `AmbientHost` posture without a containment claim

The application provisions the runtime, sidecar entry point, and SDK package
through a host-approved launch recipe. Swallowtail does not install or discover
them.

## Execution Plan

### Batch 33.1 — Sidecar Protocol And Frozen Corpus

- [ ] Execute card 089.
- [ ] implement the smallest source-tagged SDK sidecar over public exports
- [ ] freeze strict correlated frames and deterministic fixtures
- [ ] suppress ambient configuration and automatic work

### Batch 33.2 — Fresh Driver Parity

- [ ] Execute card 090 after card 089.
- [ ] add the separate Rust driver and exact compatibility claim
- [ ] prove explicit fresh-session prompt, steering, follow-up, events, abort,
      close, tools, model, and catalogue behavior

### Batch 33.3 — Persistent Session Attachment

- [ ] Execute card 091 after card 090.
- [ ] realize new, load-with-replay, and replay-free resume
- [ ] prove exact cwd agreement before ready and ordered cleanup

### Batch 33.4 — Route Admission And Acceptance

- [ ] Execute card 092 after card 091.
- [ ] expose prepared and addable connection paths
- [ ] update realized architecture, route/feature matrices, and guides
- [ ] record coexistence or explicit RPC deprecation from evidence

## Acceptance Criteria

- [ ] `pi.sdk-sidecar` is independently selectable and never substitutes for
      `pi.rpc`
- [ ] no SDK, Node, or sidecar type leaks into provider-neutral public records
- [ ] no ambient settings, resources, updates, retries, or discovery execute
- [ ] load and resume bind the host-leased cwd before readiness
- [ ] load replay comes from the typed public SDK surface; resume emits none
- [ ] cancellation and close join SDK and process work before lease release
- [ ] default QA is deterministic and performs no install or provider call
- [ ] the production route matrix states the evidence-backed RPC disposition

## Lane Runway

- previous: g04.024 hosted API-key Kimi Platform Chat
- this milestone: full Pi SDK sidecar route
- next: Gemini CLI enterprise API-key requalification
- then: serial per-route feature completion

## Decision Gates

- Stop if the route requires Pi deep imports or direct session-file parsing.
- Stop if ambient resources or automatic network work cannot be disabled.
- Stop if the effective cwd cannot be returned and checked before ready.
- Stop if deterministic QA requires package installation or provider access.
- Stop if SDK/package, Node, sidecar, wire, provider, or route identities must
  be flattened.
- Stop if the launch boundary requires an unpublished package or mutates the
  consuming application's dependency state.
