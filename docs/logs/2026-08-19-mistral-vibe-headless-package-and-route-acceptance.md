# 2026-08-19 Mistral Vibe Headless Package And Route Acceptance

## Result

Card 277 accepted `mistral-vibe.headless` as an unreleased additive production
route on `swallowtail-adapter-mistral-vibe`.

Current source is 36 packages and 44 routes. Immutable `v0.3.2` stays 30
packages and 36 routes. Exact claim remains GitHub/PyPI `2.24.2` /
`mistral-vibe.release`, spawn `vibe --prompt --output streaming --max-turns 8
--trust --agent plan --workdir`. Streaming NDJSON is the wire. Swallowtail
does not pass `--auto-approve`/`--yolo`, does not bind a credential lease,
does not select a model route, and does not flatten onto `vibe-acp`.
`prepare_mistral_vibe_headless` is the production constructor.

Live install, login, and prompt were not justified: this host has no `vibe`
on PATH, and the card forbids unbounded live qualification. Deterministic
acceptance stands alone.

## Validation

- `effigy validate:focused swallowtail-adapter-mistral-vibe swallowtail-testkit`
- `effigy package:verify-affected swallowtail-adapter-mistral-vibe`
- `effigy check:examples`
- `effigy qa:docs`
- `effigy qa:routes` / `effigy qa:guides` as index agreement

## Next

Implement the Qoder headless identity corpus (card 278).
