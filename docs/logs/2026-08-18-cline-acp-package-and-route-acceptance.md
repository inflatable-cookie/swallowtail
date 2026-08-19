# 2026-08-18 Cline ACP Package And Route Acceptance

## Result

Card 265 accepted `cline.acp` as an unreleased additive production route.

Current source is 33 packages and 40 routes. Immutable `v0.3.2` stays 30
packages and 36 routes. Package `swallowtail-adapter-cline` is separately
selectable. Exact claim remains npm `cline@3.0.55` / `cline.package`, spawn
`cline --acp`, qualified-only. Swallowtail does not pass `--auto-approve true`,
does not bind a credential lease, and does not flatten onto `cline --json`.

Live install, login, and prompt were not justified: this host has no `cline`,
and the card forbids unbounded live qualification. Deterministic acceptance
stands alone.

## Validation

- `effigy validate:focused swallowtail-adapter-cline` — 28 tests
- `effigy package:verify-affected swallowtail-adapter-cline`
- `effigy check:examples`
- `effigy qa:docs`
- `effigy qa:routes` / `effigy qa:guides` as index agreement

## Next

Implement the Cline headless identity corpus (card 304).
