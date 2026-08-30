# 261 Claude Code 2.1.251 Identity

Status: promoted
Owner: Tom
Date: 2026-08-30
Card: g05 batch 017

## Question

Is official npm `@anthropic-ai/claude-code` `latest` = `2.1.251` a
compatible extension of headless `2.1.220..=2.1.241` and response-only
`2.1.227..=2.1.241`, a new milestone, or a stop? Headless and
response-only stay one family. Watcher flags stay unmapped.

## Remaining AllowUnverified rank

Named family only. This run does not rank other families.

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Claude Code | installed `2.1.251` | headless `2.1.220..=2.1.241`; response-only `2.1.227..=2.1.241` | operator-named family; official npm `latest` is `2.1.251` |

Do not flatten this family onto Claude Agent ACP. Do not split headless and
response-only into separate currentness runs. Do not map watcher MCP,
settings, Stop-hook, or skill surfaces. Do not widen maximum-turn or other
feature-specific exact version sets.

## Method

Compared npm `@anthropic-ai/claude-code@2.1.251` and published
intermediates `2.1.242`, `2.1.243`, `2.1.245`, `2.1.246`, `2.1.247`,
`2.1.248`, and `2.1.250` to the frozen `2.1.241` identity corpus, the
`2.1.220` headless decoder specimen, the `2.1.227`/`2.1.228`
response-only specimens, extracted official wrapper tarballs, extracted
official darwin-arm64 binaries (`--version` and `--help`), extracted
official linux-x64 binaries (digest only; not executed on macOS), the
installed host binary, and GitHub `CHANGELOG.md` for `2.1.242` through
`2.1.251`.

No provider prompt, install, update, or live stream. Official artifacts
were extracted to `/tmp` only. Host `claude` was observed and not
replaced.

## Identity

| Fact | Value |
| --- | --- |
| npm latest | `2.1.251` (published 2026-08-28T15:34:26.421Z) |
| npm integrity | `sha512-eG+ZPPpW2Dbmnntf1Fz9/T9ewS8I8SKfc1tcU2PqSwmftfjRPP7BXPaCyLuZ8kvgTdiPnJi/2/JnTvTRieneEQ==` |
| npm shasum | `6c9d93e34244186783c5dcfd68605b0d0c6e16f7` |
| npm tarball SHA-256 | `44d28caf1711767c14a0388db56b13f49dbd8d3e1db635dd98aa3115c760cf27` |
| darwin-arm64 tarball SHA-256 | `cb3ecffa649ea20b78f3b7fe4a7395d7a225a510ae18521f6b65c669ecf4d9fd` |
| darwin-arm64 binary SHA-256 | `625869b01e0050f260b2980fac248fd9cef9e462612bded4ec9d3d49ff8969a5` |
| linux-x64 tarball SHA-256 | `7e2d6d13baccd64cd2d9a8781643ae23a48db009ad746d1df0cfec38d5ed0a44` |
| linux-x64 binary SHA-256 | `fd5f10ff0eb58daec04900466b143ea98aab50abf208a422bc008eaec13f61f7` |
| official `--version` | `2.1.251 (Claude Code)` |
| official `--help` SHA-256 | `5ff2e7a0bca8535fb9ec097fa0a21e9d6b735ed94104fa0d1f58ac73a841d52d` |
| host CLI | installed; SHA-256 match to official darwin-arm64 |
| host `--help` | byte-identical to extracted official darwin-arm64 |
| Agent SDK latest | `@anthropic-ai/claude-agent-sdk@0.3.251` |
| frozen SDK pin | `@anthropic-ai/claude-agent-sdk@0.3.220` (the `2.1.220` corpus) |
| Research 202 ceiling | `2.1.241` |

Published stables after previous ceiling `2.1.241`: `2.1.242`
(2026-08-24T19:16:14.767Z), `2.1.243` (2026-08-24T23:10:45.498Z),
`2.1.245` (2026-08-25T04:45:52.102Z), `2.1.246`
(2026-08-25T19:17:34.421Z), `2.1.247` (2026-08-26T18:02:01.046Z),
`2.1.248` (2026-08-27T20:35:36.671Z), `2.1.250`
(2026-08-27T22:27:48.314Z), `2.1.251`. Unpublished gaps: `2.1.244` and
`2.1.249`. First unpublished later stable is `2.1.252`. Dist-tag
`stable` remains `2.1.236`; this family's documented channel is npm
`latest`.

The npm package remains an installer wrapper. `cli-wrapper.cjs`,
`install.cjs`, `bin/claude.exe`, `LICENSE.md`, and `README.md` are
byte-identical to `2.1.241`. `package.json` changes only the version pin
and optionalDependencies platform packages. `sdk-tools.d.ts` adds
unmapped Read `artifactRead` / page-image extras and MCP stored fields.
That is not selected stream-JSON.

Official extracted `2.1.241` darwin-arm64 `--help` is byte-identical to
the frozen `2.1.241` linux-x64 dump
(`71ad650f59e08ae40ede14c534db4f49d8590ee5a4f92f6da2882d3a5560fea6`).
Official extracted `2.1.251` help is not byte-identical to that dump.

## Protocol comparison

Selected Swallowtail argv flags are present and unchanged:

- `-p` / `--print`
- `--input-format text` (`text`, `stream-json`)
- `--output-format stream-json` (`text`, `json`, `stream-json`)
- `--verbose`
- `--no-session-persistence`
- `--model`
- `--effort` (`low`, `medium`, `high`, `xhigh`, `max`)
- `--permission-mode plan` (headless)
- `--tools Read,Glob,Grep` (headless) or `--tools ""` (response-only)
- `--setting-sources user,project,local` (headless)
- `--safe-mode`, `--disable-slash-commands`, `--no-chrome`,
  `--prompt-suggestions false` (response-only)
- `--mcp-config` and `--strict-mcp-config`

Help additions stay unused: `--restricted`, `--all`, `attach`, `logs`,
`stop`/`kill`, `respawn`, `rm`. `--bg` wording now names those commands.
Prior unused flags stay unused: `--bare`, `--brief`, `--cloud`,
`--include-hook-events`, `--include-partial-messages`,
`--forward-subagent-text`, `--json-schema`, `--max-budget-usd`.

Changelog extras stay unmapped except already-mapped bugfixes:
`--strict-mcp-config` no longer prompts unused `.mcp.json` servers, and
the command sandbox honors `--setting-sources`. The `2.1.251`
`--input-format stream-json` merge fix is not selected; both routes send
`--input-format text`. GitHub changelog has no `2.1.242` heading;
`2.1.250` is bug-fix only. `2.1.248` adds `--restricted`. Watcher
candidate flags remain present and unmapped.

g03.068's fail-closed response-only validator still applies. This card
did not capture a live stream. Feature-specific exact version sets stay
on the `2.1.220..=2.1.241` probed points.

## Segment decision for card 018

Compatible extension on both axes. Same behaviors
`claude-code.headless.stream-json.v1` and
`claude-code.response-only.stream-json.v1`. Keep AllowUnverified.

Raise latest qualified from `2.1.241` to `2.1.251` on both existing
segments: headless `2.1.220..=2.1.251`, response-only
`2.1.227..=2.1.251`. Qualify published intermediates `2.1.242`,
`2.1.243`, `2.1.245`, `2.1.246`, `2.1.247`, `2.1.248`, and `2.1.250`.
Exclude unpublished `2.1.244` and `2.1.249`. After qualification,
synthetic later-stable UnverifiedNewer is `2.1.252`.

Do not flatten onto Claude Agent ACP. No new public operation. No
provider prompt. Do not advertise watcher support. Do not widen
maximum-turn or other feature-specific exact version sets.

## Sources

- npm `@anthropic-ai/claude-code@2.1.242` through `@2.1.251`
- official `@anthropic-ai/claude-code-darwin-arm64@2.1.241` and `@2.1.251`
- official `@anthropic-ai/claude-code-linux-x64@2.1.241` and `@2.1.251`
- [CHANGELOG.md](https://github.com/anthropics/claude-code/blob/main/CHANGELOG.md)
- frozen `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.241/`
