# 331 Pi RPC 0.86.1 Identity

Status: promoted
Owner: Tom
Date: 2026-09-21
Task: operator-named currentness; no generation card

## Question

Is official npm `@earendil-works/pi-coding-agent` `latest` = `0.86.1` a
compatible extension of `pi.package` `0.85.0..=0.85.1` on
`pi.rpc.strict-lf-v0.84.0-message-update-delta`, a new private milestone,
or a stop?

## Remaining AllowUnverified rank

Named family only. This run does not rank other families.

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Pi RPC | not installed | published points through `0.85.1`; `0.83.1`, `0.84.5`, `0.85.2` gaps | operator-named family; official npm `latest` is `0.86.1` |

Do not flatten this family onto `oh-my-pi.package` (`@oh-my-pi/pi-coding-agent`
latest at observation: `18.2.7`). Do not raise `pi.sdk-sidecar.package`.
Gemini stays deferred.

## Method

Compared official npm `@earendil-works/pi-coding-agent@0.85.1`, `0.86.0`,
and `0.86.1` tarballs extracted in `/tmp/pi-rpc-0.86.1-identity`. Built a
deterministic shipped-tree inventory (`dist-inventory.json`) because mapped
files differ and the package tree refactors. Compared GitHub tags `v0.85.1`,
`v0.86.0`, and `v0.86.1`, selected git blobs
`packages/coding-agent/docs/rpc.md`, `src/modes/rpc/rpc-types.ts`,
`src/modes/rpc/rpc-mode.ts`, `src/modes/rpc/jsonl.ts`,
`src/core/session-cwd.ts`, `src/modes/json-event.ts`, `src/cli/args.ts`,
and `src/core/agent-session.ts`, plus extracted official `dist` for those
surfaces, `dist/cli.js`, `dist/cli/setup.js`, executing `dist/bundle/cli.js`
and `dist/bundle/cli-runtime.js` at `0.86.1`, and `CHANGELOG.md`.

No provider prompt. No live RPC session. Host `pi` was not on PATH; missing
install is not a gap. Official artifacts stayed in `/tmp` and were not
executed.

Official latest was rechecked immediately before this identity freeze:
npm `latest` and GitHub latest stable remain `0.86.1`. Unpublished `0.85.2`
and `0.86.2` are absent.

## Identity

| Surface | Version | Evidence |
| --- | --- | --- |
| Host CLI | not installed | `pi` not on PATH; missing install is not a gap |
| Official npm latest | `0.86.1` | published 2026-09-20T11:16:39.121Z; integrity `sha512-vZBuNfJnruxZyemZ3O05V0S/Ylze08ahFTIQ1Mik++gVdOevPl89gt/Uv0U97BPAJaj9cj6Vf9rcIgKtUrd0BA==`; shasum `b8e38a10c51d28e946ac2740f13c01358f886a52`; tarball SHA-256 `8dff93e6fa03e0d498e72a78d2c7bb5f094f5e06ee268e6abd000ba2984a0b6a`; gitHead `13cbf77df2396303013a41646bcfa77b4271ae56` |
| GitHub tag | `v0.86.1` | commit `13cbf77df2396303013a41646bcfa77b4271ae56`; release published 2026-09-20T11:19:20Z |
| Intermediate npm | `0.86.0` | published 2026-09-19T23:14:16.198Z; tarball SHA-256 `3f0f502f497c44888ddbd68a11897651b739eaabdf4b235e3a7ed9735353b0cd`; gitHead `ecac0a9c4edad3dac5d9f8b40e0c7db7a56471fc` |
| Intermediate GitHub tag | `v0.86.0` | commit `ecac0a9c4edad3dac5d9f8b40e0c7db7a56471fc`; release published 2026-09-19T23:15:37Z |

Published stables after previous ceiling `0.85.1`: `0.86.0` and `0.86.1`.
npm has no `0.85.2` or `0.86.2`. GitHub has no `v0.85.2` or `v0.86.2`.
Unpublished `0.83.1` and `0.84.5` remain absent.

npm gitHead matches the GitHub tag commit at both hops.

Shipped-tree counts: `0.85.1` 1056 files, `0.86.0` 1094, `0.86.1` 1100.
`0.85.1` → `0.86.0` adds 61 / removes 23 / changes 233 / identical 800.
`0.86.0` → `0.86.1` adds 10 / removes 4 / changes 41 / identical 1049.
Identical through all three hops: 793 files.

Mapped `rpc-types.js`, `jsonl.js`, `session-cwd.js`, `json-event.js`,
`docs/rpc.md`, `dist/cli.js`, and `dist/cli/setup.js` are byte-identical
through all three hops. Extracted `args.js` at `0.86.0` is byte-identical
to `0.85.1`; `0.86.1` only adds unmapped `META_API_KEY` help. `rpc-mode.js`
changes only at `0.86.0`: already-mapped `steer` / `follow_up` pass
`{ source: "rpc" }`; `0.86.0` and `0.86.1` are byte-identical.
`agent-session.js` at `0.86.0` routes steer/follow-up through
`_queueUserInput` → `_runInputHandlers`, which no-ops when
`!hasHandlers("input")`; `0.86.0` and `0.86.1` are byte-identical.

`0.86.1` `bundle/cli.js` is a 160-byte compile-cache stub that loads
`cli-runtime.js` (660 bytes). The interpreted selected entry remains
`dist/cli.js` (`setupCli` + `main`), byte-identical from `0.85.1` through
`0.86.1`.

## Selected protocol

Selected mapped commands remain `prompt`, `steer`, `follow_up`, `abort`,
`get_state`, `get_available_models`, `set_auto_compaction`,
`set_auto_retry`, `set_steering_mode`, and `set_follow_up_mode`. Direct
`bash`, `switch_session`, `fork`, `clone`, extensions, `clear_queue`, and
`compact` stay unmapped. RPC command count stays 33.

Selected argv flags remain in extracted `0.86.1` `dist/cli/args.js`:
`--mode rpc`, `--no-session`, `--offline`, `--provider`, `--model`,
`--tools`, `--no-extensions`, `--no-skills`, `--no-prompt-templates`,
`--no-themes`, `--no-context-files`, `--no-tools`. Standing-unused
help-unselected `--use-theme`, `defaultTools`, `--`, and `powershell` are
byte-identical carry-forwards from `0.84.4`.

`rpc-mode.js` at `0.86.0` adds `{ source: "rpc" }` on already-mapped
`steer` / `follow_up` so unmapped extension `input` handlers can see those
commands. `_runInputHandlers` returns `{ text, images }` unchanged when no
extension input handlers exist. Swallowtail selected argv includes
`--no-extensions`. Wire commands, payloads, and `rpc-types.js` stay
unchanged. `docs/rpc.md` is byte-identical from `0.85.1` through `0.86.1`.

Changelog extras stay unmapped: prompt cache warming, `/bug` reporting,
Radius catalogue, per-model compaction budgets, custom-provider
`TranscriptContext`, `user_bash` fail-closed, Meta Muse / `META_API_KEY`,
Node compile cache, and inherited provider-stream, catalogue, retry, and
clipboard fixes.

## Decision

Compatible extension of the mapped strict-LF subset on the existing
`0.84.0` message-update-delta revision. No new milestone.

- Keep baseline `0.80.10` and claim id `pi.rpc.package-window-2`.
- Keep unpublished gaps `0.80.11`, `0.81.2`, `0.82.2`, `0.83.1`,
  `0.84.5`, and `0.85.2` incompatible.
- Keep exact `0.83.0` Deprecated on
  `pi.rpc.strict-lf-v0.83.0-bash-extension-hook`.
- Keep Maintained `0.84.0..=0.84.4` and `0.85.0..=0.85.1`; add Maintained
  `0.86.0..=0.86.1` on `pi.rpc.strict-lf-v0.84.0-message-update-delta`.
  Do not extend `0.85.0..=0.85.1` across unpublished `0.85.2`.
- Raise `PI_PACKAGE_LATEST_QUALIFIED_VERSION` to `0.86.1`.
- Synthetic later-stable UnverifiedNewer is unpublished `0.86.2`.
- Decoder specimen remains `pi-rpc-0.80.10`. Frozen
  `pi-rpc-0.80.10-0.83.0` compatibility rows stay unchanged.
- Do not flatten onto Oh My Pi. Do not raise the SDK sidecar pin.

The claim edit is serial after this identity freeze. This record does not
edit production claims.

## Sources

- npm `@earendil-works/pi-coding-agent@0.85.1`, `@0.86.0`, `@0.86.1`
- [GitHub `v0.86.1`](https://github.com/earendil-works/pi/releases/tag/v0.86.1)
- [GitHub `v0.86.0`](https://github.com/earendil-works/pi/releases/tag/v0.86.0)
- git tags `v0.85.1`, `v0.86.0`, and `v0.86.1`; npm gitHeads `d981de1`,
  `ecac0a9`, `13cbf77`
- frozen `crates/swallowtail-adapter-pi/tests/fixtures/pi-rpc-0.85.1/`
- frozen `crates/swallowtail-adapter-pi/tests/fixtures/pi-rpc-0.86.1/`
