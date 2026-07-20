# Codex Exec Capability Expansion

Date: 2026-07-19
Status: recorded

## Result

g01 card 024 is complete.

- Codex exec accepts one bounded image, one JSON Schema output contract, exact
  reasoning selection, explicit external-search policy, and an optional
  deadline only when preflight binds the matching capability and host service
- image and schema arguments are scoped host leases, never consumer paths
- provider invocation ignores ambient user configuration and rules, supports a
  host-approved non-Git resource, denies approvals, prevents tool subprocess
  environment inheritance, and supplies explicit read-only sandbox and
  disabled-or-live web-search configuration
- normal completion cancels the outstanding deadline wait; expiry and operator
  cancellation force-stop and join the owned process as distinct outcomes
- all terminal and startup-failure paths release acquired materializations
- app-server catalog results retain supported reasoning modes and the provider
  default without turning catalog evidence into operation selection

No Soundcheck source or product policy entered Swallowtail.

## Evidence

- current `codex-cli 0.144.6` command help
- current Codex manual configuration reference
- current generated app-server v2 TypeScript schema
- `crates/swallowtail-adapter-codex/src/exec_input.rs`
- `crates/swallowtail-adapter-codex/src/exec_validation.rs`
- `crates/swallowtail-adapter-codex/src/exec_pump.rs`
- `crates/swallowtail-adapter-codex/tests/exec_run.rs`
- `crates/swallowtail-adapter-codex/tests/exec_validation.rs`
- `crates/swallowtail-adapter-codex/tests/exec_deadline.rs`
- full Effigy QA: 75 tests pass

## Next Lane

Card 025 proves the public structured-run parity seam and records the bounded
Soundcheck-owned dependency, replacement, validation, rollback, and legacy
removal sequence without modifying the consumer repository.
