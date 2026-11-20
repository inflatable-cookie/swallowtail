Fixed remaining shell-gate, CI, and `swallowtail-testkit` hygiene issues.

- `scripts/check-release-floor.sh` captures `rustup toolchain list`, escapes dots in `SWALLOWTAIL_MSRV`, and matches with `grep -Eq` (matching `scripts/check-msrv.sh`, avoiding `pipefail`/SIGPIPE races and `rg`).
- `scripts/validation/archive.sh` distinguishes `rg` exit codes `0`, `1`, and `>= 2` in `validation_archive_member_list_is_safe` and escapes regex metacharacters in `$validation_repo_root`.
- `scripts/check-public-api.sh` treats only `grep` exit code `1` as empty removals and fails on `>= 2`.
- `scripts/check-provider-route-matrix.sh` permits `immutable_routes <= current_routes` when routes do not grow on patch/same-route states while still requiring strict extension when candidate routes add entries.
- `scripts/tests/validation-selectors.sh` uses `${TMPDIR:-/tmp}`, cleans up temporary directories with an `EXIT` trap, resolves `mktemp` and `tar` via `PATH` rather than `/usr/bin/*`, and covers the `rg` failure and regex-escaping paths.
- `scripts/tests/oxide-selector-routing.py` uses stdlib `tomllib` when available (with a Python `< 3.11` parser fallback for hosts on Python 3.9) and removes the dead `sdk240-host-setup.yml` workflow check.
- `scripts/verify-source-consumer.sh` updates its stale workspace crate count comment.
- `.github/workflows/ci.yml` adds explicit `timeout-minutes` to every job and uses `dtolnay/rust-toolchain@stable` consistently instead of `@master`.
- `crates/swallowtail-testkit` runs `BoundSelection` preflight checks per profile while keeping profile-independent shared-contract assertions single-pass, records process I/O calls internally and honors `RecordingOutcome::Fail` in `RecordingProcessHandle::{write_stdin,close_stdin,read_output}` without altering the `RecordedHostCall` public API, aligns `assert_extension_policies` docstring and common-event rejection coverage with `swallowtail_core::ExtensionPolicy` (`Preserve` and `Reject`), and shares `run_all_synthetic_profiles()` via `OnceLock` in `tests/conformance_profiles.rs`.
