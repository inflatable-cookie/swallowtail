# 380 Antigravity CLI 1.3.1 Catalogue Identity

Status: proposed for review.

Owner: swallowtail#103 (`antigravity.catalogue`)
Date: 2026-10-08
Axis: `antigravity-cli.release`
Package: `swallowtail-adapter-antigravity`

## Outcome

Extend only the maintained Antigravity catalogue qualification from `1.2.11`
to official stable `1.3.1`. Keep baseline `1.1.9`, claim
`antigravity.catalogue.release-window-1`, behavior
`antigravity.catalogue.cli-1.1.8-artifact-1.1.9-v1`, `AllowUnverified`, and the
existing `1.1.8` incompatibility. Exclude unpublished `1.2.18` inside the
semver interval; `1.3.2` remains the first unpublished later point and
`UnverifiedNewer`. Headless remains at exact pinned `1.2.11`; its
`1.1.18..=1.2.10` gap is untouched.

The qualification is a compatible extension. No release note names a changed
`agy models` list output, decoder, or process lifecycle. This conclusion uses
the existing operator ruling that official release notes are behavioral
authority. In particular, the `1.2.13` model-API retry note is classified as a
request retry change outside the distinct model-slug listing command; that
boundary is an inference from the official notes and official CLI guide, not a
live observation.

## Identity and method

Re-probed the documented official GitHub Releases channel on 2026-10-08. Latest
stable is `1.3.1`, published `2026-10-07T03:22:02Z`, tag commit
`968f1170bd0e002e9d0914730975bc8a2cc65861`. The exact stable hops after the
qualified ceiling are `1.2.12`, `1.2.13`, `1.2.14`, `1.2.15`, `1.2.16`,
`1.2.17`, `1.3.0`, and `1.3.1`.

For each release, the frozen identity fixture records its tag commit, release
time, the complete eight-asset GitHub digest and size manifest, and a canonical
asset-name set. The selected Linux x64 and Mac ARM64 tarballs were downloaded;
each downloaded digest matches GitHub's published digest. Each archive
contains exactly one regular file, `antigravity`; its byte size and SHA-256
are recorded in the complete per-archive inventory. Other platform assets are
identified from the official manifest but were not downloaded or unpacked.
The prior `1.2.11` boundary identity is carried forward from frozen Research
353, without re-downloading it.

Every adjacent public tag comparison has exactly one commit and changes only
`CHANGELOG.md`. That file is discovery and release-note evidence; the table
below classifies the selected route against all notes for each hop. The
per-platform artifacts are opaque native executables. No binary was run and no
exhaustive binary inspection was attempted under the existing release-notes
authority ruling. `identity.json`, `protocol.json`, and `dist-inventory.json`
freeze the exact source/runtime identities and mutation-sensitive path sets,
including each changed executable's previous and current digest per hop, at
[`tests/fixtures/antigravity-cli-1.3.1/`](../../crates/swallowtail-adapter-antigravity/tests/fixtures/antigravity-cli-1.3.1/).

| Hop | Published | Tag commit | Release-note areas | `agy models` classification |
| --- | --- | --- | --- | --- |
| `1.2.11→1.2.12` | 2026-09-27 | `8cb7cd1bbab008cada5e8b27b56ed29107f45de0` | UI scrolling, Gemini API-key quota handling, history replay, terminal and Vim behavior | unchanged |
| `1.2.12→1.2.13` | 2026-09-29 | `77b1aad0cb850c184a987661b149985de41a3568` | model-API retry-delay handling, rendering, artifact-viewer labels, narrow-terminal dialogs | unchanged; retry note is request-path scoped per the evidence above |
| `1.2.13→1.2.14` | 2026-09-30 | `eaf9e06660d2ca8f20f1474ed03f10e3dbfd35e5` | queued messages, remote-control process, history, sign-in, JSON Schema startup checks, files/media | unchanged |
| `1.2.14→1.2.15` | 2026-10-02 | `ae27bff644d78ed0c759b4d8a6b79506f1bed96d` | Android assets, usage display, permission handling, conversation reconnect, customization, MCP auth | unchanged |
| `1.2.15→1.2.16` | 2026-10-03 | `65a3c69e388148c9327f307efe82ddb1c0c8d7d4` | settings, rendering, image subagent, background commands, customization and skills | unchanged |
| `1.2.16→1.2.17` | 2026-10-05 | `274d81b9929aaa2b91a7266106d0d0b7f19adf52` | announcement cards, Windows sandbox, markdown and TIFF handling | unchanged |
| `1.2.17→1.3.0` | 2026-10-06 | `5e9c9c6c3fb1aea16dbc77918d687a842620cd1f` | verbosity default, diff navigation, scrolling, path handling | unchanged |
| `1.3.0→1.3.1` | 2026-10-07 | `968f1170bd0e002e9d0914730975bc8a2cc65861` | diff and rewind, plugins, agents, tasks, notifications, MCP restart, sign-in, Gemini API-key retry | unchanged |

All eight changed source-path sets are exactly `{CHANGELOG.md}`. The complete
release notes and exact artifact digests, sizes, and file inventories remain
in the fixture so the table stays compact. Official docs separately describe
`agy models` as listing model slugs; they do not claim model invocation.

## Claim and boundaries

The supported catalogue segment remains one maintained behavior segment from
`1.1.9` through `1.3.1`, with exact exclusion `1.2.18`. The excluded point has
neither an official stable release nor tag. Official stable hops `1.2.12` to
`1.2.17`, `1.3.0`, and `1.3.1` qualify on the existing behavior revision.
Published release metadata for `1.3.2` and its tag are absent at observation.

The current Antigravity CLI's host binary was identified statically as a
Mach-O ARM64 file; its version command was not run because it may auto-update.
The fixture records its digest without its local path. This static observation
does not qualify a host runtime. No `agy` command, installation, provider
prompt, live catalogue, credential, or host mutation occurred. Official ACP
registry `1.3.0` is a different product. Headless interior releases stay
deferred and are not inspected as a route claim here.

## Sources

- [Official GitHub latest stable release](https://github.com/google-antigravity/antigravity-cli/releases/tag/1.3.1)
- [Official CLI changelog](https://github.com/google-antigravity/antigravity-cli/blob/1.3.1/CHANGELOG.md)
- [Official Antigravity CLI headless guide](https://www.antigravity.google/docs/cli/headless/), which documents `agy models` separately as the model-slug listing command
- [Research 353](./353-antigravity-headless-adaptation-to-current-official.md) for the frozen `1.2.11` boundary and the release-notes behavioral-authority ruling
- [Contract 029](../knowledge/contracts/029-interface-version-qualification-and-compatibility.md)
- [`selection.rs`](../../crates/swallowtail-adapter-antigravity/src/selection.rs) and the frozen `antigravity-cli-1.1.9` decoder corpus
