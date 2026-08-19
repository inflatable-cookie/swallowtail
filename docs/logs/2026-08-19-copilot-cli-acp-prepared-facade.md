# 2026-08-19 Copilot CLI ACP Prepared Facade

## Result

Card 272 added `prepare_copilot_cli_acp` and a typed session operation on
`swallowtail-adapter-copilot-cli`. Preflight names `swallowtail.copilot-cli.acp`,
exact `copilot-cli.package` `1.0.80`, and public preview as
`ExperimentalObserved`. Access stays host-owned `LocalUnauthenticated`;
Swallowtail does not bind a credential lease, run GitHub login, or pass
`--port` / `--yolo`. Missing working-resource authority, `copilot-cli.tcp-port`
axis, and unqualified releases fail before ACP work. Current source stays
35 packages and 42 production routes.

`effigy validate:focused swallowtail-adapter-copilot-cli` and
`effigy package:verify-affected swallowtail-adapter-copilot-cli` passed. No
live install, login, or prompt.

## Next

Implement the GitHub Copilot CLI ACP package and route acceptance (card 273).
