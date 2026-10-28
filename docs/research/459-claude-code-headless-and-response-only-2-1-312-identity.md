# Research 459: Claude Code headless and response-only 2.1.312 identity

Status: both claims qualified through official stable `2.1.312` as compatible
extensions. The claims stay separate from each other and from
`claude-agent.acp` and `claude-agent.sdk`.

Headless axis: `claude-code.headless-stream-json`
Headless route: `claude-code.headless`
Headless claim: `claude-code.headless.window-1`
Previous headless ceiling: `2.1.310` on maintained `claude-code.headless.stream-json.v2`

Response-only axis: `claude-code.response-only-stream-json`
Response-only route: `claude-code.response-only`
Response-only claim: `claude-code.response-only.window-1`
Previous response-only ceiling: `2.1.310` on maintained `claude-code.response-only.stream-json.v3`

Question, headless: can `claude-code.headless` extend `2.1.287..=2.1.310` on
`stream-json.v2` through current official stable `2.1.312`?

Question, response-only: can `claude-code.response-only` extend
`2.1.282..=2.1.310` on `stream-json.v3` through the same stable?

## Observation

Re-probed 2026-10-28T09:37:07Z, then again immediately before the claim edit.
npm `latest` and GitHub's latest non-prerelease agree on `2.1.312`. npm
`stable` is the delayed channel at `2.1.308`. npm `next` is also `2.1.312`.
Preview, nightly, alpha, and rc tags were ignored.

| Channel | Point |
| --- | --- |
| npm `latest` | `@anthropic-ai/claude-code` `2.1.312`, published `2026-10-27T18:36:10.160Z`; wrapper tarball SHA-256 `7bbcaf79930440d62daa309ae4fea63ba3df998dd6736b8d1c9a5767bf0bc7c1` |
| npm `2.1.311` | published `2026-10-26T19:47:57.395Z`; wrapper tarball SHA-256 `d97fea08695f32b13b3dec072687f2acc550484caecaf5005d4ebad86c5bf199` |
| GitHub latest stable | `v2.1.312`, published `2026-10-27T20:24:20Z`, commit `0e1f4789406477380b267a01f3ddd50b0211e5b3` |
| GitHub `v2.1.311` | published `2026-10-26T21:51:23Z`, commit `08c6a4f5121ae140d3c1b5c96e3e5ae53587b54b` |
| Linux x64 `2.1.312` `claude` | SHA-256 `4f4178ce41d83e23341ebfbaecdbcf8ea6bf7baea0690025abe49eeec3a5d01d` |
| Darwin arm64 `2.1.312` `claude` | SHA-256 `6b5ec691195e551b02c41df9f7d723f4ea61eaa03c2df90acefe3736f1aeb620` |
| Linux x64 `2.1.311` `claude` | SHA-256 `86dc1fcc69498159eaf15f446613e847010f9036bfbbdffd4c49350be8489ada` |
| Darwin arm64 `2.1.311` `claude` | SHA-256 `85d8c08707fcfd1e78ffa0204eefbc6ec9e4fe22f375fb2767ef85d957ed3161` |
| Re-downloaded `2.1.310` | wrapper and both `claude` digests match Research 445 |
| First unpublished after latest | `2.1.313`; npm 404 and GitHub tag absent |
| Host `claude` | absent |

Published hops after the previous ceiling: `2.1.311` and `2.1.312`, for both
claims. The wrapper, Darwin arm64, and Linux x64 tarballs were downloaded,
checked against current npm SHA-512 integrity, and hashed per file. No
downloaded executable was run. No prompt, login, or live session was sent.
Public Git tags identify commits and do not prove source-to-npm-runtime
provenance.

## Surfaces checked

`selected-control-flow.json` binds measured selected JS on `2.1.310`,
`2.1.311`, and `2.1.312` to the Darwin arm64 and Linux x64 executable hashes.
Prompt `ok:false` blocking, PreToolUse deny-before-dispatch, rewrite
`hookRewrites.push`, the rewrite recheck that turns `ask` into `deny` on
`hookUpdatedInput`, Stop and SubagentStop `stop_hook_active` reentry, and the
Stop cap default 8 stay. Identifier remangling changes the snippets. The
decision tokens do not. Selected stream-json help lines stay byte-identical.
Every hop has `selected_mapped_change` false on each route.

Package trees add and remove no files. Platform packages change `claude` and
`package.json` on both hops. The wrapper changes `package.json` and
`sdk-tools.d.ts`. Launcher files `bin/claude.exe`, `cli-wrapper.cjs`,
`install.cjs`, `LICENSE.md`, and `README.md` stay byte-identical from
`2.1.310` through `2.1.312`. The `.d.ts` additions (`toolsUsed`,
`local_file`, and an ask-user `unanswered` field) are Claude Agent SDK
declarations. Neither CLI route imports them.

`response-only-isolation.json` freezes the `--safe-mode` option, builtin-only
plugin filter, init `tools` / `mcp_servers` / `permissionMode` fields, and
the permissionMode default mapper. The option text is byte-identical across
the six binaries. `--disable-slash-commands` help stays `Disable all skills
and commands`.

From `2.1.311` the builtin registrar adds `cc-plugin-claude-platform` on the
existing `e` path (`CLAUDE_CODE_ENTRYPOINT` is not `local-agent`, and the
cowork predicate is false). `isAvailable` requires feature flag
`tengu_peppy_crescent` with default argument false, a `tst` predicate, no
`--agent-id`, and not `mcpAutoDiscovered`. The docs MCP URL
`https://platform.claude.com/docs/mcp` is behind `mayConnectMcpServer`, which
requires that availability. The skill stays behind
`CLAUDE_CODE_DISABLE_CLAUDE_API_SKILL` and the `claude-api` skill gate.
Headless still passes strict empty MCP. Response-only still requires empty
`mcp_servers` and fail-closes otherwise. The plugin is
`gated-builtin-unselected`.

Easel's registration changes from `if(e)` to `if(e)` or a `remote_cowork`
predicate. These routes are not `remote_cowork`. The local `e` path is
unchanged. Empty tools leave easel unselected.

Release notes are discovery. The `2.1.311` compact-and-continue and
stream-json escape notes are absent from the `2.1.311` binary and present in
the `2.1.312` embedded changelog. The credit-error function and the selected
JS stay structurally identical. Both routes decode stream-json with
`serde_json`, so a `\uXXXX` escape does not change a parsed value.
Response-only still fail-closes an unknown event type. `--hold-idle-file`,
`--max-hold-min`, the prompt-hook scheduler cap, and the invalid command-name
output fix stay outside a new mapped operation. The hop ledger is
`headless-protocol.json` and `response-only-protocol.json`.

## Headless decision

Compatible extension. Keep claim id `claude-code.headless.window-1`, baseline
`2.1.220`, `AllowUnverified`, deprecated `stream-json.v1` on
`2.1.220..=2.1.286`, and maintained `stream-json.v2` on `2.1.287..=2.1.312`.
Unpublished gaps stay `2.1.244`, `2.1.249`, `2.1.253` through `2.1.256`,
`2.1.262`, `2.1.264`, `2.1.279`, `2.1.297`, and `2.1.298`. `2.1.311` and
`2.1.312` qualify on v2. `2.1.313` is `UnverifiedNewer`.

The selected command stays `-p`, text input, stream-json, verbose, no session
persistence, model, `--permission-mode plan`, tools `Read,Glob,Grep`, setting
sources `user,project,local`, and empty strict MCP. Slash commands and hooks
stay enabled. Cancellation stays process `force_stop`.

The optional maximum-turn gate stays the Research 226 set
`2.1.220..=2.1.241` except unpublished `2.1.230`. This extension does not
admit `2.1.311` or `2.1.312`.

## Response-only decision

Compatible extension on its own axis. Keep claim id
`claude-code.response-only.window-1`, baseline `2.1.227`, `AllowUnverified`,
deprecated v1 on `2.1.227..=2.1.278`, deprecated v2 on `2.1.280..=2.1.281`,
and maintained v3 on `2.1.282..=2.1.312`. The same unpublished gaps stay.
`2.1.311` and `2.1.312` qualify on v3. `2.1.313` is `UnverifiedNewer` on v3.

The selected command stays `-p`, text, stream-json, verbose, no session
persistence, model, `--tools ""`, `--safe-mode`, `--disable-slash-commands`,
`--no-chrome`, `--prompt-suggestions false`, and empty strict MCP. Init still
requires `permissionMode` `default`, empty tools, empty `mcp_servers`, and
the prepared version. One text turn, `num_turns` 1. Unknown event types still
fail closed.

## Sources

- [Fixture `claude-code-2.1.312`](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.312/)
- [npm `@anthropic-ai/claude-code`](https://www.npmjs.com/package/@anthropic-ai/claude-code)
- [GitHub `v2.1.312`](https://github.com/anthropics/claude-code/releases/tag/v2.1.312)
- [GitHub `v2.1.311`](https://github.com/anthropics/claude-code/releases/tag/v2.1.311)
- [Research 445](./445-claude-code-headless-and-response-only-2-1-310-identity.md)
- [Contract 029](../knowledge/contracts/029-interface-version-qualification-and-compatibility.md)
