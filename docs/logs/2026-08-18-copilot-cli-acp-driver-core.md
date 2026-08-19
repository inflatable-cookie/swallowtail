# 2026-08-18 Copilot CLI ACP Driver Core

## Result

Card 271 added package `swallowtail-adapter-copilot-cli` and the smallest
`copilot-cli.acp` driver. Discovery is exact `copilot-cli.package` `1.0.80`.
Spawn is `copilot --acp --stdio`. First op is initialize, `session/new`,
and one bounded `session/prompt`. Public preview stays visible. Credentials
stay host-owned `LocalUnauthenticated`. TCP `--port`, `--yolo`,
`--allow-all`, server-start tool/effort flags, interactive-only slash
commands, and GitHub login stay out. Permission advertises `allow_always`
and does not select it. Current source is 35 packages and 42 production
routes. Immutable `v0.3.2` stays 30 and 36. No production matrix yet.

`effigy validate:focused swallowtail-adapter-copilot-cli` passed. No live
install, login, or prompt.

## Next

Implement the GitHub Copilot CLI ACP prepared facade (card 272).
