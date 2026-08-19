# 2026-08-18 Goose ACP Prepared Facade

## Result

Card 268 added `prepare_goose_acp` and a typed session operation on
`swallowtail-adapter-goose`. Preflight names `swallowtail.goose.acp` and
exact `goose.release` `1.46.0`. Access stays host-owned
`LocalUnauthenticated`; Swallowtail does not bind a credential lease, run
`goose configure`, or pass `--with-builtin`. Missing working-resource
authority, `goose.serve` axis, and unqualified releases fail before ACP
work. Current source stays 34 packages and 41 production routes.

`effigy validate:focused swallowtail-adapter-goose` and
`effigy package:verify-affected swallowtail-adapter-goose` passed. No
live install, configure, or prompt.

## Next

Implement the Goose ACP package and route acceptance (card 269).
