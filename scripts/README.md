# Scripts

Keep repository automation small and explicit. Prefer Effigy tasks for normal
routing. Add scripts only when a task needs reusable logic that does not belong
in application code.

Normal validation scripts:

- `check-integration-guide-coverage.py` — Contract 052 production-route,
  feature-header, canonical-guide, index, coverage-state, and compiling-example
  traceability behind `effigy qa:guides`
- `check-consumer-front-door.py` — source-install TOML, canonical Git identity,
  release package and route inventories, and support-policy presence behind
  `effigy qa:consumer-docs`
- `validate-focused-packages.sh` — one nextest invocation and one
  warnings-denied all-target clippy invocation for one to four explicit
  workspace packages
- `verify-affected-packages.sh` — independently assemble and inspect one to
  four explicit package archives, then compile them through one shared
  extracted target
- `validation/package-scope.sh` — shared exact package argument validation
- `validation/archive.sh` — shared archive member, manifest, and content audit
- `tests/validation-selectors.sh` — deterministic plan and argument failure
  coverage for the focused selectors
- `security:dependencies` — RustSec advisory, allowed-license, and allowed-
  source policy through the repository `deny.toml`

Release-preparation scripts:

- `check-package-metadata.sh` — Contract 036 metadata, package set, MSRV, and
  dependency topology
- `check-release-floor.sh` — warnings-denied Clippy and full tests for the
  Rust 1.90 general package set and Rust 1.94.1 Bedrock exception
- `verify-source-consumer.sh` — isolated external Cargo consumer against exact
  repository `HEAD` when the worktree is clean; dirty pre-commit simulations
  use an explicitly reported synthetic Git snapshot
- `generate-public-api-baseline.sh` — generate the reviewed semantic Rust API
  inventory with pinned `cargo-public-api` and nightly versions
- `check-public-api.sh` — compare 27 immutable release packages plus separately
  baselined unreleased Muse source with reviewed semantic API evidence
- `check-msrv.sh` — general floor, Bedrock floor, and current stable checks

Historical registry-candidate scripts:

- `verify-packages-local.sh` — credential-free package assembly, content audit,
  checksums, extraction, patched local-workspace verification, and packaged
  Kimi local-server protocol/lifecycle execution; an explicit output retains
  one immutable candidate
- `verify-release-candidate.sh` — checksum, source-bundle, and from-source
  package reproducibility
- `verify-candidate-consumers.sh` — isolated Nucleus and Soundcheck prepared-
  runtime tests plus packaged Codex failure/lifecycle conformance against an
  explicit candidate
- `verify-candidate-provider-facades.sh` — execute deterministic prepared
  facade suites for all 23 production routes from one extracted candidate
- `verify-candidate-provider-lifecycle.sh` — execute the exact lifecycle
  matrix and bound-management suites from one extracted candidate
- `verify-packaged-consumer-runtime.sh` — assemble a transient candidate from
  the current source snapshot and run the credential-free cross-consumer
  runtime gate
- `verify-packaged-provider-runtime.sh` — assemble one transient candidate,
  reproduce it from its source bundle, run all packaged provider-facade
  suites, then reuse it for the unchanged Nucleus and Soundcheck runtime proof
- `release-package-set.sh` — one ordered 24-package and current-consumer set

The registry-candidate scripts preserve earlier evidence. They are not called
by the active source-tag release configuration or normal package selectors.

These scripts do not upload, tag, push, read release credentials, change
owners, create releases, or edit consumers.
