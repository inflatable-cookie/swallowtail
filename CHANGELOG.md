# Changelog

All notable Swallowtail changes are recorded here. Releases are source-only
annotated Git tags from the canonical repository.

## [Unreleased]

### Changed
- requalify `claude-code.response-only` for exact Claude Code `2.1.228` only,
  preserving the prepared API, ordinary text projection, empty tool and MCP
  boundary, discarded private-thinking envelopes, and Max/OAuth access without
  an API key

## [0.3.2] - 2026-08-11

### Added
- add the separately selectable `swallowtail-adapter-command-code` package and
  exact Command Code `1.15.1` headless route with plan-mode one-shot runs,
  bounded NDJSON activity and usage, local provider account access, and
  explicit retained interactive continuity through exact session resume
- add the separately selectable `swallowtail-idioms` package with portable
  idiom records, deterministic confidence decay and merge, lint, bounded
  selection, fail-soft signal recording, a static-rules backend, and registry
  pull/push merge without transport authority
- add opt-in route-path idiom delivery through runtime host ports, fixed
  caller/provider/route/session folding, capability gating, and Codex
  app-server conformance
- add the exact Claude Code `2.1.227` `claude-code.response-only` route for one
  bounded text response through local Max/OAuth with empty tools and MCP, no
  working resource or retained session, and no structured-output claim

### Changed
- classify exact Claude Code medium-effort thinking-token estimates as
  content-free coalescible progress while validating and discarding the empty
  private-thinking envelope before the single assistant text result
- admit Codex app-server child activity from the exact root
  `subAgentActivity(kind=started)` spawn confirmation without weakening
  fail-closed handling for unobserved child identities
- advance current source to 30 packages and 36 production routes while
  preserving the immutable `v0.3.1` 28-package, 34-route baseline; publish
  [v0.3.2 candidate release notes](docs/releases/0.3.2.md) with upgrade,
  rollback, package, route, and source-only distribution truth

### Fixed
- diagnose approved npm shebang launches that require an exact host-side
  interpreter recipe when ambient `PATH` is intentionally absent
- keep Effigy doctor health routing cheap instead of invoking broad validation

## [0.3.1] - 2026-08-08

### Added
- publish [v0.3.1 release notes](docs/releases/0.3.1.md) for the compatible
  debug-observation and provider-session history patch
- add Contract 054 portable provider-session history pages: plan/request/
  response, plan-bound older cursors, `Exact` / `AtLeast` / `Unknown` totals,
  newest-first window helper, and a read-only driver role distinct from load
  and reconciliation
- prove `codex.app-server` synthetic newest-first history pages over bounded
  `thread/read(includeTurns: true)` without control side effects
- prove `opencode.http` and retained `alibaba.conversations` history pages over
  their existing ascending replay walks without live handles or control side
  effects; runtime history plans accept resource-free DirectModelInference
  posture as well as ambient working-resource harness routes
- document provider-session history pages and the Codex, OpenCode, and Alibaba
  retained mappings; ACP load-as-history routes stay unsupported
- add Contract 053 opt-in debug observation: structured `DebugObservation`
  records, defaulted `DiagnosticObserver::observe_debug`, and fail-soft
  `HostServices` emit helpers for restricted wire and lifecycle context
- emit failure-path debug observations across shared discovery/prep, ACP/RPC,
  headless, hosted HTTP/SSE/WS, realtime, remote ACP, Anthropic managed-agent,
  Ollama, llama.cpp attached/owned, and Bedrock catalogue surfaces without
  changing safe diagnostics
- document host opt-in wiring and the current emitter inventory in the
  debug-observation guide and runtime example

## [0.3.0] - 2026-08-08

### Added
- expose shared runtime helpers for installed-executable discovery, prepared
  plan construction, ordered event emission, and terminal projection
- add exact malformed-inbound diagnostics and regression coverage across the
  Codex app-server boundary

### Changed
- **Breaking:** make `codex_cli_binding` and `ollama_runtime_binding` return
  `Option<InterfaceVersionBinding>` so malformed provider-observed versions
  fail closed instead of reaching an infallible parse path
- centralize repeated adapter discovery, preparation, run-loop, and runtime
  plan-family machinery without changing the 28-package or 34-route set
- publish [v0.3.0 candidate release notes](docs/releases/0.3.0.md) with the
  exact API migration, rollback, and source-only distribution boundary

### Fixed
- bound local-process reader joins, preserve force-stop race truth, and close
  runtime waiter and sender-drop paths without hangs
- enforce remote ACP deadlines and joined worker cleanup across HTTP and
  WebSocket transports
- remove provider-reachable panic paths, reject malformed versions and
  unexpected tool-call shapes safely, and preserve exact adapter diagnostics
- make docs indexes, route inventory, MSRV selection, and literal-only version
  parse checks deterministic release gates

## [0.2.0] - 2026-08-06

### Added
- add the separately selectable `swallowtail-adapter-muse` package and exact
  `muse-code.headless` route for local Meta account access to
  `meta` / `muse-spark-1.2`
- add explicit Muse reasoning effort, prepared read-only execution, bounded
  JSONL activity, exact signed-payload discovery, and operator-gated live
  acceptance evidence

### Changed
- promote the coordinated source release to 28 packages and 34 production
  routes without changing crates.io, GitHub Release, binary, or installer
  posture
- raise the verified Rust floor to `1.95.0` for all packages and retire the
  separate Bedrock override; this breaking requirement makes the release
  `0.2.0`
- split Muse event and corpus internals before release without changing public
  API, diagnostics, or guaranteed behavior
- publish [v0.2.0 release notes](docs/releases/0.2.0.md) with exact upgrade,
  rollback, support, and known-limit guidance

## [0.1.1] - 2026-08-06

### Changed
- publish [v0.1.1 release notes](docs/releases/0.1.1.md) for the compatible
  source-tag repair

### Fixed
- preserve accepted Anthropic Managed Agents cancellation as `Cancelled` when
  cancellation and the operation deadline become ready concurrently
- synchronize Kimi detachment conformance with the fixture peer before
  asserting observer-close evidence under workspace contention

## [0.1.0] - 2026-08-06

### Added
- selected one annotated Git tag as the initial distribution; crates.io,
  GitHub Release assets, binaries, sidecars, and installers are excluded
- coordinated 27 independently selectable Rust library packages at version
  `0.1.0`
- qualified 33 production routes across installed harnesses, attached and
  owned local runtimes, hosted APIs and SDKs, and realtime services
- made adapter-local prepared facades the normal integration path while
  retaining provider-neutral low-level runtime roles
- added explicit configured-instance, model-route, access, host-service,
  policy, preflight, operation, event, terminal, and cleanup evidence
- added model catalogue, structured-run, interactive-session, realtime,
  callback, activity, task-list, subagent, lifecycle, reconciliation,
  restoration, detachment, and failure contracts where routes support them
- kept provider credentials, billing, prompts, tools, routing, retry, fallback,
  persistence, and product UI downstream
- retained exact provider and harness version qualification independently from
  the Swallowtail package version
- established Rust `1.90.0` as the general floor, Rust `1.94.1` for Bedrock,
  and Apple Silicon macOS as the initial verified target
- removed Bedrock's legacy Rustls 0.21 dependency path and automated advisory,
  license, and source policy
- replaced declaration hashing with a pinned 27-package semantic public API
  baseline containing 7,819 normalized entries
- documented every supported public item and made all 27 crates deny missing
  public documentation
- completed canonical guidance and compiling normal-path examples for all 33
  routes and every portable feature family
- added deterministic package, API, documentation, MSRV, security, route,
  guide, example, facade, lifecycle, and external-consumer validation surfaces

### Changed
- Established this tag as the first pre-1.0 public API and guaranteed-behavior
  baseline. Compatible changes advance the patch version; breaking API or
  guaranteed-behavior changes advance the minor version; provider
  qualification remains a separate axis.
- Defined migration from existing path or revision integrations: move every
  direct dependency to the same tag and update the lock file atomically.
  Rollback restores the previous manifests and lock file; `v0.1.0` never moves.
- Added [v0.1.0 release notes](docs/releases/0.1.0.md) covering package
  selection, route inventory, installation, limits, and remaining gates.
