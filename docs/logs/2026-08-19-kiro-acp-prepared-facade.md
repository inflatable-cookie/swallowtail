# 2026-08-19 Kiro ACP Prepared Facade

## Result

Card 293 added `prepare_kiro_acp` and a typed session operation on
`swallowtail-adapter-kiro`. Preflight names `swallowtail.kiro.acp` and
exact `kiro-cli.release` `2.18.1`. Access stays host-owned
`LocalUnauthenticated`; Swallowtail does not bind a credential lease, run
`kiro-cli login`, inherit `KIRO_API_KEY`, or pass `--cloud` / `--agent`.
Missing working-resource authority, `kiro.headless` axis, and
unqualified releases fail before ACP work. `session/prompt` uses field
`prompt`. Current source stays 39 packages and 45 production routes.

`effigy validate:focused swallowtail-adapter-kiro` (30 tests) and
`effigy package:verify-affected swallowtail-adapter-kiro` passed. No
live install, login, or prompt.

## Next

Implement the Kiro ACP package and route acceptance (card 294).
