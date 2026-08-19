# 2026-08-19 Kiro ACP Driver Core

## Result

Card 292 added package `swallowtail-adapter-kiro` and the smallest
`kiro.acp` driver. Discovery is exact `kiro-cli.release` `2.18.1`. Spawn
is `kiro-cli acp` only. First op is initialize, `session/new`, and one
bounded `session/prompt` using field `prompt`. Credentials stay
host-owned `LocalUnauthenticated`. `--cloud`, `--agent`,
`kiro-cli chat --no-interactive`, `--trust-all-tools`, `session/load`,
and docs field `content` stay out. Permission advertises `allow_always`
and does not select it. Initialize result fields stay unrecovered: a
present `agentInfo.name` must be `kiro-cli`, and a present version must
match `2.18.1`. Current source is 39 packages and 45 production routes.
Immutable `v0.3.2` stays 30 and 36. No production matrix yet.

`effigy validate:focused swallowtail-adapter-kiro` passed (27 tests,
Clippy warnings denied). No live install, login, or prompt.

## Next

Implement the Kiro ACP prepared facade (card 293).
