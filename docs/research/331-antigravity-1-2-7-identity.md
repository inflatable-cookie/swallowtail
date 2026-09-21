# 331 Antigravity 1.2.7 Catalogue Identity

Status: promoted
Owner: Tom
Date: 2026-09-21
Authority: Contracts 017, 023, and 029; Research 177, 283, and 323; the
Antigravity prepared guide; and the official GitHub channel. Tom's
2026-09-15 ruling still holds: official release notes are the behavioural
authority. Catalogue-only; headless stays stopped.

## Question

Do official GitHub `google-antigravity/antigravity-cli` `1.2.3` through
`1.2.7` extend the catalogue claim qualified through `1.2.2`, or does any
hop force the headless `1.1.22` stop to move with it?

## Remaining AllowUnverified rank

Named family only.

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Antigravity catalogue | not installed | `1.1.9..=1.2.2` | operator-dispatched catalogue-only hop; official GitHub stable is `1.2.7` |

Headless stays stopped at `1.1.17`. Gemini is out of scope. `antigravity-acp`
is another family. `1.1.8` stays independently incompatible.

## Method

Re-probed official GitHub releases and tags on 2026-09-21. Latest stable is
`1.2.7`, published `2026-09-19T01:01:46Z`, tag commit
`7bb195acaec9e7788df5210d0dc3e15f3cefc6b3`. Retrieved the official
linux-x64 and mac-arm64 tarballs for `1.2.3` through `1.2.7` into `/tmp`;
every tarball SHA-256 matches GitHub's declared asset digest, every
archive holds only `antigravity`, and each binary carries its own
newline-terminated version literal. Public compares
`1.2.2...1.2.3` through `1.2.6...1.2.7` are each one commit and change
only `CHANGELOG.md`. GitHub has no release or tag `1.2.8`. Host `agy` is
not on `PATH` and was not installed.

No provider operation, prompt, login, credential, installation, host
update, or downloaded-binary execution occurred. Exhaustive binary
scanning stayed stopped.

## Identity

| Version | Published | Tag commit | linux-x64 tarball SHA-256 / size | linux-x64 binary SHA-256 / size / Build ID | mac-arm64 tarball SHA-256 |
| --- | --- | --- | --- | --- | --- |
| `1.2.3` | 2026-09-15T02:03:04Z | `444063c79f81a36f124ad613b30caf113f093c0b` | `57afb34f2a4be9296beb477e600761b6ac7401eb3a54a64a0014d573b7fc3af4` / 57891892 | `c4c8a6722f9b570e370941b0953ba29051336307d7999ec842bdf7500b0ca7c8` / 215384320 / `41c0722061557995df58308202bf857a` | `c244ec966f5d8c22d845117332a6858f3abaf0fb5c8058e337d51b77b2c93e50` |
| `1.2.4` | 2026-09-16T03:54:29Z | `e5dcb8247e3c364671f2bae4c64971c8632590ed` | `dcd3e4d8c8afb1902d59c1ae52812458d2ddab67a5d2db44810c512910d918fe` / 58151273 | `5c19ea964509bc4fd33c3789860f017a8ca22ef93cb48d333c7b95b118bbb6a7` / 217022720 / `4c44c1cba03957b96aa5127baafa441d` | `f59c12c289e74bbb48178f827702c6224bd0aa920914319f85b42760dfc72f4c` |
| `1.2.5` | 2026-09-17T04:10:06Z | `48e88e0723bc8f3c7b6bdae85d34067b04fc9277` | `e450caab5682acc920721b04cf0f6860c313d1f5296bc6e49d79cd3843802e65` / 58151164 | `84808e105f643f135d9b347b36005d5bbe6cfe09c5ffc61e3311700adb5a3286` / 217284864 / `cd38e0524c59a16f19d3d459ae25311d` | `b37495eebe53e1c565dd6d77bdd1d3ba603216b5d3acddafb185f97d35fdf9eb` |
| `1.2.6` | 2026-09-18T04:21:05Z | `d39491f6f98a62aaf29af76964aa4fe75bc044d4` | `3d4973187c4c074e70894068053eeef1b7dfa9c0a16bfe9c3b81954de0d2cc5c` / 58626363 | `312eb057d8b8155383e74242e948333c6556adc779e9398b121a863390691fe9` / 218132688 / `15cff90bb5b1dfa7429a148ef393c21f` | `14e1be7ed9b35e512b6aa2f2b2ba3ad79877dbde1b2d487892d3721806ff9e59` |
| `1.2.7` | 2026-09-19T01:01:46Z | `7bb195acaec9e7788df5210d0dc3e15f3cefc6b3` | `e410dd56d8c213ef12643d3ff5eaaab57a17e05bbf72e9415322f23879fc4a18` / 61763170 | `9991515b6d5307bcf701069622b0537b6b206e605f3c891c0cf3a3d208dea8b0` / 224268544 / `4666c9380200e7538aab7b73748b9341` | `ce9fe3f4d6f44a2b1c83b334fc5c8f2975079959e24dd805e10eb49ab8c76a7e` |

The Research 323 `1.2.2` linux-x64 tarball and binary hashes stay the
previous-ceiling boundary. Every mac-arm64 extracted digest and size is
in the frozen corpus.

## Published selected-path classification per claim

No hop names a selected-path change to `agy models`. Retry, timeout, and
print-error notes stay on the agent model-request loop and
`-p`/`--prompt` headless path.

| Hop | Catalogue | Headless |
| --- | --- | --- |
| `1.2.2→1.2.3` | unchanged | none published on the selected print path |
| `1.2.3→1.2.4` | unchanged | agent turns no longer terminate on tool-schema validation failure |
| `1.2.4→1.2.5` | unchanged | outside-signal-killed commands record canceled instead of exit 0 |
| `1.2.5→1.2.6` | unchanged | default `--print-timeout` becomes unlimited; agent/model API failure prints `AGY_ERROR` and exits 3 |
| `1.2.6→1.2.7` | unchanged | per-attempt retry backoff capped at 30 seconds; headless waiting-notice skip repaired |

`1.2.6` and `1.2.7` deepen the headless gap. They do not reopen it: the
30-second cap is a per-attempt delay, not a finite attempt bound or
disable control, and no separate Contract 023 acceptance exists. The
catalogue command stays isolated, so headless does not have to move.

Interactive `/model` picker, Remote Control, plugins, skills, Kitty
graphics, and `GEMINI_API_KEY` daemon background stay unmapped.

## Decision

- **Catalogue: compatible-extension.** Advance to maintained
  `1.1.9..=1.2.7` on `antigravity.catalogue.cli-1.1.8-artifact-1.1.9-v1`.
  Qualify `1.2.3` through `1.2.6`. Synthetic `1.2.8` stays
  `UnverifiedNewer`.
- **Headless: stop stands.** Keep `1.1.9..=1.1.17`. Gap becomes
  `1.1.18..=1.2.7`. Do not raise `ANTIGRAVITY_HEADLESS_LATEST_QUALIFIED_VERSION`.
- No private milestone, no new public operation, no flatten onto Gemini
  CLI or `antigravity-acp`. Decoder specimens stay on
  `antigravity-cli-1.1.9`.

## Sources

- [GitHub `1.2.7`](https://github.com/google-antigravity/antigravity-cli/releases/tag/1.2.7)
  and the release notes for `1.2.3` through `1.2.7`
- [Changelog at `1.2.7`](https://github.com/google-antigravity/antigravity-cli/blob/1.2.7/CHANGELOG.md)
- official `agy_cli_linux_x64.tar.gz` and `agy_cli_mac_arm64.tar.gz` for
  `1.2.3`..=`1.2.7`
- frozen `crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.2.7/`
- [Research 323](./323-antigravity-1-2-2-identity.md) and
  [Research 283](./283-antigravity-1-1-26-identity.md)
- [Contract 023](../contracts/023-harness-operation-isolation-and-native-boundary.md)
  and [Contract 029](../contracts/029-interface-version-qualification-and-compatibility.md)
