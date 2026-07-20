# Integration Family and Transport Inventory

Status: completed
Owner: Tom
Updated: 2026-07-19

## Goal

Build a current, official-source inventory of available harness and direct-model
surfaces before Swallowtail chooses runtime traits.

## Governing References

- Contract 004: Runtime Ownership Boundary
- Contract 005: Integration Identity and Transport Diversity
- Contract 006: Execution Layer and Access Boundary
- `docs/vision/001-swallowtail-vision.md`
- `docs/research/001-two-consumer-runtime-requirements.md`
- `docs/research/002-execution-layer-and-access-profile-evidence.md`

## Initial Candidates

- Codex
- Claude Code
- OpenCode
- Cursor
- Pi
- Kimi
- xAI/Grok direct routes
- GLM, Qwen, and DeepSeek hosted and open-weight routes
- Qwen Code and Gemini CLI
- local model runtimes and potential Monkey-backed routes
- additional serious integration families discovered during research

The list is a starting point, not a closed support registry.

## Scope

- use current official documentation, protocol specifications, SDKs, and
  installed CLI evidence where available
- enumerate every useful supported surface per family: CLI, structured CLI,
  app-server/RPC, ACP, SDK, HTTP API, WebSocket, local runtime, remote service,
  and provider-specific protocols
- classify harness and direct-inference capabilities separately; do not add a
  vague hybrid layer when one driver supports both
- record credential mechanism, entitlement and metering, endpoint audience, and
  support authority separately
- include subscription OAuth, subscription-backed API keys, usage-billed keys,
  workload identity, cloud identity, gateways, local access, and custom routes
- record auth, entitlement, discovery, model selection, sessions/runs, tools/callbacks,
  streaming, structured output, cancellation, resume, attachments, remote
  operation, lifecycle ownership, maturity, and extension points
- identify shared mechanisms and irreducible provider-specific behavior
- promote stable identity and capability findings into architecture/contracts

## Out Of Scope

- implementing adapters or runtime traits
- treating unofficial reverse-engineered behavior as a stable contract
- promising support solely from a provider brand or model name
- choosing crate grouping before surface relationships are understood

## Acceptance Criteria

- each claimed surface cites current authoritative evidence
- each family may contain multiple independently identified drivers
- harness control and direct model inference are distinct
- operation shape and access-profile dimensions remain independent from
  execution layer
- provider-supported, integration-maintained, experimental, and prohibited
  routes are distinguished
- prohibited routes are excluded even when technically reproducible
- no fallback silently crosses execution, access, billing, or topology bounds
- transport compatibility does not erase provider-specific capabilities
- configured-instance and model-route differences are explicit
- at least three materially different surface shapes constrain later runtime
  decisions
- uncertainty, experimental status, and unavailable official detail remain
  visible

## Validation

- source links and access dates
- cross-check against installed CLI help where available
- promotion into Swallowtail architecture/contracts
- Swallowtail docs QA and all Markdown links
- `git diff --check`

## Stop Condition

Do not infer a stable transport from marketing copy or a model provider name.
Record uncertain surfaces as research gaps.

## Closeout

- Research 003 inventories seven materially different surface shapes.
- Contract 007 separates open-weight artifacts from serving runtimes,
  deployments, compatible facades, and model routes.
- Installed CLI help cross-checks Codex, Claude Code, OpenCode, and Cursor
  surfaces without inspecting credentials.
- Provider-specific uncertainty and experimental routes remain visible.
