# Changelog

All notable Swallowtail changes are recorded here. Releases are source-only
annotated Git tags from the canonical repository.

## [Unreleased]

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
