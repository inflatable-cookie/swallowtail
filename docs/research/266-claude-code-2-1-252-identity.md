# 266 Claude Code 2.1.252 Identity

Status: promoted
Owner: Tom
Date: 2026-09-01
Card: g05 batch 037

## Question

Is official npm `@anthropic-ai/claude-code` `latest` = `2.1.252` a
compatible extension of headless `2.1.220..=2.1.251` and response-only
`2.1.227..=2.1.251`, a new milestone, or a stop? Headless and
response-only stay one family. Watcher flags stay unmapped. Exact
`2.1.251` watcher help/digest/live authorization stays unwidened.

## Remaining AllowUnverified rank

Named family only. This run does not rank other families.

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Claude Code | installed `2.1.251` | headless `2.1.220..=2.1.251`; response-only `2.1.227..=2.1.251` | operator-named family; official npm `latest` is `2.1.252`; Research 265 selected this family alone |

Do not flatten this family onto Claude Agent ACP. Do not split headless and
response-only into separate currentness runs. Do not map watcher MCP,
settings, Stop-hook, or skill surfaces. Do not widen watcher help, digest,
or live authorization. Do not widen maximum-turn or other feature-specific
exact version sets.

## Method

Compared npm `@anthropic-ai/claude-code@2.1.252` to the frozen `2.1.251`
identity corpus, the `2.1.220` headless decoder specimen, the
`2.1.227`/`2.1.228` response-only specimens, extracted official wrapper
tarballs, extracted official darwin-arm64 binaries (`--version` and
`--help`), extracted official linux-x64 binaries (digest only; not
executed on macOS), the installed host binary, GitHub tag `v2.1.252`, and
GitHub `CHANGELOG.md` heading `2.1.252`.

No provider prompt, install, update, or live stream. Official artifacts
were extracted to `/tmp` only. Host `claude` was observed and not
replaced. Parser/help evidence used `env -i` with a throwaway `HOME`.

## Identity

| Fact | Value |
| --- | --- |
| npm latest | `2.1.252` (published 2026-08-31T17:07:28.168Z) |
| npm integrity | `sha512-ftoO0eLOZyEDrA3KDd7QZH5qdvToiTcoip3YdGGx8wzH4R9YUwHO+5VG01JDRn8u7MrRcXkf7FvbMYezEt0VyQ==` |
| npm shasum | `f5396b69ed26971a0e13205ebc760da7d98bf92e` |
| npm tarball SHA-256 | `e5e04447d3afdf70f7578f9d22217c530a0ef8c59ae2f78e32d1a6ea2fb3cafa` |
| GitHub tag | `v2.1.252` (published 2026-08-31T19:46:55Z; lightweight commit `f275fa282e76c5e5456912268f2c367a7f4f4797`) |
| darwin-arm64 tarball SHA-256 | `d11551a495051a745ee033160bc379e5a388e3e6d87666e9259da09a7d24049b` |
| darwin-arm64 binary SHA-256 | `b661c6a094fcc32656bf7c0071c5b45bf900b34d4f0a1ab3d78fd59aeba2c2c7` |
| darwin-arm64 binary size | 197220928 |
| linux-x64 tarball SHA-256 | `ecce38cb26f10215a98608c23ddaf4db6fe07bce651c0367617f8829569824fb` |
| linux-x64 binary SHA-256 | `a715a45105e593fc9808d035d77781f88480b9897975a9df41837f0c591bd4b3` |
| linux-x64 binary size | 214371672 |
| official `--version` | `2.1.252 (Claude Code)` |
| official `--help` SHA-256 | `5ff2e7a0bca8535fb9ec097fa0a21e9d6b735ed94104fa0d1f58ac73a841d52d` |
| host CLI | installed `2.1.251`; SHA-256 match to official `2.1.251` darwin-arm64 |
| host `--help` | byte-identical to extracted official `2.1.252` darwin-arm64 |
| Agent SDK latest | `@anthropic-ai/claude-agent-sdk@0.3.252` |
| frozen SDK pin | `@anthropic-ai/claude-agent-sdk@0.3.220` (the `2.1.220` corpus) |
| Research 261 ceiling | `2.1.251` |

Published stables after previous ceiling `2.1.251`: `2.1.252` only.
Unpublished gaps already in the window: `2.1.244` and `2.1.249`. First
unpublished later stable is `2.1.253`. Dist-tag `stable` remains
`2.1.236`; this family's documented channel is npm `latest`. Do not infer
compatibility from `latest` alone.

The npm package remains an installer wrapper. `cli-wrapper.cjs`,
`install.cjs`, `bin/claude.exe`, `LICENSE.md`, `README.md`, and
`sdk-tools.d.ts` are byte-identical to `2.1.251`. `package.json` changes
only the version pin and optionalDependencies platform packages.

Official extracted `2.1.252` darwin-arm64 `--help` is byte-identical to
the frozen `2.1.251` dump
(`5ff2e7a0bca8535fb9ec097fa0a21e9d6b735ed94104fa0d1f58ac73a841d52d`).

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

Mapped deltas from `2.1.251`: none. Help is byte-identical.

Unmapped changelog `2.1.252` extras: Bash task-output swap on some Macs;
always-allow save without `.claude/settings.local.json`; Remote Control
stall when `claude.ai` is degraded; large background-task failure output
exceeding API request size. Prior unused flags stay unused: `--bare`,
`--brief`, `--cloud`, `--include-hook-events`,
`--include-partial-messages`, `--forward-subagent-text`, `--json-schema`,
`--max-budget-usd`, `--restricted`, `--all`, `attach`, `logs`,
`stop`/`kill`, `respawn`, `rm`.

g03.068's fail-closed response-only validator still applies. This card
did not capture a live stream. Feature-specific exact version sets stay
on the `2.1.220..=2.1.241` probed points.

## Watcher audit

Official `2.1.252` help SHA-256 equals the frozen exact `2.1.251` watcher
isolation and tool-admission digest. Watcher candidate flags remain
present and unmapped. Exact watcher version stays `2.1.251`. This family
does not copy `watcher-isolation.json` or `watcher-tool-admission.json`,
does not raise watcher help/digest/live authorization, and does not
treat route qualification as watcher live-readiness. The watcher route
remains behind its separate mechanism-change gate.

## Segment decision for card 038

Compatible extension on both axes. Same behaviors
`claude-code.headless.stream-json.v1` and
`claude-code.response-only.stream-json.v1`. Keep AllowUnverified.

Raise latest qualified from `2.1.251` to `2.1.252` on both existing
segments: headless `2.1.220..=2.1.252`, response-only
`2.1.227..=2.1.252`. Qualify published `2.1.252`. There is no published
intermediate between `2.1.251` and `2.1.252`. Exclude unpublished
`2.1.244` and `2.1.249`. After qualification, synthetic later-stable
UnverifiedNewer is unpublished `2.1.253`.

Do not flatten onto Claude Agent ACP. No new public operation. No
provider prompt. Do not advertise watcher support. Do not widen
maximum-turn or other feature-specific exact version sets.

## Sources

- npm `@anthropic-ai/claude-code@2.1.252`
- official `@anthropic-ai/claude-code-darwin-arm64@2.1.252`
- official `@anthropic-ai/claude-code-linux-x64@2.1.252`
- GitHub release [`v2.1.252`](https://github.com/anthropics/claude-code/releases/tag/v2.1.252)
- [CHANGELOG.md](https://github.com/anthropics/claude-code/blob/v2.1.252/CHANGELOG.md)
- frozen `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.251/`
