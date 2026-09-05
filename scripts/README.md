# Scripts

Keep repository automation small and explicit. Prefer Effigy tasks for normal
routing. Add scripts only when a task needs reusable logic that does not belong
in application code.

Normal validation scripts:

- `check-deepseek-harness-corpus.py` — package-independent redacted
  JSON-RPC fixture, lifecycle, correlation, and bound validation for the
  pinned DeepSeek Harness runtime evidence
- `check-deepseek-harness-web-corpus.py` — package-independent redacted Web
  `/api` fixture, loopback trust, method allowlist, history, correlation,
  carrier, and bound validation for the pinned DeepSeek Harness web evidence
- `check-zcode-app-server-corpus.py` — package-independent redacted
  app-server fixture, handshake, lifecycle, correlation, and bound
  validation for the pinned ZCode runtime evidence
- `check-integration-guide-coverage.py` — Contract 052 production-route,
  feature-header, canonical-guide, index, coverage-state, and compiling-example
  traceability behind `effigy qa:guides`
- `check-consumer-front-door.py` — source-install TOML, canonical Git identity,
  release package and route inventories, and support-policy presence behind
  `effigy qa:consumer-docs`
- `check-roadmap-status-drift.py` — batch-card section, milestone annotation,
  and generation-index ready/completed/stop counts against Status frontmatter
  behind `effigy qa:docs:roadmaps:status`. Accepted Status buckets and census
  phrases: `docs/roadmaps/status-grammar.md`
- `check-roadmap-number-collision.py` — unique numbered milestone and
  batch-card files in the working tree, after fetching the advertised
  canonical `https://github.com/inflatable-cookie/swallowtail.git` `main`
  commit into an isolated Git store (sanitized config, no user-repo
  destination ref, tags, or `FETCH_HEAD`). A number already assigned to a
  path on that commit cannot appear on another path. Behind
  `effigy qa:docs:roadmaps:numbers` and CI `roadmap-numbers`.
  Hermetic mutation tests: `effigy qa:docs:roadmaps:numbers:test`
- `check-docs-links.py` — front-door Markdown links plus `docs/research` and
  `docs/logs` bodies behind `effigy qa:docs:links`
- `validate-focused-packages.sh` — one nextest invocation and one
  warnings-denied all-target clippy invocation for one to four explicit
  workspace packages
- `verify-affected-packages.sh` — independently assemble and inspect one to
  four explicit package archives, then compile them through one shared
  extracted target
- `run-with-isolated-home.sh` — run one command under an isolated `HOME` and
  named provider-home variables, restoring the host environment on exit
- `validation/package-scope.sh` — shared exact package argument validation
- `validation/path.sh` — canonical path resolution for macOS `/var` aliases
- `validation/archive.sh` — shared archive member, manifest, and content audit
- `tests/validation-selectors.sh` — deterministic plan and argument failure
  coverage for the focused selectors
- `security:dependencies` — RustSec advisory, allowed-license, and allowed-
  source policy through the repository `deny.toml`

Release-preparation scripts:

- `check-package-metadata.sh` — Contract 036 metadata, package set, MSRV, and
  dependency topology
- `check-release-floor.sh` — lockfile-read-only, warnings-denied Clippy and full
  tests for the unified Rust 1.95 package floor; accepted Effigy release sync
  owns package-aware workspace-member lock updates before gates
- `verify-source-consumer.sh` — isolated external Cargo consumer against exact
  repository `HEAD` when the worktree is clean; dirty pre-commit simulations
  use an explicitly reported synthetic Git snapshot
- `generate-public-api-baseline.sh` — generate the reviewed semantic Rust API
  inventory with pinned `cargo-public-api` and nightly versions
- `check-public-api.sh` — compare the 40-package `v0.4.1` API baseline, while
  forbidding removals from immutable `v0.4.0`. Historical `v0.3.3` and earlier
  files stay immutable
- `check-msrv.sh` — unified Rust 1.95 floor and current stable checks

Archived registry-candidate scripts:

The `v0.1.x` registry-candidate machinery moved to
`release-candidates/0.1.0/scripts/` as frozen evidence: `verify-packages-local.sh`,
`verify-release-candidate.sh`, `verify-candidate-consumers.sh`,
`verify-candidate-provider-facades.sh`, `verify-candidate-provider-lifecycle.sh`,
`verify-packaged-consumer-runtime.sh`, `verify-packaged-provider-runtime.sh`,
and `check-muse-code-corpus.py`. They held stale MSRV and route facts and
re-implemented the shared archive audits; the active source-tag release path
(`config/release.toml` gates through Effigy) never called them. The archived
copies are immutable records, not inputs. `release-package-set.sh` stays live
because `verify-affected-packages.sh` sources its package lists.
