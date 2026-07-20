# 001 Swallowtail Vision

Status: active
Owner: Tom
Updated: 2026-07-19

## Intent

Swallowtail is the reusable Rust substrate between applications and AI model or
agent harness providers.

It should let a host application discover provider readiness, inspect models,
open interactive sessions, run bounded structured work, exchange events and
tool calls, cancel work, resume where supported, and report safe diagnostics
without rebuilding each connector from scratch.

## Product Boundary

Swallowtail owns mechanism:

- provider and adapter identity
- readiness and capability discovery
- model catalog normalization
- interactive session and structured-run lifecycles
- event, tool-call, cancellation, and resume transport
- provider reference opacity
- safe, structured diagnostics
- adapter conformance support
- harness and direct-inference execution layers
- supported authentication, entitlement, and readiness mechanisms

Host applications own intent:

- system prompts and agent instructions
- product tools and their authority
- task, goal, memory, project, and review semantics
- credential policy and durable product state
- UI, operator workflow, and scheduling policy
- the choice of local or remote execution host

## Experience Goal

A consumer should integrate one small, capability-led surface, then retain
provider-specific power where it matters. Supporting a new provider should not
require copying process supervision, event translation, cancellation, and
diagnostics into every application.

## Integration Ecosystem

Swallowtail is not a Codex abstraction. It is an ecosystem of provider- and
transport-specific adapters behind a small shared runtime vocabulary.

Coverage should extend across available agent harnesses and direct model routes
through every useful supported surface: CLI, structured CLI, app-server/RPC,
ACP, SDK, HTTP API, WebSocket, local runtime, remote service, and future
provider-specific protocols.

Each surface may require its own implementation. Swallowtail shares lifecycle
mechanisms and records without pretending that Codex, Claude Code, OpenCode,
Cursor, Pi, Kimi, xAI/Grok, local runtimes, or later integrations behave alike.

Integration family, adapter driver, transport family, configured instance, and
model route remain distinct identities. One family may expose several drivers;
one model may be reachable through several families or instances.

Open-weight model artifacts add another boundary: GLM, Qwen, DeepSeek, and
later artifacts may be served through several local or remote runtimes. Artifact
identity, serving driver, deployment, protocol facade, and route remain
separate. Compatible APIs do not imply identical runtime behavior.

## Execution Shapes

Swallowtail has two execution layers:

1. **Harness interaction** — a provider-owned agent loop or lifecycle.
2. **Direct model inference** — a model endpoint or local runtime while the
   consumer owns orchestration.

These are independent from the operation shapes below.

Swallowtail treats these as related but distinct:

1. **Interactive session** — durable conversational continuity, streamed
   events, tool exchange, interruption, and optional resume.
2. **Structured run** — bounded input, optional schema-constrained output,
   progress, cancellation, and a terminal result.

Neither is forced through a lowest-common-denominator `send_prompt` API.

## Access

Every driver instance has an explicit access profile: credential mechanism,
entitlement and metering, endpoint audience, and support authority. API keys
may be subscription-backed; OAuth may authorize direct inference; neither
credential shape determines the execution layer or billing model.

Swallowtail supports provider-published routes and may support routes maintained
by an integration when their lower authority is visible. Experimental routes
remain opt-in. Provider-prohibited routes are excluded. Consumers choose
whether and when to cross execution, credential, entitlement, endpoint,
billing, privacy, support-authority, or topology boundaries.

## Topology

Local desktop execution is not the default assumption. A consumer may place
the execution host beside a server, inside a worker, or on the client. Provider
adapters operate where credentials, binaries, SDKs, and filesystem authority
actually live.

## Success

- Nucleus and Soundcheck share real connector infrastructure without sharing
  product policy.
- several materially different harness, API, SDK, CLI, and local-runtime shapes
  fit through capabilities and extensions without breaking core vocabulary.
- adding a new driver does not require weakening native lifecycle features of
  existing drivers.
- consumers can test failure, cancellation, and event handling using fixtures
  rather than real accounts.
- provider payloads and secrets do not leak into stable public diagnostics.
- the crate graph remains usable without Tauri, Nucleus, or Soundcheck.
