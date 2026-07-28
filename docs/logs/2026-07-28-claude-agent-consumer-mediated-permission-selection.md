# Claude Agent Consumer-Mediated Permission Selection

Date: 2026-07-28

## Evidence

A Figmatic structured conversion halted when Claude Agent attempted to read a
verification gate outside the approved working resource. The ACP bridge sent
`session/request_permission`; Swallowtail correlated it, selected
`reject_once`, cancelled the turn, and returned
`ProviderRequestObserved`.

The fail-closed result matched the current contract. It also proved that a
legitimate promotion pass cannot continue when the consumer is able and
authorized to decide the provider request.

## Decision

- Keep reject-and-stop as the default Claude Agent ACP profile.
- Add an explicit prepared-run opt-in for consumer-mediated permissions.
- Bind the exact `acp/session/request-permission` namespace in the immutable
  run plan.
- Expose only offered one-shot allow and reject options.
- Transport one exactly-once consumer selection without executing the tool or
  choosing approval inside Swallowtail.
- Keep persistent permission options unsupported.

This is the operator-selected first callback conversion ahead of the broader
card 088 inventory. The remaining audit stays open.

## Validation

- `cargo test -p swallowtail-adapter-claude-agent`
- `cargo clippy -p swallowtail-adapter-claude-agent --all-targets -- -D warnings`
- `effigy package:api`
- `effigy qa:routes`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy format:check`
- `git diff --check`
