# 177 Antigravity 1.1.17 Identity

Status: promoted
Owner: Tom
Date: 2026-08-21
Card: g04 batch 080

## Question

Is official GitHub `google-antigravity/antigravity-cli` `1.1.17` a
compatible extension of `antigravity-cli.release` through `1.1.15`, a
new milestone, or a stop?

## Remaining AllowUnverified rank

Operator-named family. This family's observation only:

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Antigravity | not installed | `1.1.9..=1.1.15` | named family; official GitHub latest is `1.1.17` |

Missing host install is not a gap. Gemini stays deferred. Do not flatten
`antigravity-cli.release` onto Gemini CLI or ACP registry
`antigravity-acp` `1.0.0`.

## Method

Compared the frozen `1.1.9` help corpus, official GitHub tags `1.1.15`,
`1.1.16`, and `1.1.17`, changelog, GitHub release notes, and the official
linux-x64 `1.1.16` and `1.1.17` release binaries `--help` / `--version`.
The public repository remains documentation, examples, and changelog, not
executable source.

No provider prompt. No live `agy models`. No live print run. Nothing was
installed. Gemini API-key and enterprise sign-in were not exercised.

## Identity

| Surface | Version | Evidence |
| --- | --- | --- |
| Host CLI | not installed | `agy` and `antigravity` absent from PATH |
| Official GitHub latest | `1.1.17` | published 2026-08-20T22:13:58Z; tag `efa16f096dc02fb654b7e86958d268195284d014`; linux-x64 tarball SHA-256 `15443966494cd62938320900acfd16df906cf4da56279e4dd8f4846c09f849df`; extracted binary SHA-256 `d1ea7370fce2ae229a370d8cc42e91d4eeb971344c5f07918e55ce05a4e19579`; size 205574400; extracted `--version` reports `1.1.17` |
| Published intermediate | `1.1.16` | published 2026-08-20T04:14:18Z; same git SHA; linux-x64 tarball SHA-256 `7742953b7835b457e9102f1357a493913657dfd147435584f609d58356ec085a`; extracted binary SHA-256 `b233e6a4f38564a06a0d3220aa79f6a7c8f11da2b85fc8f0957f8a14d46e6cc9`; size 205545512; extracted `--version` reports `1.1.16` |

Published stables after previous ceiling `1.1.15`: `1.1.16`, `1.1.17`.
Public git `1.1.15..1.1.17` is changelog-only. Public git
`1.1.16..1.1.17` is identical. Shared tag SHA is not binary identity:
the two GitHub assets differ and report their own versions. Changelog
file at the shared tag still heads at `1.1.16`; `1.1.17` notes live on
the GitHub release. No `1.1.18`. `1.1.8` stays independently
unqualified.

ACP registry lists `antigravity-acp` `1.0.0`. That is a different
family. It is not this CLI axis.

## Selected protocol

Selected flags remain on official `1.1.17` help: `--print`,
`--output-format`, `--model`, `--mode`, `--sandbox`, `--effort`,
`--json-schema`, `--conversation`, and the `models` subcommand.
`--dangerously-skip-permissions` and ambient `--continue` stay
unselected.

`--input-format` remains present and unmapped. `models --help` still
exposes only `-h`/`--help`. Help `1.1.16` is byte-identical to
`1.1.17`. The only help addition since `1.1.15` is the `mcp` subcommand
(`add` / `remove` / `list` / `enable` / `disable`). That stays unused.

`1.1.16` changelog extras stay unmapped: interactive `@` completion,
Gemini API-key `/effort` path, TUI fixes. `1.1.17` release notes stay
unmapped: harness consolidation, `/teamwork-preview`, Vim insert Enter,
Ogg MIME. Gemini API-key sign-in stays unused.

Help banner says `Usage of antigravity:`. Frozen `1.1.9` help says
`Usage of agy:`. That is argv0 noise, not a protocol change. Automatic
executable name stays `agy`.

Decoder fixtures remain `antigravity-cli-1.1.9`.

## Segment decision for card 081

Compatible extension of the mapped catalogue and headless subset. Reuse
`antigravity.catalogue.cli-1.1.8-artifact-1.1.9-v1` and
`antigravity.stream-json.cli-1.1.8-artifact-1.1.9-v1`. Do not add a
private milestone for `mcp`, `--input-format`, or Gemini API-key
sign-in.

- Keep baseline `1.1.9` and claim ids `release-window-1`.
- Extend Maintained `1.1.9..=1.1.17` on both claims.
- Qualify published intermediate `1.1.16`.
- Keep `1.1.8` incompatible.
- Raise `ANTIGRAVITY_LATEST_QUALIFIED_VERSION` to `1.1.17`.
- Synthetic later-stable UnverifiedNewer is `1.1.18`.
- Decoder specimen remains `antigravity-cli-1.1.9`.

Card 081 owns the claim change.

## Sources

- Host PATH on 2026-08-21: no `agy`
- [GitHub `1.1.17`](https://github.com/google-antigravity/antigravity-cli/releases/tag/1.1.17)
- [GitHub `1.1.16`](https://github.com/google-antigravity/antigravity-cli/releases/tag/1.1.16)
- [Changelog at `1.1.17`](https://github.com/google-antigravity/antigravity-cli/blob/1.1.17/CHANGELOG.md)
- official `agy_cli_linux_x64.tar.gz` for `1.1.16` and `1.1.17`
- frozen `crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.1.15/`
- ACP registry `https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json` (`antigravity-acp` `1.0.0`, not this axis)
