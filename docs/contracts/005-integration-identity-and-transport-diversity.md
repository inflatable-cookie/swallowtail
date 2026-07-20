# 005 Integration Identity and Transport Diversity

Status: active
Owner: Tom
Updated: 2026-07-19

## Purpose

Prevent broad provider coverage from collapsing into a false universal harness
or a Codex-shaped runtime.

## Identity Layers

Swallowtail keeps these identities separate:

1. **Integration family** — the named harness, provider, or local runtime.
2. **Adapter driver** — one implementation for one exposed control surface.
3. **Transport family** — CLI, structured CLI, app-server/RPC, ACP, SDK,
   HTTP API, WebSocket, local runtime, remote service, or custom protocol.
4. **Configured instance** — one endpoint, binary, account, credential context,
   version, ownership mode, and execution host.
5. **Model route** — one model selection reachable through an instance.

None can substitute for another. In particular, provider name and model name
are not durable adapter-instance identities.

## Adapter Rules

- Every useful exposed surface may have its own driver.
- Drivers preserve native lifecycle and capability differences.
- A family-level crate may contain several drivers only when their shared code
  and release cadence justify it.
- A direct model API has its own driver and configured instance; models exposed
  through it are model routes. It does not become a harness merely because it
  supports tools, streaming, or provider-side state.
- Protocol compatibility such as ACP does not erase provider-specific
  capabilities, extensions, auth, or recovery behavior.
- Provider extensions remain namespaced and optional outside the owning driver.
- Configured-instance capabilities reflect transport, version, account,
  endpoint, and host reality rather than family-wide promises.
- Execution layer and access-profile dimensions follow Contract 006 and remain
  independent of family, driver, transport, instance, and model-route identity.
- Open-weight artifact, serving-runtime, deployment, facade, and route identity
  follows Contract 007. A model family is not a transport driver.
- Registries resolve explicit driver and instance identity before model route.
- Unsupported capability combinations fail before provider work begins.

## Coverage Posture

Swallowtail aims to support as many useful harness and model-provider surfaces
as practical. Coverage grows through explicit adapter implementations and
conformance evidence, not conditionals in one generic connector.

Initial research candidates include, without limiting later coverage:

- Codex
- Claude Code
- OpenCode
- Cursor
- Pi
- Kimi
- xAI/Grok direct routes
- GLM, Qwen, and DeepSeek hosted and open-weight routes
- local model runtimes, including potential Monkey-backed routes

This list records operator scope, not verified current transport availability.
Official surface and capability research must precede implementation cards.

## Stability Gate

Do not stabilize the shared runtime API from only the two current Codex
consumer paths. Runtime decisions must account for several materially different
integration and transport shapes first.
