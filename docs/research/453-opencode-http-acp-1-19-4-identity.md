# 453 OpenCode HTTP and ACP 1.19.4 Identity

Status: promoted
Date: 2026-10-26

Official npm `opencode-ai` `latest` and GitHub stable `v1.19.4` agree.
`opencode.http` and `opencode.acp` both extend through `1.19.4` on their
existing behavior revisions. The claims stay unflattened.

## Question

Do the published stables after qualified `1.18.35` keep the selected HTTP
route surface and the selected ACP behavior, or does one hop change identity?

## Rank

| Surface | Host | Official | Boundary at observation | Result |
| --- | --- | --- | --- | --- |
| `opencode.http` / `opencode.server` | absent | npm and GitHub `1.19.4` | `1.18.35`, AllowUnverified | compatible extension |
| `opencode.acp` / `opencode.executable` | absent | same package and tag | `1.18.35`, AllowUnverified | compatible extension |

`opencode` was not on `PATH`. No binary was executed. Missing host is not a
gap. HTTP MCP live honouring stays exact `1.18.18` and does not transfer.

## Method

Re-probed npm `opencode-ai` dist-tag `latest` and GitHub
`anomalyco/opencode` releases on 2026-10-26. Both named stable `1.19.4`,
published `2026-10-22T13:03:41Z`. Preview, next, and beta tags were ignored.
No prerelease sits between `1.18.35` and `1.19.4`.

Published stables after the ceiling, in order: `1.19.0` (`2026-10-11`),
`1.19.1` (`2026-10-15`), `1.19.2` (`2026-10-16`), `1.19.3` (`2026-10-16`),
`1.19.4` (`2026-10-22`). No stable `1.18.36` or later `1.18` patch exists.
`1.19.5` was not published.

Downloaded npm `opencode-ai` and `opencode-linux-x64` tarballs and the
standard GitHub tag archives `archive/refs/tags/vX.tar.gz` for `1.18.35`
through `1.19.4`. npm has no `gitHead`. Tag commit and release
`target_commitish` differ, so source evidence is the tag archive, not the
API tarball. Linux x64 release-asset digests are the GitHub release API
digests; those binaries were not downloaded again. The `1.18.35` archive
SHA-256 `3092a7b9f55d80c42a9c1bb2e2cc0a9961316673faf92b741577257a1576dd59`
and canonical tree `616f4afe54525f9d0a7cdcbf0818f1f00c6cba722daaa7f1b4f9c2deef1216b2`
match Research 376.

Canonical tree hash: sort relative paths bytewise; each line is
`path<TAB>sha256:<file>` or `path<TAB>symlink:<target>` plus LF; SHA-256 the
UTF-8. Path-list hashes are the newline-joined paths with a trailing
newline. The empty list hashes to SHA-256 of the empty string.

No prompt, login, live server, catalogue, session, or provider contact.

Fixtures:

- `crates/swallowtail-adapter-opencode/tests/fixtures/opencode-1.19.4/`
- `crates/swallowtail-adapter-opencode/tests/fixtures/opencode-acp-1.19.4/`

## Identities

| Version | npm `opencode-ai` SHA-256 | Tag commit | Source archive SHA-256 | Canonical tree | Files |
| --- | --- | --- | --- | --- | ---: |
| `1.18.35` | `4d3408d0950d70cf870efe3f86e8bd0271d8149bea0767008d42de14f0986a04` | `53d1eabb61e21162157817bf677da0a4ad3332e3` | `3092a7b9f55d80c42a9c1bb2e2cc0a9961316673faf92b741577257a1576dd59` | `616f4afe54525f9d0a7cdcbf0818f1f00c6cba722daaa7f1b4f9c2deef1216b2` | 6568 |
| `1.19.0` | `ebb8bf46c6a3ac363b568296dccfeda75cdb4360db097edd43e947b1c742b1bd` | `733b2c627e5aec39cb33cc22373bdce33053ef2c` | `5dfcfffdf1df64d4a2e8e0e034b5128eb2376cab6d6d34ed808108aac4ce0d78` | `82ca5e56310fd21977a588911f6708ecba8396f1bb52b9cf5d7e8ed0eb2f0cda` | 6574 |
| `1.19.1` | `b64566f66b0e46eec2d54512a35ece43a8391c89dd815a0fc09f9c881590b571` | `4a929747483f9e19b05c7cf944fc0aca6e5111b0` | `d96b32749ab8e96fa553086d9d03c0356022a3af312911ed065bd69f22c261be` | `989fa05f7347be8aacdb6bb4c2725116f55ae8e29b02d75cd71314511be83148` | 6572 |
| `1.19.2` | `4cc647226daff4d19520597f76b94ce07c63cd34bd05730597c375e3797ea440` | `7901b7d325c34852caa84fc2c325f3a08dd7a473` | `d55e90544ff8392149903c33a7b059e0c748851c67043b8323915bfe1848af87` | `942e41509a9cf958a79089ff3072838ead3b1a6ab6a8ab8866a4520c4f986d57` | 6574 |
| `1.19.3` | `0ca1a05ab99438bdfbcb40ee3f82ebd46f57679db40ab3492ce8545b106990dc` | `e3c4eb242585dcd77be1a867391b7c25fd47ce52` | `8395f12e26df2d16610760ae224daf87da1d253a9883bf230be2c53984fe99ea` | `0fcdf1dfa5dec44ad05a2f14e264fe0dd37cbea395d04ae217c84b071e2f7aca` | 6574 |
| `1.19.4` | `2ceca45c7e0bbd50ddd3f4d6d651ed43645647d74279d656e7c7747a4b6a9137` | `148842ef7cb1bc34192b3fff57d0f9721ab9c801` | `2e553ed005b2df4a7efb9edd26db8fc3caafb1b4dfc3c5c3869192334584d8bb` | `ae0d6884639fda9deb79082e5b27483c9b6a6f45fb8868dadd89714722cbf287` | 6576 |

| Hop | Added | Removed | Changed | Identical |
| --- | ---: | ---: | ---: | ---: |
| `1.18.35` → `1.19.0` | 6 | 0 | 132 | 6496 |
| `1.19.0` → `1.19.1` | 0 | 2 | 66 | 6566 |
| `1.19.1` → `1.19.2` | 2 | 0 | 41 | 6591 |
| `1.19.2` → `1.19.3` | 0 | 0 | 36 | 6598 |
| `1.19.3` → `1.19.4` | 2 | 0 | 46 | 6588 |

npm `opencode-ai` keeps four files. `LICENSE`, `bin/opencode.exe`, and
`postinstall.mjs` are byte-identical; only `package.json` changes. Each
`opencode-linux-x64` hop changes both `bin/opencode` and `package.json`.
`1.19.4` also moves `@ff-labs/fff-bun` from `0.9.4` to `0.10.5`. The Linux
release asset grows from about 58MB to about 76MB. That is a bundled Bun
runtime bump, not a protocol change.

## Selected protocol

HTTP selected routes stay `global.health`, `provider.list`,
`session.create`, `session.prompt_async`, `event.subscribe`, `session.abort`,
`session.delete`, `session.list`, `session.status`, `session.get`, and
`session.messages`. Declaration and handler files for those routes are
byte-identical from `1.18.35` through `1.19.4`. Selected OpenAPI path
objects are byte-identical. The path set stays 162 paths. `info.version`
stays `1.0.0`.

OpenAPI document hash changes twice, outside the selected paths:
`1.19.0` edits the `Config.autoupdate` description; `1.19.3` adds optional
`ProviderConfig.reasoning_options`. Swallowtail does not write OpenCode
config. Catalogue decoding uses reported variant keys. When
`reasoning_options` is absent, the catalogue path is unchanged.

`1.19.1` stops session sharing. `POST /session` still returns the created
session, and Swallowtail reads `id` and `version` only. Swallowtail does not
set share mode and does not call share or unshare. The share endpoints
remaining in the OpenAPI document are not selected operations.

ACP sources under `packages/opencode/src/acp/`, `cli/cmd/acp.ts`, and the
MCP config/HTTP files are byte-identical across the six points.
`@agentclientprotocol/sdk` stays `0.21.0`. ACP session creation does not
call the removed share helper.

Classified outside the selected surface: CLI OpenCode 2 upgrade and 1.x
update checks, a PTY origin check, Copilot `reasoningOpaque` replay,
generated SDK types, package version metadata, the share discontinuation,
and the Bun pin. Each interesting implementation delta has a recorded
classification. None adds a selected public operation or narrows the
mapped HTTP or ACP contract.

## Decision

Compatible extension. Not an identity stop.

- `opencode.http.server-window-1` stays `AllowUnverified` on
  `opencode.server`. Keep `1.18.11..=1.18.35` at
  `opencode.http-sse.surface-19`. Add `1.19.0..=1.19.4` on the same
  behavior, both Maintained. A single `1.18.11..=1.19.4` interval would
  qualify unpublished `1.18.36`.
- `opencode.acp.executable-window-1` stays `AllowUnverified` on
  `opencode.executable`. Deprecated `1.18.18..=1.18.30` stays. Maintained
  `opencode.acp-v1.client-mcp-servers-v2` stays `1.18.31..=1.18.35` and
  adds `1.19.0..=1.19.4`.
- `1.18.36` is incompatible. `1.19.5` is `UnverifiedNewer`.
- Historical gaps `1.14.52`, `1.15.8`, `1.15.14`, `1.16.1`, `1.16.3`, and
  `1.17.21` stay.
- The two axes are not flattened. HTTP MCP live honouring stays exact
  `1.18.18`.

## Sources

- npm registry `opencode-ai` and `opencode-linux-x64`, dist-tag `latest`
- GitHub `anomalyco/opencode` stable releases and tag archives
- Frozen fixtures named above
- Prior ceiling evidence: Research 376 and Research 377
