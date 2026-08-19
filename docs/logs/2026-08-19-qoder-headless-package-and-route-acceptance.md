# 2026-08-19 Qoder Headless Package And Route Acceptance

## Result

Card 281 accepted `qoder.headless` as an unreleased additive production
route on `swallowtail-adapter-qoder`.

Current source is 37 packages and 45 routes. Immutable `v0.3.2` stays 30
packages and 36 routes. Exact claim remains npm `@qoder-ai/qodercli@1.1.25`
/ `qoder.package`, spawn `qodercli --print --output-format stream-json
--permission-mode dont_ask --max-turns 8 --no-session-persistence --cwd`.
Stream-json NDJSON is the wire. Swallowtail does not pass `--yolo` /
`bypass_permissions` / `accept_edits`, does not bind a credential lease,
does not select a model route, and does not flatten onto ACP or SDK stdio.
Bind `qodercli`, not the `qoder` IDE dispatcher.
`prepare_qoder_headless` is the production constructor.

Live install, login, and prompt were not justified: this host has no
`qoder`/`qodercli` on PATH, and the card forbids unbounded live qualification.
Deterministic acceptance stands alone.

## Validation

- `effigy validate:focused swallowtail-adapter-qoder swallowtail-testkit`
- `effigy package:verify-affected swallowtail-adapter-qoder`
- `effigy check:examples`
- `effigy qa:docs`
- `effigy qa:routes` / `effigy qa:guides` as index agreement

## Next

Implement the Pi ACP identity corpus (card 282).
