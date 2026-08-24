# 156 llama.cpp Owned Context-Size Binding

Status: complete
Owner: Tom
Created: 2026-08-24
Updated: 2026-08-24
Milestone: [g04.056 llama.cpp Owned Context Size](../056-llama-cpp-owned-context-size.md)
Depends on: card 155; promoted Research 203 with a non-empty deliver-now set

## Goal

Bind only Research 203's exact positive context-size subset through typed
adapter-local owned-serving input, immutable start evidence/specification,
driver validation, and launch construction.

## Scope

1. Add the smallest typed adapter-local selection admitted by Research 203.
   Preserve current constructors and caller-omission behavior.
2. Keep the control inside `swallowtail-adapter-llama-cpp` and the owned-
   serving profile. Do not add a portable context capability, Contract 040
   control, generic settings map, or attached-route behavior.
3. Bind selected values through preparation, immutable safe start evidence or
   specification, configured driver, and launch arguments. Reject knowable
   value, plan, evidence, driver, build, or instance drift before artifact
   acquisition or process start.
4. Emit only exact Research 203 values on exact runtime
   `b10069-178a6c449`. Caller omission must preserve the current argument bytes
   with no `--ctx-size` member. Explicit zero must not alias omission unless
   Research 203 admits it and the roadmap is revised.
5. Preserve model artifact and alias selection, loopback port zero, offline,
   no-UI, no-agent, startup observation, health, properties, catalogue, and
   one-child lifecycle behavior.
6. Preserve local unauthenticated access, host services and deadline, endpoint
   authority, cancellation, failures, diagnostics, stop, and cleanup ordering.
7. Expose only the requested/dispatched/accepted/effective states proved by
   Research 203. Do not infer host feasibility or effective context from
   successful startup.
8. Advance only exact feature-local revisions selected by Research 203.

## Acceptance Criteria

- [x] only Research 203 deliver-now values prepare
- [x] selection, immutable evidence/specification, driver, and argv agree
- [x] caller omission remains byte- and behavior-stable with no context flag
- [x] invalid, zero when withheld, negative, overflow, aliased, and mismatched
      values reject before effects
- [x] artifact, endpoint, readiness, access, deadline, cancellation, failure,
      diagnostics, stop, and release remain unchanged
- [x] no shared runtime control, attached-route behavior, model/hardware-fit,
      effective-context, inference, output, quality, latency, cost, or billing
      claim enters the API

Typed `LlamaCppContextSize` rejects zero, overflow, and non-positive values
before a serving selection can carry them. Omission stays `new(artifact, model)`.
Named validation runs with card 157.

## Validation

```sh
cargo fmt -p swallowtail-adapter-llama-cpp
effigy validate:focused swallowtail-adapter-llama-cpp
effigy package:verify-affected swallowtail-adapter-llama-cpp
effigy package:api
effigy qa:northstar
git diff --check
```

Auto-continue to card 157 when exact preparation, launch, readiness, rejection,
and lifecycle preservation pass.

## Stop Conditions

- implementation needs a shared capability, generic settings map,
  contract/currentness change, or breaking public API
- admitted values cannot remain exact across input, immutable state, driver,
  and launch arguments
- omission, readiness, artifact, process, endpoint, or cleanup truth changes

## Out Of Scope

- route guide, shared closeout, live model work, another build/profile,
  release, publication, merge, generation rollover, or g04 closure
