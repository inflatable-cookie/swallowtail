# 276 Antigravity 1.1.24 Identity

Status: promoted
Owner: Tom
Date: 2026-09-02
Card: g05 batch 050

## Question

Is official GitHub `google-antigravity/antigravity-cli` `1.1.24` a
compatible extension of `antigravity-cli.release` through `1.1.17`, a
new milestone, or a stop?

## Remaining AllowUnverified rank

Operator-named family. This family's observation only:

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Antigravity | not installed | `1.1.9..=1.1.17` | named family; official GitHub latest is `1.1.24` |

Missing host install is not a gap. Gemini stays deferred. Do not flatten
`antigravity-cli.release` onto Gemini CLI or ACP registry
`antigravity-acp`. Do not reopen Claude Code. Next Task stays on the
post-Codex all-route checkpoint.

## Method

Compared the frozen `1.1.9` help corpus, frozen `1.1.17` identity, official
GitHub tags `1.1.17` through `1.1.24`, changelog, GitHub release notes, and
the official linux-x64 release tarballs for every hop.

Each tarball was downloaded into `/tmp`, hashed, and extracted. The extracted
`antigravity` binary was hashed and inspected with `readelf` and `strings`.
Downloaded binaries were not executed. Version presence was corroborated from
in-binary literals. Selected mapped flag literals were checked in every hop.

The public repository remains documentation, examples, and changelog, not
executable source.

No provider prompt. No live `agy models`. No live print run. Nothing was
installed. Gemini API-key and enterprise sign-in were not exercised.

## Identity

| Surface | Version | Evidence |
| --- | --- | --- |
| Host CLI | not installed | `agy` and `antigravity` absent from PATH |
| Official GitHub latest | `1.1.24` | published 2026-09-02T02:38:18Z; tag `bf27ce1134b4ead2f7bfa0a4fb3cb5fcbebcaa5a`; linux-x64 tarball SHA-256 `cff1fb7ed735da72c35658645a4f916cf74f020d4cd30ab95ebe8c2a49a4d569`; extracted binary SHA-256 `22c6ddeb06d2da6049ff861e44954bf232b77bd791986104326e9500f5327193`; size 209273088; ELF Build ID `0d87e8b60bfaf0d76a8d5e6f838dddae`; in-binary literal `1.1.24` |
| Previous ceiling | `1.1.17` | recomputed tarball and extracted-binary SHA-256 match frozen Research 177 exactly |

Published stables after previous ceiling `1.1.17`: `1.1.18`, `1.1.19`,
`1.1.20`, `1.1.21`, `1.1.22`, `1.1.23`, `1.1.24`. Every hop is a published
stable GitHub release. No `1.1.25`. `1.1.8` stays independently unqualified.

Public git `1.1.17..1.1.24` is changelog-only. Each tag commit updates
`CHANGELOG.md` only. Shared changelog SHAs are not binary identity: every
linux-x64 asset is a distinct `antigravity` binary that contains its own
version literal.

## Selected protocol

Selected flags remain as in-binary literals on every hop through official
`1.1.24`: `--print`, `--output-format`, `--model`, `--mode`, `--sandbox`,
`--effort`, `--json-schema`, `--conversation`, and the `models` subcommand.
`--dangerously-skip-permissions` and ambient `--continue` stay unselected.

`--input-format` remains present and unmapped. `mcp` stays unused. `mic-serve`
first appears as an in-binary literal at `1.1.20` and stays unmapped. `/voice`,
`--remote-control`, `--project` name widening, Gemini API-key `/effort`, and
TUI extras stay unmapped.

Every hop ships exactly one file named `antigravity`. Nothing is added or
removed. The binaries are not byte-identical. The complete hop inventory is
frozen in `dist-inventory.json`.

Changelog extras that touch already-mapped print or catalogue lifecycle stay
classified as compatible-extension repairs, not a new public operation:

- `1.1.18` print-mode dropped-stream now exits non-zero
- `1.1.20` print-mode benign tool errors no longer fail the run
- `1.1.23` `models` / `agents` no longer hang on inherited stdin
- `1.1.24` piped headless stdio hang-on-exit closed with `FD_CLOEXEC`

Decoder fixtures remain `antigravity-cli-1.1.9`.

## Segment decision for card 051

Compatible extension of the mapped catalogue and headless subset. Reuse
`antigravity.catalogue.cli-1.1.8-artifact-1.1.9-v1` and
`antigravity.stream-json.cli-1.1.8-artifact-1.1.9-v1`. Do not add a
private milestone for `mic-serve`, `mcp`, `--input-format`, or Gemini
API-key sign-in.

- Keep baseline `1.1.9` and claim ids `release-window-1`.
- Extend Maintained `1.1.9..=1.1.24` on both claims.
- Qualify published intermediates `1.1.18` through `1.1.23`.
- Keep `1.1.8` incompatible.
- Raise `ANTIGRAVITY_LATEST_QUALIFIED_VERSION` to `1.1.24`.
- Synthetic later-stable UnverifiedNewer is `1.1.25`.
- Decoder specimen remains `antigravity-cli-1.1.9`.

Card 051 owns the claim change.

## Sources

- Host PATH on 2026-09-02: no `agy`
- [GitHub `1.1.24`](https://github.com/google-antigravity/antigravity-cli/releases/tag/1.1.24)
- [Changelog at `1.1.24`](https://github.com/google-antigravity/antigravity-cli/blob/1.1.24/CHANGELOG.md)
- official `agy_cli_linux_x64.tar.gz` for `1.1.17` through `1.1.24`
- frozen `crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.1.17/`
- Research 177 and Research 274
