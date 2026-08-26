# g04.078 llama.cpp Owned Reasoning Controls

Status: ready
Owner: Tom
Created: 2026-08-26
Updated: 2026-08-26
Depends on: g04.056; g04.077 closeout; per-route feature completion programme
Vision tags: explicit behavior, owned local runtime, route-local controls
Contract refs: 010, 011, 023, 024, 029, 037, 040, 041, 052
Research: 003, 203, 225

## Problem

Production route `llama-cpp.owned` owns an exact llama.cpp server child at
`b10069-178a6c449`, but exposes only context-size selection. The exact server
documents `--reasoning on|off|auto` and `--reasoning-budget -1|0|N`; consumers
cannot select either through typed preparation.

These flags are serving controls, not portable model reasoning. Their useful
behavior depends on the operator-supplied GGUF, detected chat template,
reasoning tags, parser configuration, and server semantics. A successful parse
does not prove that a selected model applies the control or that inference
output changes.

## Generation Runway Goal

Qualify and, only when exact evidence permits, bind a closed adapter-local
reasoning selection and budget for `llama-cpp.owned`. Preserve omission as the
current no-reasoning-argument launch and keep model/template applicability,
serving state, inference, and observation claims exact.

## Goals

- [ ] freeze exact parser, aliases, placement, repetition, precedence,
      defaults, diagnostics, and source truth for both candidate flags
- [ ] freeze exact `on|off|auto` and `-1|0|N` semantics, including chat-template
      and reasoning-tag dependencies
- [ ] distinguish requested, prepared, dispatched, parser-accepted, applied,
      effective, and observed state
- [ ] promote Research 225 with an exact deliver-now table or honest empty set
- [ ] conditionally add only closed adapter-local controls supported by that
      table, with no raw string or provider-neutral capability widening
- [ ] preserve no-flag launch bytes, context-size composition, exact runtime
      identity, one-child ownership, readiness, and cleanup
- [ ] prove unsupported model/template, value, version, and stale-evidence rows
      reject before process work when delivery proceeds

## Non-Goals

- portable reasoning effort, thinking budget, or generation-control APIs
- claiming model capability from a server flag, GGUF filename, help text, or
  parser acceptance
- selecting or downloading a model, changing chat templates, reasoning
  formats, reasoning tags, grammars, sampling, or output parsers
- changing `llama-cpp.attached`, adding inference to the owned serving route,
  or changing attached-route reasoning-content rejection
- live model load, prompt, inference, tool execution, account work, paid work,
  currentness, release, merge, generation rollover, or g04 closure

## Named Scope

The lane is restricted to route `llama-cpp.owned`, driver
`swallowtail.llama-cpp.owned-b10069-openai-chat`, compatibility axis
`llama.cpp.owned-runtime`, and exact point `b10069-178a6c449`.

Card 216 must freeze the exact `b10069` source and artifact behavior. The
official exact server README is primary evidence. Current `master` may
corroborate only; it cannot backport semantics. The evidence must cover
`--reasoning`, `--reasoning-budget`, `--reasoning-format`, template capability
detection, reasoning start/end tags, environment/config competitors, repeated
flags, option placement, invalid values, defaults, readiness observation, and
failure or silent-no-op behavior.

The operator supplies the GGUF. Research 225 must therefore name the exact
preflight fact that makes a model/template row eligible. A flag that parses but
can silently do nothing for a non-reasoning template is not a useful
deliver-now row unless the adapter can bind or reject that condition before
process work. `/props` or another prompt-free local channel may qualify only
the state it reports; it must not be promoted into effective inference truth.

Any candidate public control stays adapter-local and closed. Omission must
dispatch neither reasoning flag. A reasoning selection may admit only
`on|off|auto`; a budget may admit only exact documented values and bounds.
Research 225 decides whether either control is independent, coupled, or empty.
No raw provider value enters core or runtime.

## Execution Plan

### Batch 78.1 — Exact llama.cpp Reasoning Evidence

- [ ] Execute card 216.
- [ ] freeze exact parser, precedence, template, application, and observation
      truth
- [ ] promote Research 225 with a non-empty exact table or honest empty set

### Batch 78.2 — Conditional Adapter-Local Binding

- [ ] Execute card 217 only when Research 225 admits a non-empty exact set.
- [ ] bind only admitted controls through typed preparation and canonical argv

### Batch 78.3 — Route-Local Acceptance

- [ ] Execute card 218 only after card 217.
- [ ] prove dispatch, omission, rejection, context composition, and unchanged
      owned-runtime lifecycle truth

## Acceptance Criteria

- [ ] only Research 225 deliver-now rows prepare reasoning controls
- [ ] every control is closed, llama.cpp-local, immutable, and exactly
      dispatched
- [ ] omission retains the exact current launch with no reasoning arguments
- [ ] docs separate server selection from model/template applicability and
      effective inference behavior
- [ ] context size, model path, host/port readiness, process ownership,
      retention, cancellation, terminal, and cleanup truth do not widen
- [ ] default QA performs no model download/load, prompt, inference, tool
      execution, provider work, paid work, or ambient configuration mutation

## Lane Runway

- predecessor: g04.077 Cursor headless Ask delivery
- this milestone: exact llama.cpp owned reasoning evidence and conditional
  binding
- execution topology: one serial worker lane, cards 216-218
- generation boundary: g04 remains open; no closure or rollover is authorized

## Decision Gates

- Stop if exact source cannot prove a useful distinction beyond parser
  acceptance and mutable current documentation.
- Stop if model/template applicability cannot be bound or rejected before
  process work with the route's available evidence.
- Stop if a selected value can silently fall back, become inert, or drift
  through ambient state without an exact disposition.
- Stop if deterministic proof needs a model download/load, provider prompt,
  inference run, paid work, or ambient configuration mutation.
- Stop if delivery needs a portable reasoning capability, shared contract or
  runtime change, sibling-route work, currentness movement, or a breaking API.

## Batch Cards

- [216-llama-cpp-owned-reasoning-controls-evidence.md](batch-cards/216-llama-cpp-owned-reasoning-controls-evidence.md)
- [217-llama-cpp-owned-reasoning-controls-binding.md](batch-cards/217-llama-cpp-owned-reasoning-controls-binding.md)
- [218-llama-cpp-owned-reasoning-controls-acceptance.md](batch-cards/218-llama-cpp-owned-reasoning-controls-acceptance.md)

## References

- [Per-Route Feature Completion Programme](./per-route-feature-completion.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Research 203 llama.cpp Owned Context Size](../../research/203-llama-cpp-owned-context-size-evidence.md)
- [Research 225 llama.cpp Owned Reasoning Controls](../../research/225-llama-cpp-owned-reasoning-controls-evidence.md)
- [Exact llama.cpp b10069 Server README](https://github.com/ggml-org/llama.cpp/blob/b10069/tools/server/README.md)
- [Contract 010 Execution Host Services And Inputs](../../contracts/010-execution-host-services-and-inputs.md)
- [Contract 023 Harness Isolation](../../contracts/023-harness-operation-isolation-and-native-boundary.md)
- [Contract 024 Compatible Chat Codec And Provider Semantics](../../contracts/024-compatible-chat-codec-and-provider-semantics.md)
- [Contract 029 Interface Version Qualification](../../contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 037 Prepared Consumer Integration](../../contracts/037-prepared-consumer-integration.md)
- [Contract 040 Generation Control](../../contracts/040-generation-control-application-and-enforcement.md)
- [llama.cpp Prepared Integration](../../guides/llama-cpp-prepared-integration.md)
