# 302 Pi RPC 0.85.1 Identity

Status: promoted
Owner: Tom
Date: 2026-09-09
Task: g05.044

## Question

Is official npm `@earendil-works/pi-coding-agent` `latest` = `0.85.1` a
compatible extension of `pi.package` `0.84.0..=0.84.4` on
`pi.rpc.strict-lf-v0.84.0-message-update-delta`, a new private milestone,
or a stop?

## Remaining AllowUnverified rank

Named family only. This run does not rank other families.

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Pi RPC | installed `0.85.1` | published points through `0.84.4`; `0.83.1` gap | operator-named family; official npm `latest` is `0.85.1` |

Do not flatten this family onto `oh-my-pi.package` (`@oh-my-pi/pi-coding-agent`
latest at observation: `18.1.15`). Do not raise `pi.sdk-sidecar.package`.
Gemini stays deferred.

## Method

Compared official npm `@earendil-works/pi-coding-agent@0.84.4`, `0.85.0`,
and `0.85.1` tarballs extracted in `/tmp/pi-rpc-0.85.1-identity`. Built a
deterministic shipped-tree inventory (`dist-inventory.json`) because mapped
files differ and the package tree refactors. Compared GitHub tags `v0.84.4`,
`v0.85.0`, and `v0.85.1`, selected git blobs
`packages/coding-agent/docs/rpc.md`, `src/modes/rpc/rpc-types.ts`,
`src/modes/rpc/rpc-mode.ts`, `src/modes/rpc/jsonl.ts`,
`src/core/session-cwd.ts`, `src/modes/json-event.ts`, `src/cli/args.ts`,
and `src/core/agent-session.ts`, plus extracted official `dist` for those
surfaces, `dist/cli.js`, `dist/cli/setup.js`, `dist/main.js`, executing
`dist/bundle/cli.js`, and `CHANGELOG.md`.

No provider prompt. No live RPC session. Host `pi --version` `0.85.1` was
observed and not replaced. Official artifacts stayed in `/tmp` and were
not executed.

Official latest was rechecked immediately before this identity freeze:
npm `latest` and GitHub latest stable remain `0.85.1`. Unpublished `0.85.2`
is absent.

## Identity

| Surface | Version | Evidence |
| --- | --- | --- |
| Host CLI | `0.85.1` | SHA-256 `e6d7fcf36a239cf3746e67ddf4222081ac01a601b85a3ee688bdfe9c161d754c`; size 660; matches official `0.85.1` `dist/bundle/cli.js` |
| Official npm latest | `0.85.1` | published 2026-09-05T12:17:19.281Z; integrity `sha512-FGRN+OHbWaefBPGaTggAdLjrIHW+s2PzLyglz/5dfLzb9of7uuXMXYC0fJIeZTw+shS32o2cuQ9jF7YSDuL/oQ==`; shasum `4cd00f653c3dabeb193b46f511044e7fbfe0f947`; tarball SHA-256 `1f498729649bdce647d1160993b4d92bf3c614cc819213bee2f91dd34f2a7af4`; gitHead `d981de1229ef899957bbe968bc8dcda02a21f477` |
| GitHub tag | `v0.85.1` | commit `d981de1229ef899957bbe968bc8dcda02a21f477`; release published 2026-09-05T12:29:01Z |
| Intermediate npm | `0.85.0` | published 2026-09-04T10:18:05.208Z; tarball SHA-256 `a0895f70a9efd9dde2a69b9cee04cb3b7c5aab68f5d47aad92b63f27a4ca13c8`; gitHead `107d79f11072bbc8a3a757ed7fd69596bee7d68c` |
| Intermediate GitHub tag | `v0.85.0` | commit `107d79f11072bbc8a3a757ed7fd69596bee7d68c`; release published 2026-09-04T10:18:28Z |

Published stables after previous ceiling `0.84.4`: `0.85.0` and `0.85.1`.
npm has no `0.85.2`. GitHub has no `v0.85.2`. Unpublished `0.83.1` and
`0.84.5` remain absent.

npm gitHead matches the GitHub tag commit at both hops.

Shipped-tree counts: `0.84.4` 1044 files, `0.85.0` 1249, `0.85.1` 1056.
`0.84.4` → `0.85.0` adds 246 / removes 41 / changes 207. `0.85.0` →
`0.85.1` adds 20 / removes 213 / changes 48. The `0.85.0` experimental
server/client tree (172 files) is added then removed.

Mapped `rpc-types.js`, `rpc-mode.js`, `jsonl.js`, `session-cwd.js`, and
`json-event.js` are byte-identical through all three hops. Extracted
`args.js` at `0.85.1` is byte-identical to `0.84.4`; `0.85.0` only adds
unmapped `PI_SERVER_DIR` / `PI_SERVER_ID` help. `dist/cli.js` at `0.85.1`
delegates to `setupCli()` plus `main()`, equivalent to the `0.84.4` main
path. Host digest matches official `0.85.1` `dist/bundle/cli.js`.

## Selected protocol

Selected mapped commands remain `prompt`, `steer`, `follow_up`, `abort`,
`get_state`, `get_available_models`, `set_auto_compaction`,
`set_auto_retry`, `set_steering_mode`, and `set_follow_up_mode`. Direct
`bash`, `switch_session`, `fork`, `clone`, extensions, `clear_queue`, and
`compact` stay unmapped. RPC command count stays 33.

Selected argv flags remain in extracted `0.85.1` `dist/cli/args.js`:
`--mode rpc`, `--no-session`, `--offline`, `--provider`, `--model`,
`--tools`, `--no-extensions`, `--no-skills`, `--no-prompt-templates`,
`--no-themes`, `--no-context-files`, `--no-tools`. Standing-unused
help-unselected `--use-theme`, `defaultTools`, `--`, and `powershell` are
byte-identical carry-forwards from `0.84.4`.

`session.abort()` already awaited `waitForIdle()` at `0.84.4`. From
`0.85.0` it also calls `abortCompaction()` and `abortBranchSummary()`.
`rpc-mode.js` abort dispatch is byte-identical. Swallowtail sends
`set_auto_compaction false` and does not send `compact`, so that extra
cancel is a bugfix of already-mapped abort covering unmapped compaction.
`docs/rpc.md` abort wording changes at `0.85.0` and is identical at
`0.85.1`. Changelog `0.85.1` states the supported stdio RPC API is
unchanged.

Changelog extras stay unmapped: persistent Claude thinking effort,
fullscreen TUI controls, `SessionManager.inMemory`, inherited provider
and catalogue fixes, GPT-6 Astra, experimental server/client packaging,
and prompt-cache ttl.

## Decision

Compatible extension of the mapped strict-LF subset on the existing
`0.84.0` message-update-delta revision. No new milestone.

- Keep baseline `0.80.10` and claim id `pi.rpc.package-window-2`.
- Keep unpublished gaps `0.80.11`, `0.81.2`, `0.82.2`, `0.83.1`, and
  `0.84.5` incompatible.
- Keep exact `0.83.0` Deprecated on
  `pi.rpc.strict-lf-v0.83.0-bash-extension-hook`.
- Keep Maintained `0.84.0..=0.84.4` and add Maintained `0.85.0..=0.85.1`
  on `pi.rpc.strict-lf-v0.84.0-message-update-delta`.
- Raise `PI_PACKAGE_LATEST_QUALIFIED_VERSION` to `0.85.1`.
- Synthetic later-stable UnverifiedNewer is unpublished `0.85.2`.
- Decoder specimen remains `pi-rpc-0.80.10`. Frozen
  `pi-rpc-0.80.10-0.83.0` compatibility rows stay unchanged.
- Do not flatten onto Oh My Pi. Do not raise the SDK sidecar pin.

The claim edit is serial after this identity freeze. This record does not
edit production claims.

## Sources

- npm `@earendil-works/pi-coding-agent@0.84.4`, `@0.85.0`, `@0.85.1`
- [GitHub `v0.85.1`](https://github.com/earendil-works/pi/releases/tag/v0.85.1)
- [GitHub `v0.85.0`](https://github.com/earendil-works/pi/releases/tag/v0.85.0)
- git tags `v0.84.4`, `v0.85.0`, and `v0.85.1`; npm gitHeads `b79e4cc`,
  `107d79f`, `d981de1`
- frozen `crates/swallowtail-adapter-pi/tests/fixtures/pi-rpc-0.84.4/`
- frozen `crates/swallowtail-adapter-pi/tests/fixtures/pi-rpc-0.85.1/`
