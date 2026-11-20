Every package now builds one `tests/integration` binary instead of one binary
per test file, and each package carries only the version-identity fixtures its
current claim still needs. The workspace declares 42 test targets, down from 263:
35 `integration` binaries plus the seven process-spawning binaries that
`.config/nextest.toml` already isolates. Superseded per-hop identity suites were
retired, removing about 32,900 lines of static JSON and digest assertions whose
current boundary, exclusion set and latest hop stay covered by the retained
table-driven suites. `effigy.toml` selectors now name `--test integration` with
module-path filters, and the frozen selector digests were refreshed for the five
routes whose package, profile and filter scope did not change.
