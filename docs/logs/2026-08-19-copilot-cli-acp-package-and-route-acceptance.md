# 2026-08-19 Copilot CLI ACP Package And Route Acceptance

## Result

Card 273 accepted `copilot-cli.acp` as an unreleased additive production route.

Current source is 35 packages and 43 routes. Immutable `v0.3.2` stays 30
packages and 36 routes. Package `swallowtail-adapter-copilot-cli` is separately
selectable. Exact claim remains npm `1.0.80` / `copilot-cli.package`, spawn
`copilot --acp --stdio`, qualified-only, public preview visible. Swallowtail
does not pass `--port`, `--yolo`, `--allow-all`, or server-start tool/effort
flags, does not log in, does not bind a credential lease, and does not flatten
onto Copilot IDE/API.

Live install, login, and prompt were not justified: this host has no `copilot`
on PATH, and the card forbids unbounded live qualification. Deterministic
acceptance stands alone.

## Validation

- `effigy validate:focused swallowtail-adapter-copilot-cli swallowtail-testkit`
- `effigy package:verify-affected swallowtail-adapter-copilot-cli`
- `effigy check:examples`
- `effigy qa:docs`
- `effigy qa:routes` / `effigy qa:guides` as index agreement

## Next

Implement the Mistral Vibe headless identity corpus (card 274).
