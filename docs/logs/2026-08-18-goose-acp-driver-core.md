# 2026-08-18 Goose ACP Driver Core

## Result

Card 267 added package `swallowtail-adapter-goose` and the smallest
`goose.acp` driver. Discovery is exact `goose.release` `1.46.0`. Spawn is
`goose acp` only. First op is initialize, `session/new`, and one bounded
`session/prompt`. Credentials stay host-owned `LocalUnauthenticated`.
`goose serve`, `--with-builtin`, `--enable-scheduler`, `goose configure`,
and `GooseMode` `auto` stay out. Permission advertises `allow_always` and
does not select it. Current source is 34 packages and 41 production
routes. Immutable `v0.3.2` stays 30 and 36. No production matrix yet.

`effigy validate:focused swallowtail-adapter-goose` passed. No live
install, configure, or prompt.

## Next

Implement the Goose ACP prepared facade (card 268).
