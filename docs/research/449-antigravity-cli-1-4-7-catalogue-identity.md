# 449 Antigravity CLI 1.4.7 Catalogue Identity

Status: promoted.

Owner: `antigravity.catalogue`
Date: 2026-10-26
Axis: `antigravity-cli.release`
Package: `swallowtail-adapter-antigravity`

## Outcome

Freeze identity of every published official stable hop after the qualified
`1.3.1` ceiling through `1.4.7`. Keep catalogue qualification at the
pre-PR baseline: one maintained segment `1.1.9..=1.3.1` on claim
`antigravity.catalogue.release-window-1`, behavior
`antigravity.catalogue.cli-1.1.8-artifact-1.1.9-v1`, and `AllowUnverified`.
Keep the existing `1.1.8` incompatibility and the unpublished `1.2.18`
exclusion. Published hops `1.3.2` through `1.4.7` stay `UnverifiedNewer`.
Unpublished `1.3.7`, `1.3.8`, and `1.3.9` sit after that ceiling and are
also `UnverifiedNewer`; they are not a Maintained interior hole. `1.4.8` is
the first unpublished later point and stays `UnverifiedNewer`. Headless
remains at exact pinned `1.2.11`.

This is an identity-stop, not a compatible extension. Q-005 (answered
2026-09-28 NO) keeps the 2026-09-15 notes ruling scoped to the five hops
through `1.2.2`. Contract 029 Artifact Authority Without Public Source
governs later points: the shipped artifact tree and digest are support
authority; changelog and notes are discovery only. The bounded Linux x64
writer observation (`entrypoints.modelsRun.run.func2` → `modelsReport` →
`headless.Write`, 32-byte two-string stride, `models` key) does not prove
selected `agy models` dispatch, slug/display/default rendering,
config/account/env/auth/model acquisition, errors/exit, or
process/lifecycle/cleanup. No retained Linux x64 or Mac ARM64 binary remains
for a reachable call graph on both selected platforms. Metadata-only
platform rows grant no behavior credit. Research 353 is historical and does
not override Q-005.

Missing proof for a later qualification: per-hop attributable static
selected `agy models` control-flow on both Linux x64 and Mac ARM64 covering
dispatch, slug/display/default rendering, config/account/env/auth/model
acquisition, errors/exit, and process/lifecycle/cleanup, with changed
binary/runtime internals bounded by reachability.

## Finite follow-up

Owning adaptation:
`antigravity.catalogue.artifact-authority-through-current-official` on the
`version-currentness` Queue lane. This record is the identity freeze for
that task. The adaptation owns qualifying the then-current official
`antigravity.catalogue` stable. It must not invent support from notes,
writer-stride identity, or metadata-only platform rows (Q-005; Contract 029
Artifact Authority). Interior unpublished `1.3.7`/`1.3.8`/`1.3.9` must not
become Maintained without published artifacts. Preserve exact headless
`1.2.11` and the catalogue bound `1.1.9..=1.3.1` excluding `1.2.18` until
that proof lands. `AllowUnverified` stays while the adaptation is open.

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
and the `Write` argument block do not. That writer observation is
insufficient for qualification. No binary was executed. The fixture
is
[`tests/fixtures/antigravity-cli-1.4.7/`](../../crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.4.7/).

| Hop | Published | Tag commit | Release-note areas (discovery only) | `agy models` classification |
| --- | --- | --- | --- | --- |
| `1.3.1→1.3.2` | 2026-10-08 | `8c1310b2179d1d4d952c54bd0e4424bd53e904f6` | plugin marketplace and install | unproven-selected-behavior |
| `1.3.2→1.3.3` | 2026-10-10 | `800852ea61796e9fe53077a2f945e74daacb4be6` | prompt header, MCP always-load, quota reported immediately | unproven-selected-behavior |
| `1.3.3→1.3.4` | 2026-10-13 | `9eebbedf051ffd75adb15e468f48b7b75be4b15b` | `--model` default effort, `/model` picker, 30-second retry cap, unknown `--agent` on `-p` | unproven-selected-behavior |
| `1.3.4→1.3.5` | 2026-10-14 | `89081f72e10a98c3689f34b1af7261f43136ab5d` | Auto mode, `/compact`, `/diff` | unproven-selected-behavior |
| `1.3.5→1.3.6` | 2026-10-15 | `680e58585fa49c266ab38bd986816f40455895cf` | ADC Claude models in `/model`, stalled-connection retry | unproven-selected-behavior |
| `1.3.6→1.4.0` | 2026-10-16 | `e8a400403c82b7eb27a333784dc3c37985aa622e` | OSC 7501, plugin validate, new 1.4 minor | unproven-selected-behavior |
| `1.4.0→1.4.1` | 2026-10-17 | `4433f3a6dabf04ce0a5eef5b08d8e3e4067b2d8f` | plugin validate skills and agents | unproven-selected-behavior |
| `1.4.1→1.4.2` | 2026-10-18 | `86a5bdda81a063d707ac4ab87fb72f7dfe30ab67` | Remote Control history | unproven-selected-behavior |
| `1.4.2→1.4.3` | 2026-10-20 | `fdfd4becaee2439086d71e34ee06d326e06f7346` | Gemini 4 Argon in `/model` and `--model` for `GEMINI_API_KEY` | unproven-selected-behavior |
| `1.4.3→1.4.4` | 2026-10-21 | `99b8981632ddf117c4eab7b1f82b615d4f683f01` | Auto mode on Windows, `/diff`, MCP conversation id | unproven-selected-behavior |
| `1.4.4→1.4.5` | 2026-10-22 | `902d212319e365583596a014c152745cdf36c49a` | thought rendering, content-safety silent retry | unproven-selected-behavior |
| `1.4.5→1.4.6` | 2026-10-23 | `fd851f1da40e4347f1d05d211dcb45f94aede4c4` | status line shows model retry countdown | unproven-selected-behavior |
| `1.4.6→1.4.7` | 2026-10-24 | `bf1e6e6d4a05803d21dbe9af5ffba04d8b4fc0ff` | hooks, seasonal theme, headless `-p` CPU | unproven-selected-behavior |

All thirteen changed source-path sets are exactly `{CHANGELOG.md}`. Official
docs separately describe `agy models` as listing model slugs. The current
unversioned headless docs also show a padded two-column sample. The `1.3.1`
writer already stores a 32-byte two-string element, and that stride does not
change through `1.4.7`, so the sample is not a hop delta after the ceiling.
The decoder corpus stays the frozen `1.1.9` slug lines.

## Claim and boundaries

The catalogue claim stays one maintained segment on the existing behavior
revision: `1.1.9..=1.3.1` (unpublished `1.2.18` stays excluded). Official
stable hops `1.3.2` through `1.4.7` remain identity-frozen and
`UnverifiedNewer` while owning adaptation
`antigravity.catalogue.artifact-authority-through-current-official` is
open. `1.3.7`, `1.3.8`, and `1.3.9` had neither a release nor a tag on
2026-10-26; they sit after the kept ceiling and assess `UnverifiedNewer`.
`1.4.8` and `1.4.9` were also absent; `1.4.8` is the first unpublished later
point (`UnverifiedNewer`). Published release metadata for `1.4.8` is absent
at observation.

No `agy` command, installation, provider prompt, live catalogue, credential,
or host mutation occurred. Headless interior releases stay deferred and are
not inspected as a route claim here.

## Sources

- [Official GitHub latest stable release](https://github.com/google-antigravity/antigravity-cli/releases/tag/1.4.7)
- [Official CLI changelog](https://github.com/google-antigravity/antigravity-cli/blob/1.4.7/CHANGELOG.md)
- [Official Antigravity CLI headless guide](https://antigravity.google/docs/cli/headless), which documents `agy models` separately as the model-slug listing command
- [Research 380](./380-antigravity-cli-1-3-1-catalogue-identity.md) for the frozen `1.3.1` boundary
- [Q-005](../knowledge/questions.md) (2026-09-28 NO): the 2026-09-15 notes ruling stays scoped to hops through `1.2.2`
- [Research 353](./353-antigravity-headless-adaptation-to-current-official.md) is historical for hops through `1.2.2` and does not override Q-005
- [Contract 029](../knowledge/contracts/029-interface-version-qualification-and-compatibility.md) Artifact Authority Without Public Source
- [`selection.rs`](../../crates/swallowtail-adapter-antigravity/src/selection.rs) and the frozen `antigravity-cli-1.1.9` decoder corpus
