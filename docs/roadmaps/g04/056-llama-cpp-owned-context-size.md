# g04.056 llama.cpp Owned Context Size

Status: planned; card 155 ready
Owner: Tom
Created: 2026-08-24
Updated: 2026-08-24
Depends on: per-route feature completion programme; Research 008
Vision tags: explicit selection, bounded serving, route-local controls
Contract refs: 007, 008, 011, 018, 029, 037, 040, 052
Research: 008; 203 reserved by card 155

## Problem

Production route `llama-cpp.owned` launches exact `llama-server` `b10069`
without `--ctx-size`, so the runtime uses its model-derived default. A caller
that owns the ephemeral serving profile cannot request a smaller explicit
context allocation before the model loads.

Exact tagged documentation exposes `-c, --ctx-size N`, with `0` meaning loaded
from model metadata. Parser breadth, allocation feasibility, model training
context, and observed effective context are not yet qualified. The route also
has no inference role: any selection belongs to owned serving, not a message
composer or attached inference request.

## Generation Runway Goal

Qualify and, only when exact evidence permits, bind a typed positive context-
size selection for exact owned runtime `b10069-178a6c449`. Caller omission must
retain the current launch command with no `--ctx-size` argument.

## Goals

- [ ] freeze current official and exact `b10069` parser, storage, model-clamp,
      allocation, readiness, property, and failure evidence
- [ ] classify omission, explicit zero, positive values, negatives, overflow,
      values above model training context, and resource-infeasible values
- [ ] distinguish requested, dispatched, accepted, effective, and observed
      context-size truth
- [ ] decide whether `/props`, another bounded readiness field, or no selected
      surface confirms the applied value
- [ ] keep the control inside the `llama-cpp.owned` serving profile
- [ ] promote Research 203 with an exact deliver-now table or honest stop
- [ ] bind only admitted values through typed adapter-local input, immutable
      start evidence/specification, driver validation, and launch arguments
- [ ] reject invalid or mismatched values before artifact acquisition or
      process start when knowable
- [ ] prove launch, readiness, failure, stop, endpoint invalidation, and
      artifact release without a live model

## Non-Goals

- a portable context-window capability or Contract 040 generation control
- a message-composer field or `llama-cpp.attached` request control
- automatic choice from model metadata, catalogue evidence, hardware, memory,
  quantization, or training context
- guaranteed allocation, throughput, latency, output length, quality, or
  effective inference capacity
- reasoning, reasoning budget, prediction limit, batching, GPU, threads,
  parallelism, cache, rope, flash attention, or another server flag
- model download, conversion, licensing, relocation, deletion, or selection
- router mode, persistent serving, public bind, TLS, API keys, UI, agent tools,
  MCP, media, inference, or Monkey lifecycle
- another llama.cpp build, attached route, currentness work, live model work,
  release, publication, merge, generation rollover, or g04 closure

## Named Scope

The lane is restricted to route `llama-cpp.owned`, driver
`swallowtail.llama-cpp.owned-b10069-openai-chat`, axis
`llama.cpp.owned-runtime`, exact opaque point `b10069-178a6c449`, one
operator-supplied GGUF artifact, and the existing offline loopback lifecycle.

The initial public candidate is an explicit positive integer context size.
No numeric ceiling is prequalified by this roadmap. Card 155 must derive a
useful exact domain from tagged parser/source truth and Swallowtail's safe
representation, without treating model or host feasibility as a universal
bound. Caller omission keeps the current no-flag launch. Explicit zero is not
an alias for omission unless exact evidence and roadmap review say otherwise.

The selection is adapter-local serving configuration. Contract 040 explicitly
keeps context windows outside portable generation controls. Contract 018 owns
the artifact, process, endpoint, readiness, and joined-cleanup boundary. The
prepared start's safe evidence/specification must make the selected value
inspectable before effects; the provider-neutral `StartServingRequest` need
not become a generic settings map.

## Execution Plan

### Batch 56.1 — Exact Context-Size Evidence

- [ ] Execute card 155.
- [ ] freeze official and exact-build domain, application, observation, and
      lifecycle evidence
- [ ] promote Research 203 with exact value/profile dispositions

### Batch 56.2 — Conditional Owned-Serving Binding

- [ ] Execute card 156 only when card 155 admits a non-empty deliver-now set.
- [ ] add the smallest typed adapter-local selection and immutable agreement

### Batch 56.3 — Route-Local Acceptance

- [ ] Execute card 157 only after card 156.
- [ ] prove admitted, omitted, rejected, readiness, failure, and cleanup truth

## Acceptance Criteria

- [ ] only Research 203 deliver-now values prepare
- [ ] omission preserves exact current launch arguments and behavior
- [ ] selected input, immutable evidence/specification, driver, and argv agree
- [ ] invalid and knowably mismatched values reject before effects
- [ ] effective context size is claimed only when an exact bounded selected
      surface confirms it
- [ ] artifact, endpoint, process, readiness, stop, and release ordering remain
      unchanged
- [ ] no portable composer, model-fit, hardware-fit, allocation, inference,
      output, quality, latency, cost, or billing claim is introduced
- [ ] default QA performs no download, install, model load, process launch,
      external request, credential, or paid work
- [ ] g04.056 closes only this route-local family; g04 remains active

## Lane Runway

- predecessor: g04.053 Qoder headless maximum turns; g04.054-055 are standing
  currentness and do not move the programme pointer
- this milestone: llama.cpp owned-serving context-size evidence and conditional
  adapter-local binding
- execution topology: one serial worker lane, cards 155-157
- generation boundary: g04 remains open; no closure or rollover is authorized

## Decision Gates

- Stop if a useful safe domain needs a live model, host-specific memory
  policy, or inferred artifact capability.
- Stop if the selected value cannot remain exact and inspectable across
  preparation, immutable start evidence/specification, driver, and argv.
- Stop if acceptance or effectiveness would be inferred from successful model
  load rather than an exact selected surface.
- Stop if delivery needs a shared capability, generic settings map, contract
  change, currentness change, sibling route, or breaking public API.

## Batch Cards

- [155-llama-cpp-owned-context-size-evidence.md](batch-cards/155-llama-cpp-owned-context-size-evidence.md) — ready
- [156-llama-cpp-owned-context-size-binding.md](batch-cards/156-llama-cpp-owned-context-size-binding.md) — conditional
- [157-llama-cpp-owned-context-size-acceptance.md](batch-cards/157-llama-cpp-owned-context-size-acceptance.md) — conditional

## References

- [Per-Route Feature Completion Programme](./per-route-feature-completion.md)
- [Advanced Route Features](../../triage/2026-08-21-advanced-route-features.md)
- [Research 008 Owned llama.cpp Serving Lifecycle](../../research/008-owned-llama-cpp-serving-lifecycle-evidence.md)
- [Research 203 llama.cpp Owned Context-Size Evidence](../../research/203-llama-cpp-owned-context-size-evidence.md)
- [Contract 007 Model Artifact And Serving Boundary](../../contracts/007-model-artifact-and-serving-boundary.md)
- [Contract 008 Runtime Registration And Preflight](../../contracts/008-runtime-registration-and-preflight.md)
- [Contract 011 Runtime Conformance Profiles](../../contracts/011-runtime-conformance-profiles.md)
- [Contract 018 Owned Ephemeral Model Serving](../../contracts/018-owned-ephemeral-model-serving-lifecycle.md)
- [Contract 029 Interface Version Qualification](../../contracts/029-interface-version-qualification-and-compatibility.md)
- [Contract 037 Prepared Consumer Integration](../../contracts/037-prepared-consumer-integration.md)
- [Contract 040 Generation Controls](../../contracts/040-generation-control-application-and-enforcement.md)
- [Contract 052 Consumer And Operator Documentation](../../contracts/052-consumer-and-operator-integration-documentation.md)
- [llama.cpp Prepared Integration](../../guides/llama-cpp-prepared-integration.md)
- [llama-server README at b10069](https://github.com/ggml-org/llama.cpp/blob/b10069/tools/server/README.md)
