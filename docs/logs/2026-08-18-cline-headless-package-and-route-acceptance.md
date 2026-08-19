# 2026-08-18 Cline Headless Package And Route Acceptance

## Result

Card 307 accepted `cline.headless` as an unreleased additive production
route on `swallowtail-adapter-cline`.

Current source is 33 packages and 41 routes. Immutable `v0.3.2` stays 30
packages and 36 routes. Exact claim remains npm `cline@3.0.55` /
`cline.package`, spawn `cline --json --auto-approve false` plus one argv
prompt. Envelope NDJSON is the wire. Swallowtail does not pass
`--auto-approve true`, does not bind a credential lease, does not select a
model route, and does not flatten onto `cline --acp`. `prepare_cline_acp`
stays a separate constructor.

Live install, login, and `--json` prompt were not justified: this host has
no `cline`, and the card forbids unbounded live qualification.
Deterministic acceptance stands alone.

## Validation

- `effigy validate:focused swallowtail-adapter-cline swallowtail-testkit` — 132 tests
- `effigy package:verify-affected swallowtail-adapter-cline`
- `effigy check:examples`
- `effigy qa:docs`
- `effigy qa:routes` / `effigy qa:guides` as index agreement

## Next

Implement the Goose ACP identity corpus (card 266).
