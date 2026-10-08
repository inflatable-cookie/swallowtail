# 407 ZCode Runtime 0.16.9 Source Identity

Status: promoted; identity discovery only, no qualification or claim change
Owner: Swallowtail worker
Date: 2026-10-08

## Question

What is the current official runtime target and exact artifact identity for
`zcode.app-server`, whose selected axis is `zcode.runtime`?

## Result

The current official desktop download is ZCode `3.14.4`. Its signed macOS
arm64 app contains `Contents/Resources/glm/zcode.cjs` with the runtime version
`0.16.9`. The `zcode-cli` package at official source tag `v3.14.3` is also
version `0.16.9`; its build script reads the root package version and injects
it as the CLI version. The source package version and the current signed
runtime agree.

This identifies the current target as `zcode.runtime 0.16.9` and freezes one
platform-specific artifact identity below. It does not change the existing
exact `0.16.3` `QualifiedOnly` claim. The app version `3.14.4`, build
`3.14.4.7912`, and npm wrapper `zcode-app-cli@3.14.4-32` are not the runtime
axis.

## Method

On 2026-10-08, read the official [install page](https://zcode.z.ai/en/docs/install)
and [release notes](https://zcode.z.ai/en/changelog). Both identify `3.14.4`
as current; the release notes date it to 2026-09-29. The changelog's release
metadata links the macOS arm64 installer and its version-scoped `latest.yml`
manifest. The manifest GET returned HTTP 200. Its `files` entry for the DMG
publishes a SHA-512 and size that match the downloaded artifact. The top-level
`path` and `sha512` in that manifest refer to the ZIP entry, not the DMG. The
manifest does not publish SHA-256. The DMG response also returned
`Last-Modified: 2026-09-29T03:53:32Z`, a multipart OSS ETag, and CRC64.

The official [ZCode GitHub repository](https://github.com/zai-org/ZCode)
advertises the vendor site. Read-only GitHub API and tag queries found source
release/tag `v3.14.3`, published 2026-09-24 at 10:54 UTC, at commit
`29628c9acdb81b703bbd4080c207a0e7ce5e276e`. That tag's
[`apps/zcode-cli/package.json`](https://github.com/zai-org/ZCode/blob/v3.14.3/apps/zcode-cli/package.json)
sets version `0.16.9`. Its
[`packages/cli/scripts/build.mjs`](https://github.com/zai-org/ZCode/blob/v3.14.3/apps/zcode-cli/packages/cli/scripts/build.mjs)
reads the root package version and injects it as `__CLI_VERSION__`. The
official site is at `3.14.4`, while the repository's latest source tag and
`main` remain `v3.14.3`; no `v3.14.4` source tag or source commit was found.

Downloaded the official macOS arm64 DMG to a fresh temporary directory,
verified its disk-image checksum, and mounted it read-only. Inspected the app
bundle and embedded runtime, verified the app's strict deep code signature,
then detached the image. No app, runtime, or provider process was executed.
No authentication, installation, host update, or provider interaction was
used. GitHub API pages were not accessible through the browser reader; the
same public metadata was obtained through direct unauthenticated HTTPS API
requests.

## Identity

| Surface | Exact observation |
| --- | --- |
| Official current product release | ZCode `3.14.4`, released 2026-09-29; official install page labels it latest |
| Official asset URL | <https://cdn-zcode.z.ai/zcode/electron/releases/3.14.4/macos-arm64/ZCode-3.14.4-mac-arm64.dmg> |
| Vendor manifest | <https://cdn-zcode.z.ai/zcode/electron/releases/3.14.4/macos-arm64/latest.yml>; changelog metadata points to this manifest for `darwin-aarch64` |
| Manifest DMG entry | `ZCode-3.14.4-mac-arm64.dmg`; SHA-512 (base64) `JPdNfNzhgvMFElXxzQNUjbYRISe7qVPeXoAFTc4EhOXonmxTpfjMuZD6QjFKu8AbJ6dF2lNWAvtYTdT9x/wl1g==`; size `255221085`; both match the downloaded DMG |
| Asset identity | 255,221,085 bytes; SHA-256 `d7a5ade455a1eab3ead02d0a6f7005a107788a65113eda38ed61d96f5a8bdeb1`; HTTP ETag `53E4BB8827DB504DB4788CDF35A8EA11-25` is multipart, not SHA-256 |
| Disk-image check | `hdiutil verify` reported a valid CRC32 checksum |
| Signed app | Bundle `dev.zcode.app`, version `3.14.4`, build `3.14.4.7912`; `codesign --verify --strict --deep` passed and the bundle satisfied its designated requirement |
| Publisher | `Developer ID Application: Beijing Knowledge Atlas Technology Joint Stock Company Limited (8A5X4JJ39T)`; Team ID `8A5X4JJ39T`; signature timestamp 2026-09-29 03:55:26; bundle CDHash `7866c324475be0c18d1fa15ef900c716a2db0b0f` |
| Selected runtime payload | `Contents/Resources/glm/zcode.cjs`; static version `0.16.9`; 14,820,968 bytes; SHA-256 `fad4c35c4c36ec210d8a06d3fa0e77de23c8545e2eb6ff90aea1eb38d1e6275f` |
| Source version | Official `zcode-cli` source package `0.16.9` at tag `v3.14.3`, source commit `29628c9acdb81b703bbd4080c207a0e7ce5e276e` |

The runtime payload is sealed by the signed app bundle. The version-scoped
vendor manifest corroborates the exact downloaded DMG with SHA-512 and size;
the recorded SHA-256 and extracted runtime hash identify the observed copy.
The `3.14.4` runtime has
version-level correlation to the official source package, but no
`3.14.4` source tag or commit is available to claim source-commit parity.
This observation covers macOS arm64 only; do not transfer these hashes to
other platform packages.

## Ordering And Next Gate

`0.16.3` and `0.16.9` are versions in the same `zcode-cli` package namespace.
Their SemVer ordering is meaningful (`0.16.3 < 0.16.9`), but this does not
establish compatibility or prove which intermediate runtime versions were
published. The official product release sequence is a separate axis, and the
numeric gap is not a hop ledger.

The next one-family qualification brief can target exact runtime `0.16.9` for
`zcode.app-server`. It must retain `QualifiedOnly` until selected-surface
evidence is complete, enumerate each official runtime point and artifact
between `0.16.3` and `0.16.9`, and compare the existing mapped app-server
surface using exact signed vendor artifacts. Do not substitute the desktop
version or wrapper package version. If source-level classification requires
the missing `3.14.4` source identity, Z.AI/ZCode release maintainers own that
input: request the source commit/tag and an immutable runtime-to-product
release manifest. The planner owns the qualification brief. No vendor request
was sent.

## Sources

- [Official ZCode install page](https://zcode.z.ai/en/docs/install)
- [Official ZCode release notes](https://zcode.z.ai/en/changelog)
- [Official ZCode source repository](https://github.com/zai-org/ZCode)
- [Official `v3.14.3` source release](https://github.com/zai-org/ZCode/releases/tag/v3.14.3)
- [Official `v3.14.3` `zcode-cli` package manifest](https://github.com/zai-org/ZCode/blob/v3.14.3/apps/zcode-cli/package.json)
- [Official `v3.14.3` CLI build version injection](https://github.com/zai-org/ZCode/blob/v3.14.3/apps/zcode-cli/packages/cli/scripts/build.mjs)
- [ZCode CDN artifact](https://cdn-zcode.z.ai/zcode/electron/releases/3.14.4/macos-arm64/ZCode-3.14.4-mac-arm64.dmg)
- [ZCode macOS arm64 `3.14.4` update manifest](https://cdn-zcode.z.ai/zcode/electron/releases/3.14.4/macos-arm64/latest.yml)
- [Research 369 currentness checkpoint](./369-all-route-version-currentness-checkpoint.md)
- [Research 126 original ZCode app-server qualification](./126-zcode-app-server-route-qualification.md)
