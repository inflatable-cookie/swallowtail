# 2026-08-18 Cline Headless Prepared Facade

## Result

Card 306 added `prepare_cline_headless` and one bounded JSON print
operation on `swallowtail-adapter-cline`. Preflight names
`swallowtail.cline.headless` and exact `cline.package` `3.0.55`. Access
stays host-owned `LocalUnauthenticated`. Swallowtail does not bind a
credential lease, pass `--auto-approve true`, select a model route, or
inherit ACP session lifecycle. `prepare_cline_acp` is unchanged. Missing
working-resource authority, wrong axis, and unqualified packages fail
before JSON work.

`effigy validate:focused swallowtail-adapter-cline` (49 tests) and
`effigy package:verify-affected swallowtail-adapter-cline` passed. No
live install, login, or `--json` prompt.

## Next

Implement the Cline headless package and route acceptance (card 307).
