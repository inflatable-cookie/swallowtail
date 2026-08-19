# 2026-08-18 Goose ACP Package And Route Acceptance

## Result

Card 269 accepted `goose.acp` as an unreleased additive production route.

Current source is 34 packages and 42 routes. Immutable `v0.3.2` stays 30
packages and 36 routes. Package `swallowtail-adapter-goose` is separately
selectable. Exact claim remains GitHub `v1.46.0` / `goose.release`, spawn
`goose acp`, qualified-only. Swallowtail does not pass `--with-builtin`,
does not run `goose configure`, does not bind a credential lease, does not
select `GooseMode` `auto`, and does not flatten onto `goose serve`.

Live install, configure, and prompt were not justified: this host has no
`goose`, and the card forbids unbounded live qualification. Deterministic
acceptance stands alone.

## Validation

- `effigy validate:focused swallowtail-adapter-goose swallowtail-testkit`
- `effigy package:verify-affected swallowtail-adapter-goose`
- `effigy check:examples`
- `effigy qa:docs`
- `effigy qa:routes` / `effigy qa:guides` as index agreement

## Next

Implement the GitHub Copilot CLI ACP identity corpus (card 270).
