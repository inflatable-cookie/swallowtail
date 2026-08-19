# 2026-08-18 Cline ACP Prepared Facade

## Result

Card 264 added `prepare_cline_acp` and a typed session operation on
`swallowtail-adapter-cline`. Preflight names `swallowtail.cline.acp` and
exact `cline.package` `3.0.55`. Access stays host-owned
`LocalUnauthenticated`; Swallowtail does not bind a credential lease or
pass `--auto-approve true`. Missing working-resource authority, wrong
axis, and unqualified packages fail before ACP work.

`effigy validate:focused swallowtail-adapter-cline` and
`effigy package:verify-affected swallowtail-adapter-cline` passed. No
live install, login, or prompt.

## Next

Implement the Cline headless identity corpus (card 304).
