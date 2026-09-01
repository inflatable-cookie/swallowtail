# 273 Claude Code 2.1.257 Identity

Status: promoted
Owner: Tom
Date: 2026-09-01
Card: g05 batch 046

## Question

Is official npm `@anthropic-ai/claude-code` `latest` = `2.1.257` a
compatible extension of headless `2.1.220..=2.1.252` and response-only
`2.1.227..=2.1.252`, a new milestone, or a stop? Headless and
response-only stay one family. Watcher flags stay unmapped. Exact
`2.1.251` watcher help/digest/live authorization stays unwidened.

## Remaining AllowUnverified rank

Named family only. This run does not rank other families.

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Claude Code | installed `2.1.257` | headless `2.1.220..=2.1.252`; response-only `2.1.227..=2.1.252` | operator-named family; official npm `latest` is `2.1.257`; Research 271 remaining family after Claude Agent ACP `0.73.0` |

Do not flatten this family onto Claude Agent ACP. Do not split headless and
response-only into separate currentness runs. Do not map watcher MCP,
settings, Stop-hook, or skill surfaces. Do not widen watcher help, digest,
or live authorization. Do not widen maximum-turn or other feature-specific
exact version sets.

## Method

Compared npm `@anthropic-ai/claude-code@2.1.257` to the frozen `2.1.252`
identity corpus, the `2.1.220` headless decoder specimen, the
`2.1.227`/`2.1.228` response-only specimens, extracted official wrapper
tarballs, hashed official darwin-arm64 and linux-x64 binaries (not
executed), the installed host binary after proving byte-identity with
official darwin-arm64, GitHub tag `v2.1.257`, and GitHub `CHANGELOG.md`
heading `2.1.257`.

No provider prompt, install, update, or live stream. Official artifacts
were extracted to `/tmp` only. Host `claude` was observed and not
replaced. Downloaded official binaries were hashed and not executed. Host
`--help` is official `2.1.257` help because the host native SHA-256 equals
the official darwin-arm64 package `claude`.

## Identity

| Fact | Value |
| --- | --- |
| npm latest | `2.1.257` (published 2026-09-01T17:15:33.223Z) |
| npm integrity | `sha512-JzpBQDzbEV+IKV9lIs/SSRIdHGrAmQXhNScoz9PZgdjnatrVnbsBXRDrF26qBBBph38pA/39d+BhDpf+7RwkwA==` |
| npm shasum | `5aa17a093a628f0030c691ed0e11bb50e3228c59` |
| npm tarball SHA-256 | `e11188b92a6198945329e4e2657ebff206fbc014b3e5fc95644f76b62300ad5d` |
| GitHub tag | `v2.1.257` (published 2026-09-01T17:53:52Z; lightweight commit `a1e64dc407dd57dfb4ea283b0f8049adf3eabee5`) |
| darwin-arm64 tarball SHA-256 | `54c80ce110673637cf932dee41a02f31c95ad1a8bd1455adf480a9a271cdb54a` |
| darwin-arm64 binary SHA-256 | `64590d7d9d9c189d33fb3dfa58c5408eaf2a10fe556bd84155d95efaab46b60e` |
| darwin-arm64 binary size | 199011264 |
| linux-x64 tarball SHA-256 | `7e53dc103c832c4a34bb3f3a515f8141d9cd4bd19fd2fecd5698030e30a589a2` |
| linux-x64 binary SHA-256 | `9a64bda9d8722a1fa05bef9a5961d07e0331b99597eda9e2f6a732f3a0ff7f05` |
| linux-x64 binary size | 215469464 |
| official `--version` | `2.1.257 (Claude Code)` |
| official `--help` SHA-256 | `a0ab4f1df36388fba86563a10839c020cc7dcb13cec2311c336aebe6963db0a1` |
| host CLI | installed `2.1.257`; SHA-256 match to official `2.1.257` darwin-arm64 |
| host `--help` | byte-identical to official `2.1.257` darwin-arm64 |
| Agent SDK latest | `@anthropic-ai/claude-agent-sdk@0.3.257` |
| frozen SDK pin | `@anthropic-ai/claude-agent-sdk@0.3.220` (the `2.1.220` corpus) |
| Research 266 ceiling | `2.1.252` |

Published stables after previous ceiling `2.1.252`: `2.1.257` only.
Unpublished in the hop: `2.1.253`, `2.1.254`, `2.1.255`, `2.1.256`.
Existing unpublished gaps in the window: `2.1.244` and `2.1.249`. First
unpublished later stable after official latest is `2.1.258`. Dist-tag
`stable` remains `2.1.236`; this family's documented channel is npm
`latest`. Do not infer compatibility from `latest` alone.

The npm package remains an installer wrapper. Wrapper file count stays 7.
`cli-wrapper.cjs`, `install.cjs`, `bin/claude.exe`, `LICENSE.md`, and
`README.md` are byte-identical to `2.1.252`. `package.json` changes only
the version pin and optionalDependencies platform packages.
`sdk-tools.d.ts` is not byte-identical: SkillCreate comment wording,
ArtifactPublish `note` removed, REPL `result` made optional. Those SDK
types are not selected stream-JSON.

darwin-arm64 and linux-x64 package file counts stay 4. LICENSE and README
are byte-identical to `2.1.252`. `claude` and `package.json` changed.
`package.json` is the version pin only.

Official host `--help` is not byte-identical to frozen `2.1.252`
(`5ff2e7a0bca8535fb9ec097fa0a21e9d6b735ed94104fa0d1f58ac73a841d52d`).
The dump adds `--system-prompt-snapshot` and expands `--bg` resume
wording.

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

Mapped deltas from `2.1.252`: none. Selected mapped flags, format
choices, and effort/permission enumerations stay.

Unmapped help deltas: `--system-prompt-snapshot` (new); `--bg` resume
wording (unused background session). Prior unused flags stay unused:
`--bare`, `--brief`, `--cloud`, `--include-hook-events`,
`--include-partial-messages`, `--forward-subagent-text`, `--json-schema`,
`--max-budget-usd`, `--restricted`, `--all`, `attach`, `logs`,
`stop`/`kill`, `respawn`, `rm`.

Unmapped changelog `2.1.257` extras: Claude Fable 5.1 default Fable model
alias (caller supplies `--model`); `--effort` lifts a new-model
default-effort hold for that session only (one-shot
`--no-session-persistence` print runs have no permanent hold); `claude -p`
waits if a Monitor the model armed is still running (Monitor unmapped;
selected `-p` help unchanged); project `defaultMode: bypassPermissions`
ignored (selected runs pass `--permission-mode plan`); `--disallowedTools`
dropped after settings reload under `allowManagedPermissionRulesOnly`
(flag unused); unbounded memory when non-JSONL is piped to
`-p --input-format stream-json` (selected routes send `--input-format
text`); remaining interactive, VSCode, cloud, Remote Control, MCP,
sandbox, telemetry, subagent, and background-session extras.

g03.068's fail-closed response-only validator still applies. This card
did not capture a live stream. Feature-specific exact version sets stay
on the `2.1.220..=2.1.241` probed points.

## Watcher audit

Official `2.1.257` help SHA-256 differs from the frozen exact `2.1.251`
watcher isolation and tool-admission digest. Watcher candidate flags
remain present and unmapped. Exact watcher version stays `2.1.251`. This
family does not copy `watcher-isolation.json` or
`watcher-tool-admission.json`, does not raise watcher help/digest/live
authorization, and does not treat route qualification as watcher
live-readiness. Official `2.1.257` is rejected at both watcher admission
seams. The watcher route remains behind its separate mechanism-change
gate.

## Segment decision for card 047

Compatible extension on both axes. Same behaviors
`claude-code.headless.stream-json.v1` and
`claude-code.response-only.stream-json.v1`. Keep AllowUnverified.

Raise latest qualified from `2.1.252` to `2.1.257` on both existing
segments: headless `2.1.220..=2.1.257`, response-only
`2.1.227..=2.1.257`. Qualify published `2.1.257`. There is no published
intermediate between `2.1.252` and `2.1.257`. Exclude unpublished
`2.1.244` and `2.1.249`. After qualification, also exclude hop-skipped
unpublished `2.1.253`, `2.1.254`, `2.1.255`, and `2.1.256`. Synthetic
later-stable UnverifiedNewer is unpublished `2.1.258`.

Do not flatten onto Claude Agent ACP. No new public operation. No
provider prompt. Do not advertise watcher support. Do not widen
maximum-turn or other feature-specific exact version sets.

## Sources

- npm `@anthropic-ai/claude-code@2.1.257`
- official `@anthropic-ai/claude-code-darwin-arm64@2.1.257`
- official `@anthropic-ai/claude-code-linux-x64@2.1.257`
- GitHub release [`v2.1.257`](https://github.com/anthropics/claude-code/releases/tag/v2.1.257)
- [CHANGELOG.md](https://github.com/anthropics/claude-code/blob/v2.1.257/CHANGELOG.md)
- frozen `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.252/`
