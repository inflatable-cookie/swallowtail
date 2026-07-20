# 007 Model Artifact and Serving Boundary

Status: active
Owner: Tom
Updated: 2026-07-19

## Purpose

Keep open-weight model identity separate from the runtime, deployment, and
protocol that make it reachable.

## Identity Boundary

Swallowtail distinguishes:

1. **Model artifact** — a specific checkpoint, format, revision,
   quantization, adapter composition, and license.
2. **Serving driver** — Ollama, llama.cpp, vLLM, SGLang, Monkey, or another
   implementation that loads or reaches artifacts.
3. **Configured deployment** — one process or service with endpoint,
   execution host, version, resource policy, and access context.
4. **Protocol facade** — the native or compatible wire surface exposed by the
   deployment.
5. **Model route** — one selectable model alias and observed capability set
   within that deployment.

A provider-hosted API route and a self-hosted copy of related weights are
different configured instances, usually through different drivers. Shared
model ancestry does not make them interchangeable.

## Capability Rules

- Model-family marketing does not establish runtime capability.
- Effective capabilities are observed at the configured deployment and model
  route.
- Context length, multimodality, thinking or reasoning fields, tool calling,
  structured output, token accounting, caching, batching, cancellation, and
  concurrency may depend on artifact revision, quantization, prompt template,
  parser, runtime, configuration, and hardware.
- OpenAI-, Anthropic-, or Ollama-compatible protocol claims record wire-family
  support only. They do not import the reference provider's full semantics.
- A driver may advertise native extensions alongside a compatible facade.
  Extensions remain namespaced and capability-gated.
- Discovery records observed versioned reality. It does not infer all routes
  from one successful request.

## Access And Topology

- Local inference may require no provider credential, but the execution host
  still owns process authority, filesystem access, network exposure, and
  resource limits.
- Remote self-hosted deployments require an explicit access profile even when
  their underlying model is open weight.
- Artifact license and provenance are distinct from endpoint credential,
  entitlement, and billing metadata.
- Hosted subscription, coding-plan, usage-billed, and self-hosted routes do not
  silently fall back into one another.
- A consumer selects whether local compute, a private deployment, or a hosted
  route is acceptable. Swallowtail reports the boundary before execution.

## Lifecycle Ownership

A serving driver may attach to an existing deployment or launch one when the
host explicitly grants that authority. The configured instance records
ownership mode. Swallowtail never assumes it may stop a service merely because
it discovered it.

Model download, license acceptance, storage placement, eviction, GPU/CPU
allocation, and fleet scheduling are host or adjacent-system policy unless a
later contract explicitly assigns a narrow mechanism to Swallowtail.

## Failure Rules

The runtime distinguishes at least:

- artifact unavailable or incompatible
- license or provenance rejection
- serving runtime unavailable
- deployment not ready
- model route not loaded or not found
- advertised protocol unsupported at the observed version
- template, tool parser, or reasoning parser incompatible
- insufficient memory or compute
- remote access rejected
- required capability absent

Failures remain safe diagnostics and retain driver, instance, and route
identity without leaking secrets or sensitive host paths.

## Deferred Decisions

This contract does not choose download mechanisms, provider-specific health
probes, or Monkey ownership. Spec 002 settles the shared semantics; Contracts
008-011 promote runtime and conformance boundaries against both hosted and
self-hosted drivers.
