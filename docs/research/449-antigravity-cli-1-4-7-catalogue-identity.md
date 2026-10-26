# 449 Antigravity CLI 1.4.7 Catalogue Identity

Status: promoted.

Owner: `antigravity.catalogue`
Date: 2026-10-26
Axis: `antigravity-cli.release`
Package: `swallowtail-adapter-antigravity`

## Outcome

Extend only the maintained Antigravity catalogue qualification from `1.3.1`
to official stable `1.4.7`. Keep baseline `1.1.9`, claim
`antigravity.catalogue.release-window-1`, behavior
`antigravity.catalogue.cli-1.1.8-artifact-1.1.9-v1`, and `AllowUnverified`.
Keep the existing `1.1.8` incompatibility and the unpublished `1.2.18`
exclusion. Exclude unpublished `1.3.7`, the skipped patch between published
`1.3.6` and `1.4.0`. `1.3.8` and `1.3.9` were absent and are not excluded,
matching the `1.2.19` precedent. `1.4.8` is the first unpublished later point
and stays `UnverifiedNewer`. Headless remains at exact pinned `1.2.11`.

The qualification is a compatible extension. No release note names a changed
`agy models` list output, decoder, or process lifecycle. The new `1.4` minor
line is not an identity stop. This conclusion uses the existing operator
ruling that official release notes are behavioral authority, plus a bounded
static look at the Linux catalogue writer. Notes that mention `/model`,
`--model`, model-request retry, content-safety retry, the status-line retry
countdown, or headless `-p` stay on the invocation or print-mode side of that
boundary. That boundary is an inference from the official notes and the
writer observation, not a live catalogue run.

## Identity and method

Re-probed the documented official GitHub Releases channel on 2026-10-26.
Latest stable is `1.4.7`, published `2026-10-24T02:47:49Z`, not draft and not
prerelease, tag commit `bf1e6e6d4a05803d21dbe9af5ffba04d8b4fc0ff`. The exact
stable hops after the qualified ceiling are `1.3.2`, `1.3.3`, `1.3.4`,
`1.3.5`, `1.3.6`, `1.4.0`, `1.4.1`, `1.4.2`, `1.4.3`, `1.4.4`, `1.4.5`,
`1.4.6`, and `1.4.7`.

For each release, the frozen identity fixture records its tag commit, release
time, the complete eight-asset GitHub digest and size manifest, and a
canonical asset-name set. The selected Linux x64 and Mac ARM64 tarballs were
downloaded; each downloaded digest matches GitHub's published digest. Each
archive contains exactly one regular file, `antigravity`. The `1.3.1`
boundary was re-downloaded; its tag commit and both platform archive and
member digests match the frozen Research 380 fixture. Other platform assets
are identified from the official manifest but were not downloaded. No `agy`
binary is on `PATH`. No version command was run.

Every adjacent public tag comparison has exactly one commit and changes only
`CHANGELOG.md`. On Linux x64, `entrypoints.modelsRun.run.func2` is
structurally identical from `1.3.1` through `1.4.7` and calls
`entrypoints.modelsReport` then `headless.Write`. `modelsReport` keeps a
32-byte two-string element stride and the six-byte map key `models`. At
`1.3.4` the source-model stack copy grows by 32 bytes; the element stride
and the `Write` argument block do not. No binary was executed. The fixture
is
[`tests/fixtures/antigravity-cli-1.4.7/`](../../crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.4.7/).

| Hop | Published | Tag commit | Release-note areas | `agy models` classification |
| --- | --- | --- | --- | --- |
| `1.3.1→1.3.2` | 2026-10-08 | `8c1310b2179d1d4d952c54bd0e4424bd53e904f6` | plugin marketplace and install | unchanged |
| `1.3.2→1.3.3` | 2026-10-10 | `800852ea61796e9fe53077a2f945e74daacb4be6` | prompt header, MCP always-load, quota reported immediately | unchanged; quota note is request-path |
| `1.3.3→1.3.4` | 2026-10-13 | `9eebbedf051ffd75adb15e468f48b7b75be4b15b` | `--model` default effort, `/model` picker, 30-second retry cap, unknown `--agent` on `-p` | unchanged; invocation and headless, not the list command |
| `1.3.4→1.3.5` | 2026-10-14 | `89081f72e10a98c3689f34b1af7261f43136ab5d` | Auto mode, `/compact`, `/diff` | unchanged |
| `1.3.5→1.3.6` | 2026-10-15 | `680e58585fa49c266ab38bd986816f40455895cf` | ADC Claude models in `/model`, stalled-connection retry | unchanged; picker and request path |
| `1.3.6→1.4.0` | 2026-10-16 | `e8a400403c82b7eb27a333784dc3c37985aa622e` | OSC 7501, plugin validate, new 1.4 minor | unchanged; minor line is not a stop |
| `1.4.0→1.4.1` | 2026-10-17 | `4433f3a6dabf04ce0a5eef5b08d8e3e4067b2d8f` | plugin validate skills and agents | unchanged |
| `1.4.1→1.4.2` | 2026-10-18 | `86a5bdda81a063d707ac4ab87fb72f7dfe30ab67` | Remote Control history | unchanged |
| `1.4.2→1.4.3` | 2026-10-20 | `fdfd4becaee2439086d71e34ee06d326e06f7346` | Gemini 4 Argon in `/model` and `--model` for `GEMINI_API_KEY` | unchanged; picker and invocation |
| `1.4.3→1.4.4` | 2026-10-21 | `99b8981632ddf117c4eab7b1f82b615d4f683f01` | Auto mode on Windows, `/diff`, MCP conversation id | unchanged |
| `1.4.4→1.4.5` | 2026-10-22 | `902d212319e365583596a014c152745cdf36c49a` | thought rendering, content-safety silent retry | unchanged; retry is request-path |
| `1.4.5→1.4.6` | 2026-10-23 | `fd851f1da40e4347f1d05d211dcb45f94aede4c4` | status line shows model retry countdown | unchanged; status line, not list output |
| `1.4.6→1.4.7` | 2026-10-24 | `bf1e6e6d4a05803d21dbe9af5ffba04d8b4fc0ff` | hooks, seasonal theme, headless `-p` CPU | unchanged; `-p` is print mode |

All thirteen changed source-path sets are exactly `{CHANGELOG.md}`. Official
docs separately describe `agy models` as listing model slugs. The current
unversioned headless docs also show a padded two-column sample. The `1.3.1`
writer already stores a 32-byte two-string element, and that stride does not
change through `1.4.7`, so the sample is not a hop delta after the ceiling.
The decoder corpus stays the frozen `1.1.9` slug lines.

## Claim and boundaries

The supported catalogue segment remains one maintained behavior segment from
`1.1.9` through `1.4.7`, with exact exclusions `1.2.18` and `1.3.7`. Official
stable hops `1.3.2` through `1.3.6` and `1.4.0` through `1.4.7` qualify on
the existing behavior revision. `1.3.7`, `1.3.8`, `1.3.9`, `1.4.8`, and
`1.4.9` had neither a release nor a tag on 2026-10-26. Only `1.3.7` is the
new exclusion. Published release metadata for `1.4.8` is absent at
observation.

No `agy` command, installation, provider prompt, live catalogue, credential,
or host mutation occurred. Headless interior releases stay deferred and are
not inspected as a route claim here.

## Sources

- [Official GitHub latest stable release](https://github.com/google-antigravity/antigravity-cli/releases/tag/1.4.7)
- [Official CLI changelog](https://github.com/google-antigravity/antigravity-cli/blob/1.4.7/CHANGELOG.md)
- [Official Antigravity CLI headless guide](https://antigravity.google/docs/cli/headless), which documents `agy models` separately as the model-slug listing command
- [Research 380](./380-antigravity-cli-1-3-1-catalogue-identity.md) for the frozen `1.3.1` boundary
- [Research 353](./353-antigravity-headless-adaptation-to-current-official.md) for the release-notes behavioral-authority ruling
- [Contract 029](../knowledge/contracts/029-interface-version-qualification-and-compatibility.md)
- [`selection.rs`](../../crates/swallowtail-adapter-antigravity/src/selection.rs) and the frozen `antigravity-cli-1.1.9` decoder corpus
