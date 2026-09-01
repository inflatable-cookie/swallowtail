# 268 Pi RPC 0.84.4 Identity

Status: promoted
Owner: Tom
Date: 2026-09-01
Card: g05 batch 039

## Question

Is official npm `@earendil-works/pi-coding-agent` `latest` = `0.84.4` a
compatible extension of `pi.package` `0.84.0..=0.84.3` on
`pi.rpc.strict-lf-v0.84.0-message-update-delta`, a new private milestone,
or a stop?

## Remaining AllowUnverified rank

Named family only. This run does not rank other families.

| Rank | Family | Host | Qualified bound | Why this order |
| --- | --- | --- | --- | --- |
| 1 | Pi RPC | installed `0.83.0` | published points through `0.84.3`; `0.83.1` gap | operator-named family; Research 267 selected this family alone; official npm `latest` is `0.84.4` |

Do not flatten this family onto `oh-my-pi.package` (`@oh-my-pi/pi-coding-agent`
latest at observation: `18.1.0`). Do not raise `pi.sdk-sidecar.package`.
Gemini stays deferred.

## Method

Compared npm `@earendil-works/pi-coding-agent@0.84.4` to the frozen
`0.84.3` identity corpus, GitHub tags `v0.84.3` and `v0.84.4`, selected git
blobs `packages/coding-agent/docs/rpc.md`, `src/modes/rpc/rpc-types.ts`,
`src/modes/rpc/rpc-mode.ts`, `src/modes/rpc/jsonl.ts`,
`src/core/session-cwd.ts`, `src/modes/json-event.ts`, and
`src/cli/args.ts`, plus extracted official `dist` for those surfaces and
`CHANGELOG.md`.

No provider prompt. No live RPC session. Host `pi --version` `0.83.0` was
observed and not replaced. Official artifacts stayed in `/tmp` and were
not executed.

## Identity

| Surface | Version | Evidence |
| --- | --- | --- |
| Host CLI | `0.83.0` | SHA-256 `af302f231437eaf6f37691bce4b34234fcb626bcb5eb3910d4fc3f6519bf78ca`; size 681; matches the frozen `0.84.2` host digest |
| Official npm latest | `0.84.4` | published 2026-08-28T22:07:57.753Z; integrity `sha512-jmOlrqUmvhh/siNWFRXjYLJzhKFIHNsAQaysRwzQPQFnPAaV/vhqHsLH/MBsIISA1Rjj7WTUFR3nJrpXoLx39w==`; shasum `3a2f04bfc5e463b4cfa36b174a586d11a0bdf9ad`; tarball SHA-256 `5bce766d19c3ceba18f3fbaad91c449c9f9d73981f9e3400ecef932006f06968`; gitHead `b79e4cc834970cca69daebffab7df1da7d1e52c4` |
| GitHub tag | `v0.84.4` | commit `b79e4cc834970cca69daebffab7df1da7d1e52c4`; release published 2026-08-28T22:08:23Z |

Published stables after previous ceiling `0.84.3`: `0.84.4` only. npm has
no `0.84.5`. GitHub has no `v0.84.5`. Unpublished `0.83.1` remains absent.

npm gitHead matches the GitHub tag commit.

Extracted `dist/cli.js` is byte-identical to `0.84.2` and `0.84.3`
(`840d1e8e689ed9e4937bcb00b9a810e02a8567d9afb10a47097f11ca93ea1521`,
size 710). The published bin remains `dist/bundle/cli.js`; that digest
changed and is not a mapped RPC surface. The executing bundle tree is
`dist/bundle/index.js` (SHA-256
`d6744208473f5f0f0a199377165922340e105cb9a98693031c3fd5bbbee3d484`).
Relative to frozen `0.84.3`, that tree adds seven chunks and removes
seven. Twelve of those are provider-transport (`chunk-2KVJKXS2`,
`google-generative-ai-YMUPJBKR`, `google-vertex-MPWMV4OF`,
`https-proxy-agent-2VXB7436`, `mistral-conversations-Q3AWZJAZ`,
`openai-completions-ERMU2SS7` added;
`chunk-GPPBJGBU`, `dist-RDWOYWHR`, `google-generative-ai-XDKMGBCJ`,
`google-vertex-D5FGEO3Y`, `mistral-conversations-YK73UAOZ`,
`openai-completions-JD4WAC3R` removed). The remaining pair is a
main-application-chunk rehash: added `chunk-OMWWHBTG` (~3.7 MB) replaces
removed `chunk-E5KXRMZK`. Bundled server RPC dispatch on that pair
changes 32→33 with exactly one addition, `clear_queue`; every other
command is identical. Packaging stays unmapped: twelve provider-transport
chunks plus a main-application-chunk rehash whose only RPC dispatch delta
is additive `clear_queue`. `clear_queue` is already recorded as additive
and unmapped.

## Selected protocol

`jsonl.ts`, `session-cwd.ts`, `json-event.ts`, and `args.ts` stay
byte-identical to `0.84.3`. `rpc-types.ts` and `rpc-mode.ts` add unused
`clear_queue` only. Selected mapped commands remain `prompt`, `steer`,
`follow_up`, `abort`, `get_state`, `get_available_models`,
`set_auto_compaction`, `set_auto_retry`, `set_steering_mode`, and
`set_follow_up_mode`. Direct `bash`, `switch_session`, `fork`, `clone`,
extensions, and `clear_queue` stay unmapped.

Selected argv flags remain in extracted `0.84.4` `dist/cli/args.js`:
`--mode rpc`, `--no-session`, `--offline`, `--provider`, `--model`,
`--tools`, `--no-extensions`, `--no-skills`, `--no-prompt-templates`,
`--no-themes`, `--no-context-files`, `--no-tools`. Standing-unused
help-unselected `--use-theme`, `defaultTools`, `--`, and `powershell`
are byte-identical carry-forwards from `0.84.3`, not new `0.84.4`
deltas.

Changelog extras stay unmapped: RPC `clear_queue`, terminal capability
overrides, extension `ui_prompt_start` / `ui_prompt_end`, fullscreen
selection copy, DeepSeek V4 Flash Vision, a JSONL resume
trailing-newline fix that does not touch mapped `jsonl.ts`, large tool
results crossing the auto-compaction threshold /
`_compactBeforeNextAssistantResponse` (inert: Swallowtail sends
`set_auto_compaction false`, so `shouldCompact` is false despite mapped
`auto_compaction` events), and extension messages with
`triggerTurn:false` delaying `message_start` / `message_end` (inert:
both argv shapes use `--no-extensions`; non-assistant `message_end`
decodes as Progress).

Published `dist/modes/rpc/rpc-types.js` is byte-identical to `0.84.3`;
the TypeScript git blob is the load-bearing protocol identity for the
additive `clear_queue` type.

## Decision

Compatible extension of the mapped strict-LF subset on the existing
`0.84.0` message-update-delta revision. No new milestone.

- Keep baseline `0.80.10` and claim id `pi.rpc.package-window-2`.
- Keep unpublished gaps `0.80.11`, `0.81.2`, `0.82.2`, and `0.83.1`
  incompatible.
- Keep exact `0.83.0` Deprecated on
  `pi.rpc.strict-lf-v0.83.0-bash-extension-hook`.
- Extend Maintained `0.84.0..=0.84.3` to `0.84.0..=0.84.4` on
  `pi.rpc.strict-lf-v0.84.0-message-update-delta`.
- Raise `PI_PACKAGE_LATEST_QUALIFIED_VERSION` to `0.84.4`.
- Synthetic later-stable UnverifiedNewer is unpublished `0.84.5`.
- Decoder specimen remains `pi-rpc-0.80.10`. Frozen
  `pi-rpc-0.80.10-0.83.0` compatibility rows stay unchanged.
- Do not flatten onto Oh My Pi. Do not raise the SDK sidecar pin.

Card 040 owns the claim change. This record does not edit production
claims.

## Sources

- npm `@earendil-works/pi-coding-agent@0.84.4`
- [GitHub `v0.84.4`](https://github.com/earendil-works/pi/releases/tag/v0.84.4)
- git tags `v0.84.3` and `v0.84.4`; npm gitHead `b79e4cc`
- frozen `crates/swallowtail-adapter-pi/tests/fixtures/pi-rpc-0.84.3/`
