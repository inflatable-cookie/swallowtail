# 167 Antigravity 1.1.15 Identity

Status: promoted
Owner: Tom
Date: 2026-08-19
Card: g03 batch 322

## Question

After g03.104 qualified Oh My Pi through `17.3.8`, is official GitHub
`google-antigravity/antigravity-cli` `1.1.15` a compatible extension of
`antigravity-cli.release` through `1.1.14`, a new milestone, or a stop?

## Remaining AllowUnverified rank

Oh My Pi is done. Remaining Research 159 families:

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Antigravity | `1.1.9` | `1.1.9..=1.1.14` | named next; host sits on a qualified bound; official latest is `1.1.15` |

Gemini stays deferred. Do not flatten `antigravity-cli.release` onto
Gemini CLI.

## Method

Compared host `agy --version`, the frozen `1.1.9` help corpus, official
GitHub tags `1.1.14` and `1.1.15`, changelog, and the official macOS
arm64 `1.1.15` release binary `--help` / `--version`. The public
repository remains documentation, examples, and changelog, not executable
source.

No provider prompt. No live `agy models`. No live print run. The host
install was not replaced. Gemini API-key and enterprise sign-in were not
exercised.

## Identity

| Surface | Version | Evidence |
| --- | --- | --- |
| Host CLI | `1.1.9` | PATH `agy` reports `1.1.9`; SHA-256 `a27bff8d7c47fe5407e6740f14ecef73e86fb65ec73fec77b0765f8849024383`; size 164873872; Developer ID `Google LLC (EQHXZ8M8AV)` |
| Official GitHub latest | `1.1.15` | published 2026-08-19T04:11:33Z; tag `76ff39c65b5d52482172b6408c27ded9b17c303d`; macOS arm64 tarball SHA-256 `66b784680c07c78df44e7bdffd6de3395385edc333ce8eb259d3719dca76485f`; extracted binary SHA-256 `6f84ffdab59b64e00d8ef74c766917a3fab77c14200e83071e56fae595d9aa2c`; size 177531792; same Developer ID, timestamp 2026-08-19 02:54:17; extracted `--version` reports `1.1.15` |

Published stables after previous ceiling `1.1.14`: `1.1.15`. Public git
`1.1.14..1.1.15` is changelog-only. The `1.1.15` GitHub asset is a new
signed binary that reports `1.1.15`. No `1.1.16`. `1.1.8` stays
independently unqualified.

## Selected protocol

Selected flags remain on official `1.1.15` help: `--print`,
`--output-format`, `--model`, `--mode`, `--sandbox`, `--effort`,
`--json-schema`, `--conversation`, and the `models` subcommand.
`--dangerously-skip-permissions` and ambient `--continue` stay unselected.

`--input-format` remains present and unmapped. Swallowtail still passes
the prompt as `--print`. `models --help` still exposes only `-h`/`--help`;
default `agy models` text listing remains the catalogue surface.
Changelog `models --output-format` stays unused. Gemini API-key sign-in
stays unused.

Decoder fixtures remain `antigravity-cli-1.1.9`.

## Segment decision for card 323

Compatible extension of the mapped catalogue and headless subset. Reuse
`antigravity.catalogue.cli-1.1.8-artifact-1.1.9-v1` and
`antigravity.stream-json.cli-1.1.8-artifact-1.1.9-v1`. Do not add a
private milestone for `--input-format` or Gemini API-key sign-in.

- Keep baseline `1.1.9` and claim ids `release-window-1`.
- Extend Maintained `1.1.9..=1.1.15` on both claims.
- Keep `1.1.8` incompatible.
- Raise `ANTIGRAVITY_LATEST_QUALIFIED_VERSION` to `1.1.15`.
- Synthetic later-stable UnverifiedNewer is `1.1.16`.
- Decoder specimen remains `antigravity-cli-1.1.9`.

Card 323 owns the claim change.

## Sources

- Host `agy --version` and frozen `1.1.9` help on 2026-08-19
- [GitHub `1.1.15`](https://github.com/google-antigravity/antigravity-cli/releases/tag/1.1.15)
- [Changelog at `1.1.15`](https://github.com/google-antigravity/antigravity-cli/blob/1.1.15/CHANGELOG.md)
- official `agy_cli_mac_arm64.tar.gz` for `1.1.15`
- frozen `crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.1.14/`
