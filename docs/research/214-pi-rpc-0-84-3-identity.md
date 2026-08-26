# 214 Pi RPC 0.84.3 Identity

Status: promoted
Owner: Tom
Date: 2026-08-26
Card: g04 batch 187

## Question

Is official npm `@earendil-works/pi-coding-agent` `latest` = `0.84.3` a
compatible extension of `pi.package` `0.84.0..=0.84.2` on
`pi.rpc.strict-lf-v0.84.0-message-update-delta`, a new private milestone,
or a stop?

## Remaining AllowUnverified rank

Named family only. This run does not rank other families.

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Pi RPC | not installed | published points through `0.84.2` | operator-named family; official npm `latest` is `0.84.3` |

Do not flatten this family onto `oh-my-pi.package` (`@oh-my-pi/pi-coding-agent`
latest at observation: `18.0.6`). Do not raise `pi.sdk-sidecar.package`.
Gemini stays deferred.

## Method

Compared npm `@earendil-works/pi-coding-agent@0.84.3` to the frozen
`0.84.2` identity corpus, GitHub tags `v0.84.2` and `v0.84.3`, selected git
blobs `packages/coding-agent/docs/rpc.md`, `src/modes/rpc/rpc-types.ts`,
`src/modes/rpc/rpc-mode.ts`, `src/modes/rpc/jsonl.ts`,
`src/core/session-cwd.ts`, `src/modes/json-event.ts`, and
`src/cli/args.ts`, plus extracted official `dist` for those surfaces and
`CHANGELOG.md`.

No provider prompt. No live RPC session. Host `pi` was not on PATH and
was not installed. Missing install is not a gap. Official artifacts stayed
in `/tmp`.

## Identity

| Surface | Version | Evidence |
| --- | --- | --- |
| Host CLI | not installed | `pi` not on PATH |
| Official npm latest | `0.84.3` | published 2026-08-24T11:09:37.600Z; integrity `sha512-Yr2p9PubrbFZmYEPYI+C8KmZP9xlFuLDnAG64RtU0ZDgrdiXYWa+y7WGyJO5OlqPliOkVCMd9IzVszO3/t0D0w==`; shasum `c040a5c2cfacd996731ce302a323269f124c8bdc`; tarball SHA-256 `d07dc417f78a14dac376a878b6556b51961f118f79771ee375333dc51356bc75`; gitHead `bfb004d4418ff05c6f909eaaab856cbe75c1fde0` |
| GitHub tag | `v0.84.3` | commit `4e58f324fae8ebfa98a3d45181fb248072a2afac`; release published 2026-08-24T11:09:57Z |

Published stables after previous ceiling `0.84.2`: `0.84.3` only. npm has
no `0.84.4`. GitHub has no `v0.84.4`. Unpublished `0.83.1` remains absent.

npm gitHead is two commits after the GitHub tag
(`Add [Unreleased] section for next cycle`,
`fix: extract Windows release ZIPs in CI`). Those commits touch CI and
package changelogs only. Selected RPC source blobs match the tag.

Extracted `dist/cli.js` is byte-identical to `0.84.2`
(`840d1e8e689ed9e4937bcb00b9a810e02a8567d9afb10a47097f11ca93ea1521`,
size 710). The published bin now points at `dist/bundle/cli.js`
(SHA-256 `1c3a5094b54aae9ae98c66516ce8c6578140363d081471ca7e91f9cb8c23dc8a`,
size 629). That packaging change is not a mapped RPC surface.

## Selected protocol

`rpc-types.ts` stays `5957eebc717658694592b27d047a20089851e7ae` from
`0.81.0` through `0.84.3`. `rpc-mode.ts` and strict-LF `jsonl.ts` stay
byte-identical to `0.84.2`. `session-cwd.ts` is unchanged; load/resume stay
blocked.

Selected commands remain `prompt`, `steer`, `follow_up`, `abort`,
`get_state`, `get_available_models`, `set_auto_compaction`,
`set_auto_retry`, `set_steering_mode`, and `set_follow_up_mode`. Direct
`bash`, `switch_session`, `fork`, `clone`, and extensions stay unmapped.

`json-event.ts` now copies `id` and `toolName` onto
`assistantMessageEvent` when the event is `toolcall_start`. Swallowtail
already classifies `toolcall_start` as Progress and maps tools from
`tool_execution_*`. Extra fields stay unmapped. Text and thinking still
use `assistantMessageEvent.type` plus `delta`. Qualified usage still comes
from `message_end`.

Selected argv flags remain in extracted `0.84.3` `dist/cli/args.js`:
`--mode rpc`, `--no-session`, `--offline`, `--provider`, `--model`,
`--tools`, `--no-extensions`, `--no-skills`, `--no-prompt-templates`,
`--no-themes`, `--no-context-files`, `--no-tools`. Unused deltas include
`--`, `powershell`, `--use-theme`, and `defaultTools`.

Changelog extras stay unmapped: installer-managed `pi update`, bundled
Node runtime, PowerShell tool, `--` end-of-options, inherited model
catalogues, TUI keybindings, session sharing, and extension-load fixes.

## Decision

Compatible extension of the mapped strict-LF subset on the existing
`0.84.0` message-update-delta revision. No new milestone.

- Keep baseline `0.80.10` and claim id `pi.rpc.package-window-2`.
- Keep unpublished gaps `0.80.11`, `0.81.2`, `0.82.2`, and `0.83.1`
  incompatible.
- Keep exact `0.83.0` Deprecated on
  `pi.rpc.strict-lf-v0.83.0-bash-extension-hook`.
- Extend Maintained `0.84.0..=0.84.2` to `0.84.0..=0.84.3` on
  `pi.rpc.strict-lf-v0.84.0-message-update-delta`.
- Raise `PI_PACKAGE_LATEST_QUALIFIED_VERSION` to `0.84.3`.
- Synthetic later-stable UnverifiedNewer is `0.84.4`.
- Decoder specimen remains `pi-rpc-0.80.10`. Frozen
  `pi-rpc-0.80.10-0.83.0` compatibility rows stay unchanged.
- Do not flatten onto Oh My Pi. Do not raise the SDK sidecar pin.

Card 188 owns the claim change.

## Sources

- npm `@earendil-works/pi-coding-agent@0.84.3`
- [GitHub `v0.84.3`](https://github.com/earendil-works/pi/releases/tag/v0.84.3)
- git tags `v0.84.2` and `v0.84.3`; npm gitHead `bfb004d`
- frozen `crates/swallowtail-adapter-pi/tests/fixtures/pi-rpc-0.84.2/`
