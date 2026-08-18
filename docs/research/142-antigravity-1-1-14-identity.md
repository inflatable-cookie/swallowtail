# 142 Antigravity 1.1.14 Identity

Status: promoted
Owner: Tom
Date: 2026-08-18
Card: g03 batch 258

## Question

After g03.084 qualified Qwen headless through `0.21.13`, which
AllowUnverified family should move first, and are host Antigravity `1.1.9`
and official GitHub `google-antigravity/antigravity-cli` `1.1.14` a
compatible extension of exact `antigravity-cli.release` `1.1.9`, a new
private milestone, or a stop?

## Remaining AllowUnverified rank

Qwen headless is done. Remaining Research 127 AllowUnverified families
with host still on a qualified bound:

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Antigravity | `1.1.9` | exact `1.1.9` | named next after Qwen |

Gemini stays deferred.

Research 127 classified Antigravity as visible unverified-newer: host
`1.1.9`, GitHub then `1.1.13` (2026-08-14). Official GitHub latest moved
to `1.1.14` on 2026-08-18. Leaving that point UnverifiedNewer would skip
useful-newer support.

## Method

Compared host `agy --version`, the frozen `1.1.9` help corpus, official
GitHub tags `1.1.9` through `1.1.14`, changelog entries, and the official
macOS arm64 `1.1.14` release binary `--help` / `--version`. The public
repository remains documentation, examples, and changelog, not executable
source.

No provider prompt. No live `agy models`. No live print run. The host
install was not replaced. Gemini API-key and enterprise sign-in were not
exercised.

## Identity

| Surface | Version | Evidence |
| --- | --- | --- |
| Host CLI | `1.1.9` | PATH `agy` reports `1.1.9`; SHA-256 `a27bff8d7c47fe5407e6740f14ecef73e86fb65ec73fec77b0765f8849024383`; size 164873872; Developer ID `Google LLC (EQHXZ8M8AV)`; source tags `1.1.8`/`1.1.9` share `03e095ac3619462ecd0928f3f5470387dbda6a00` |
| Official GitHub latest | `1.1.14` | published 2026-08-18T04:10:43Z; tag `fbf22703a9c4bda0758b5bace0ab3142746780a9`; macOS arm64 tarball SHA-256 `077159b1adc5dbfcbd18cf979ea70f77d61061d4568ea3db48c2a60b6e3e4c2a`; extracted binary SHA-256 `95ac56b30400c7e048ca8567c9ea80be26eebaddea288957fe5ae4c2acf45cd1`; size 177350416; same Developer ID, timestamp 2026-08-18 02:27:44 |

Published stables after previous ceiling `1.1.9`: `1.1.10`, `1.1.11`,
`1.1.12`, `1.1.13`, `1.1.14`. Tags `1.1.12` and `1.1.13` share commit
`f7519c9084190ed421e89dd81c63970b5177c9ef`. Public git `1.1.13..1.1.14`
is changelog-only; the `1.1.14` GitHub asset is a new signed binary that
reports `1.1.14`. No `1.1.15`. `1.1.8` stays independently unqualified.

## Selected protocol

Selected flags remain on official `1.1.14` help: `--print`,
`--output-format`, `--model`, `--mode`, `--sandbox`, `--effort`,
`--json-schema`, `--conversation`, and the `models` subcommand.
`--dangerously-skip-permissions` and ambient `--continue` stay unselected.

The only selected-help addition is unmapped `--input-format` (print-mode
stdin NDJSON turns). Swallowtail still passes the prompt as `--print`.
`models --help` still exposes only `-h`/`--help`; default `agy models`
text listing remains the catalogue surface. Changelog `1.1.12`
`--output-format` on `models` stays unused.

`1.1.10` and `1.1.12` fix `--model`, `--effort`, and `--mode` actually
applying in `-p` runs. That restores selected flags; it does not change
adapter mapping. `1.1.13` adds a Gemini API-key path. Swallowtail keeps
the personal Google subscription profile and does not flatten onto that
route.

Decoder fixtures remain `antigravity-cli-1.1.9`.

## Decision

Compatible extension of the mapped catalogue and headless subset. Reuse
`antigravity.catalogue.cli-1.1.8-artifact-1.1.9-v1` and
`antigravity.stream-json.cli-1.1.8-artifact-1.1.9-v1`. Do not add a
private milestone for `--input-format` or Gemini API-key sign-in.

- Keep baseline `1.1.9` and claim ids `release-window-1`.
- Replace exact `1.1.9` with Maintained `1.1.9..=1.1.14` on both claims.
- Keep `1.1.8` incompatible. Qualify published intermediates `1.1.10`
  through `1.1.13`.
- Raise `ANTIGRAVITY_LATEST_QUALIFIED_VERSION` to `1.1.14`.
- Synthetic later-stable UnverifiedNewer is `1.1.15`.
- Decoder specimen remains `antigravity-cli-1.1.9`.

Card 259 owns the claim change.

## Sources

- Host `agy --version` and frozen `1.1.9` help on 2026-08-18
- [GitHub `1.1.14`](https://github.com/google-antigravity/antigravity-cli/releases/tag/1.1.14)
- [Changelog at `1.1.14`](https://github.com/google-antigravity/antigravity-cli/blob/1.1.14/CHANGELOG.md)
- official `agy_cli_mac_arm64.tar.gz` for `1.1.14`
