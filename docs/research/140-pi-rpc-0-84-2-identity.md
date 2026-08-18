# 140 Pi RPC 0.84.2 Identity

Status: promoted
Owner: Tom
Date: 2026-08-18
Card: g03 batch 254

## Question

After g03.082 qualified Claude Agent ACP through `0.69.0`, which
AllowUnverified family should move first, and are host Pi `0.83.0` and
official npm/GitHub `@earendil-works/pi-coding-agent` `0.84.2` a compatible
extension of `pi.package` through exact `0.83.0`, a new private milestone,
or a stop?

## Remaining AllowUnverified rank

Claude Agent ACP is done. Remaining families have host still on a qualified
bound (registry newer only), Research 127 numbers unless noted:

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Pi RPC | `0.83.0` | published points through `0.83.0` | named next after Claude Agent ACP |
| 2 | Qwen headless | `0.21.2` | through `0.21.2` | later family |
| 3 | Antigravity | `1.1.9` | exact `1.1.9` | later family |

Gemini stays deferred. Do not flatten this family onto `oh-my-pi.package`.

Research 127 already classified Pi RPC as visible unverified-newer: host
`0.83.0`, npm then `0.84.2` (2026-08-14), exact published points through
`0.83.0`. Official npm `latest` is still `0.84.2` on 2026-08-18. Leaving
that point UnverifiedNewer would skip useful-newer support.

## Method

Compared host `pi --version`, npm `@earendil-works/pi-coding-agent`, GitHub
tags `v0.83.0` through `v0.84.2`, and selected git blobs
`packages/coding-agent/docs/rpc.md`, `src/modes/rpc/rpc-types.ts`,
`src/modes/rpc/rpc-mode.ts`, `src/modes/rpc/jsonl.ts`,
`src/core/session-cwd.ts`, plus `src/modes/json-event.ts` from `0.84.0`.

No provider prompt. No live RPC session. The host install was not replaced.

## Identity

| Surface | Version | Evidence |
| --- | --- | --- |
| Host CLI | `0.83.0` | Homebrew npm global; `dist/cli.js` SHA-256 `af302f231437eaf6f37691bce4b34234fcb626bcb5eb3910d4fc3f6519bf78ca`; size 681; shebang `#!/usr/bin/env node`; gitHead `845d6ff1f6643aba440341cce877ce1c43ebbc39` |
| Official npm/GitHub latest | `0.84.2` | published 2026-08-14T10:09:06.966Z; integrity `sha512-l4E+B7hgXKWddRo8bC/eSue2aWZjEgJ9xIpf5p0Og+lq8a2TArCwJ0HCoCPCgaBP/tN4zbYH/wOwvx9pJpeLCA==`; gitHead/tag `914cf1472e715297caa30db4b9535d534a9eb718`; tarball SHA-256 `95b899cd7b1a0c1f0174c7bf33ab427435e3553a7d1f4756661aa9c7f1a68ffa` |

Published stables after previous ceiling `0.83.0`: `0.84.0`, `0.84.1`,
`0.84.2`. npm has no `0.83.1` and no `0.84.3`. GitHub tags match npm
gitHead for `0.83.0`, `0.84.1`, and `0.84.2`. npm `0.84.0` gitHead
`8199aca40c9cf27aff3de7ba852e420985a54bf5` is one commit after GitHub tag
`a5f43bf8aff3c55752432655f7334e3dafd1e256`; selected RPC blobs match.

## Selected protocol

`rpc-types.ts` is byte-identical from `0.81.0` through `0.84.2`
(`5957eebc717658694592b27d047a20089851e7ae`). `jsonl.ts` strict-LF framing
is byte-identical from `0.83.0` through `0.84.2`. `session-cwd.ts` is
byte-identical across that span; load/resume stay blocked.

Selected commands remain `prompt`, `steer`, `follow_up`, `abort`,
`get_state`, `get_available_models`, `set_auto_compaction`,
`set_auto_retry`, `set_steering_mode`, and `set_follow_up_mode`. Direct
`bash`, `switch_session`, `fork`, `clone`, and extensions stay unmapped.

`0.84.0` changes selected streaming wire: RPC `message_update` drops
cumulative `message` and `assistantMessageEvent.partial`, emitting only
`assistantMessageEvent` deltas. Swallowtail already maps that event from
`assistantMessageEvent.type` and `delta` only. `rpc-mode.ts` routes events
through new `toJsonEvent`. Catalogue `get_available_models` reads a
snapshot instead of awaiting refresh.

`0.84.2` restores unused top-level `usage` on `message_update`. Qualified
usage still comes from `message_end`. Extra `usage` is ignored.

Selected argv flags remain in extracted `0.84.2` `dist/cli/args.js`:
`--mode rpc`, `--no-session`, `--offline`, `--provider`, `--model`,
`--tools`, `--no-extensions`, `--no-skills`, `--no-prompt-templates`,
`--no-themes`, `--no-context-files`, `--no-tools`. Unused deltas include
`--use-theme` and `defaultTools`.

## Decision

Compatible extension of the mapped strict-LF subset, with one new private
milestone for the `0.84.0` streaming-event shape. Do not close
`0.83.0..=0.84.2`; unpublished `0.83.1` would become qualified.

- Keep baseline `0.80.10` and claim id `pi.rpc.package-window-2`.
- Keep unpublished gaps `0.80.11`, `0.81.2`, `0.82.2`, and `0.83.1`
  incompatible.
- Mark exact `0.83.0` Deprecated on
  `pi.rpc.strict-lf-v0.83.0-bash-extension-hook`.
- Add Maintained `0.84.0..=0.84.2` on
  `pi.rpc.strict-lf-v0.84.0-message-update-delta`.
- Raise `PI_PACKAGE_LATEST_QUALIFIED_VERSION` to `0.84.2`.
- Synthetic later-stable UnverifiedNewer is `0.84.3`.
- Decoder specimen remains `pi-rpc-0.80.10`. Frozen
  `pi-rpc-0.80.10-0.83.0` compatibility rows stay unchanged.

Card 255 owns the claim change.

## Sources

- Host `pi --version` on 2026-08-18
- npm `@earendil-works/pi-coding-agent@0.84.2`
- [GitHub `v0.84.2`](https://github.com/earendil-works/pi/releases/tag/v0.84.2)
- git tags `v0.83.0` through `v0.84.2` and npm gitHead `8199aca` for `0.84.0`
