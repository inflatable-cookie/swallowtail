# 2026-07-25 Ollama Attached Prepared Facade

Status: complete

## Changed

`swallowtail-adapter-ollama` now exposes a prepared normal path for an
operator-managed native runtime:

- `prepare_ollama_attached` observes and binds one approved deployment
- `prepare_inventory` keeps installed, running, and selected-detail evidence
  separate
- `prepare_inference_attempt` derives one exact route-bound native request
- `observe_inventory` and `start_run` delegate to the existing low-level role

The prepared integration retains configured-instance identity, opaque endpoint
target, host, access provenance, exact runtime compatibility, selected route,
native model tag, manifest digest, available host services, and the low-level
driver escape hatch.

## Authority

Swallowtail owns neither the Ollama server nor its model artifacts. Preparation
does not install, pull, unload, start, stop, or mutate anything. Invocation may
change runtime-managed model residency, but that effect grants no restoration,
duration, exclusive-capacity, process, or serving authority.

The consumer still selects the endpoint, route, native tag, expected digest,
host, access evidence, content, output bound, deadline, and every attempt.

## Version Posture

Research 035 and corrected Contract 031 align Ollama with the project-wide
forward-compatibility rule. `0.14.0` through `0.32.1` remains the guaranteed
window. Exact `0.32.2` remains excluded by current evidence. Semantic
prereleases fail. Later exact stable versions may proceed visibly unverified
through the latest qualified behavior and must pass the same drift and protocol
checks.

## Validation

- complete Ollama all-target suite: 28 deterministic tests pass; one
  operator-gated live probe ignored
- prepared fixtures: four pass across local and remote-authoritative hosts
- attached-runtime conformance, exact exclusion, unverified-newer execution,
  cancellation, endpoint drift, and runtime drift: pass
- compile-tested example and warnings-denied Rust lint: pass
- Doctor: unchanged at 19 pre-existing oversized-file findings

## Next

Card 023 reviews Kimi ACP, Anthropic direct, and Ollama native together before
the remaining adapter rollout.
