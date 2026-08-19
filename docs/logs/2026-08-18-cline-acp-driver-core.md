# 2026-08-18 Cline ACP Driver Core

## Result

Card 263 added package `swallowtail-adapter-cline` and the smallest
`cline.acp` driver. Discovery is exact `cline.package` `3.0.55`. Spawn is
`cline --acp` only. First op is initialize, `session/new`, and one bounded
`session/prompt`. Credentials stay host-owned `LocalUnauthenticated`.
`--json`, `--id`, `--auto-approve true`, OAuth `authenticate`, and
`session/load` stay out. Permission advertises `allow_always` and does not
select it.

`effigy validate:focused swallowtail-adapter-cline` passed. No live
install, login, or prompt.

## Next

Implement the Cline ACP prepared facade (card 264).
