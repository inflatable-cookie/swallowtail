# Research 466: Claude Code headless and response-only 2.1.316 identity

Status: both claims qualified through official stable `2.1.316` as compatible
extensions. The claims stay separate from each other and from
`claude-agent.acp` and `claude-agent.sdk`.

Headless axis: `claude-code.headless-stream-json`
Headless route: `claude-code.headless`
Headless claim: `claude-code.headless.window-1`
Previous headless ceiling: `2.1.312` on maintained `claude-code.headless.stream-json.v2`

Response-only axis: `claude-code.response-only-stream-json`
Response-only route: `claude-code.response-only`
Response-only claim: `claude-code.response-only.window-1`
Previous response-only ceiling: `2.1.312` on maintained `claude-code.response-only.stream-json.v3`

Question, headless: can `claude-code.headless` extend `2.1.287..=2.1.312` on
`stream-json.v2` through current official stable `2.1.316`?

Question, response-only: can `claude-code.response-only` extend
`2.1.282..=2.1.312` on `stream-json.v3` through the same stable?

## Observation

Re-probed 2026-11-02T23:56:42Z, then again immediately before the claim edit.
npm `latest` and GitHub's latest non-prerelease agree on `2.1.316`. npm
`stable` is the delayed channel at `2.1.309`. npm `next` is also `2.1.316`.
Preview, nightly, alpha, and rc tags were ignored.

| Channel | Point |
| --- | --- |
| npm `latest` | `@anthropic-ai/claude-code` `2.1.316`, published `2026-10-31T00:42:05.743Z`; wrapper tarball SHA-256 `0b23c8ff73a58c263e88aa740bd8647723141f70dad089df63061858e019c4c3` |
| npm `2.1.317` (npm `next` only) | published `2026-11-02T23:57:53.069Z`; absent from npm `latest` and `stable` and from public tags; `UnverifiedNewer` |
| npm `2.1.315` | published `2026-10-30T21:05:24.080Z`; wrapper tarball SHA-256 `801d2fbfe74ba695fda1990ece740f4c38a7745833ac3f0ffa65ed9e31699e65` |
| npm `2.1.314` | published `2026-10-30T00:12:56.911Z`; wrapper tarball SHA-256 `0b9647fdfc4c4ced1254bc4c16207de7a9d3fa3ffe7a6c26e1f181f2789a3279` |
| npm `2.1.313` | absent; registry returned 404 and the point appears in no channel |
| GitHub latest stable | `v2.1.316`, published `2026-10-31T01:01:35Z`, commit `80c6959d28c4c30ecd14fc1cdf172f19d9adad81` |
| GitHub `v2.1.314` | published `2026-10-30T01:06:26Z`, commit `b3f291d131924810e2b7713ededc144843a229d5` |
| GitHub `v2.1.315` | no tag and no release |
| Darwin arm64 `2.1.316` `claude` | SHA-256 `64c327e92b2c5e0f78c0a980c5a518121e1f0d140ca771ca5dd4488d1a2dcb3f` |
| Linux x64 `2.1.316` `claude` | SHA-256 `fd13c70000fec3f2116dfbcbb6121210b4a29328bf0e46210a59dd594a67d824` |
| Darwin arm64 `2.1.315` `claude` | SHA-256 `ac4a9b309f0a0728cdd0fe1b0959cef3db77d4bcf6ce8bdc5d9f626006fcf58e` |
| Linux x64 `2.1.315` `claude` | SHA-256 `a3b9d66caf61b40d6aad99ed4de80f48f17e0e024ddf1b6e4289efade06105df` |
| Darwin arm64 `2.1.314` `claude` | SHA-256 `21d6e7b9a6f07b4cfcb7df13760b3b72aa25e084e13a5354413382987ba832ed` |
| Linux x64 `2.1.314` `claude` | SHA-256 `4487042b4c4f5d88d4568f68ae98aae0b1ffae58864cf57fb868641bc5f84594` |
| Re-downloaded `2.1.312` | wrapper `7bbcaf79930440d62daa309ae4fea63ba3df998dd6736b8d1c9a5767bf0bc7c1`; both `claude` digests match Research 459 |
| First unpublished after latest | `2.1.318`; npm 404 and GitHub tag absent |
| Host `claude` | `2.1.314` (Claude Code); its linked executable SHA-256 `21d6e7b9a6f07b4cfcb7df13760b3b72aa25e084e13a5354413382987ba832ed` equals the official Darwin arm64 `2.1.314` artifact |

npm `next` advanced to `2.1.317` after the first probe of this run, while npm
`latest` and GitHub's latest non-prerelease stayed on `2.1.316`. The official
stable did not move, so the hop set is unchanged. `2.1.317` is a published
stable above the ceiling without an official stable channel or public tag;
`AllowUnverified` already reports it as `UnverifiedNewer`. `2.1.318` is the
first unpublished point.

Published hops after the previous ceiling: `2.1.314`, `2.1.315`, and
`2.1.316`. `2.1.313` was never published and stays a gap. `2.1.315` was
published to npm without a public GitHub tag or release, so that hop has no
provider note and its identity rests on npm metadata, the integrity-checked
tarballs, and the shipped artifact tree. The wrapper, Darwin arm64, and Linux
x64 tarballs were downloaded, checked against current npm SHA-512 integrity,
extracted, and hashed per file. No downloaded executable was run. No prompt,
login, or live session was sent. Public Git tags identify commits and do not
prove source-to-npm-runtime provenance. The host `--version` probe is
observation only; it is not a claim input and performed no provider work.

## Surfaces checked

`selected-control-flow.json` binds measured selected JS on `2.1.312`,
`2.1.314`, `2.1.315`, and `2.1.316` to the Darwin arm64 and Linux x64
executable hashes. The stop records own different surfaces. Research 374
stopped on the `2.1.287` `context: fork` stream change and the `2.1.290`
permission recheck after a `PreToolUse` rewrite. Research 422 and its
adaptation record 423 cover the `2.1.294` settings-hook surfaces. The brief
names both records, and this record re-checks both.

The `2.1.290` rewrite surface is the `hookRewrites.push` row and the recheck
that turns `ask` into `deny` on `hookUpdatedInput`. The settings-hook
surfaces are prompt `ok:false` blocking, PreToolUse deny-before-dispatch,
Stop and SubagentStop `stop_hook_active` reentry, and the Stop cap default 8.
All of them stay. Identifier remangling changes the snippets. The decision
tokens do not. Selected stream-json help lines stay byte-identical. Every hop
has `selected_mapped_change` false on each route.

The `2.1.287` forked-stream surface has its own file,
`forked-stream-surface.json`. It re-extracts the provider's assistant-frame
schema description on every hop: the CLI emits one assistant message per
completed content block, consecutive messages may share `message.id`,
`message.stop_reason` is null and usage is not final on those frames, the
turn's stop reason and total usage arrive on the result message, and
`parent_tool_use_id` is non-null inside a subagent. That description is
byte-identical from `2.1.312` through `2.1.316` on both platforms. The
nullable `parent_tool_use_id` schema field stays, and the `context:"fork"`
command definitions stay. On the adapter side, a null `message.stop_reason` on
a parented assistant frame is projected as provider-unspecified activity, not a
final answer, and `claude_code_activity::tests::forked_assistant_frame_accepts_null_stop_reason_without_final_activity`
re-runs in `validate:current-claude-code-headless`. `--forward-subagent-text`
exists but neither route passes it, so it cannot create forked text on these
routes; forked frames come only from `context: fork` prompt commands, which
headless leaves reachable. The forked-stream surface is unchanged across all
three hops.

Package trees add and remove no files. Both platform packages change `claude`
and `package.json` on every hop. The wrapper changes `package.json` on every
hop and `sdk-tools.d.ts` only from `2.1.312` to `2.1.314`. Launcher files
`bin/claude.exe`, `cli-wrapper.cjs`, `install.cjs`, `LICENSE.md`, and
`README.md` stay byte-identical from `2.1.312` through `2.1.316`. The `.d.ts`
additions (`outsidersMayHaveWritten` and `backgroundWakesOwner`) are Claude
Agent SDK declarations. Neither CLI route imports them.

The exact CLI option inventory adds only `--agents-view-if-allowed` at
`2.1.314` and `--gateway-json` at `2.1.315`, removes nothing, and adds nothing
at `2.1.316`. The first is an interactive agents-view allowlist entry; the
second is a `claude auth login` presentation flag. Neither route passes either
flag and neither authenticates. The `--safe-mode`, `--tools`, and
`--disable-slash-commands` option texts are byte-identical across the four
points.

`response-only-isolation.json` freezes the `--safe-mode` option description,
builtin-only plugin filter, init `tools` / `mcp_servers` / `permissionMode`
fields, and the permissionMode default mapper. From `2.1.314` the launch path
also records `isInteractivityDecided` and `isNoDialogLaunch`; the setter wraps
the same stored value and the print detector is unchanged, so `-p` still makes
`isInteractive()` false. `cc-plugin-claude-platform` and easel keep their
builtin registrations and the same 13 `@builtin` ids. The plugin's
`isAvailable` still begins with the interactive getter and then requires the
`tengu_peppy_crescent` flag. The builtin availability filter skips the plugin
before adding its MCP server, `mayConnectMcpServer` rechecks `isAvailable`, and
response-only `--safe-mode` independently blocks `mcpAutoDiscovered`. The
plugin stays `gated-builtin-unselected`. Easel's extra `remote_cowork`
registration stays outside both routes. Headless passes strict empty MCP;
response-only requires empty `mcp_servers` and fail-closes otherwise.

`2.1.315` grows each platform executable by about 30 MiB and embeds a large
block of non-CLI tool-runner text that `2.1.314` does not carry. Whole-binary
identity is therefore not claimed. The selected CLI option texts, stream-json
help, selected markers, hook decision literals, init construction, and
safe-mode isolation stay present and structurally unchanged, and no
stream-json subtype or event type is added or removed.

Release notes are discovery. The `2.1.314` note fixing `claude -p` exiting 0
with no output on a startup failure is a failure-reporting fix on the selected
route: the adapter already requires a terminal result frame and fails closed
without one. The same note's `--setting-sources` exclusion fix makes an
already-mapped flag apply as documented, which is a compatible extension. The
`2.1.314` `permissions.blockWritesOutsideWorkingDirectories` setting, Bash and
Monitor hook fixes, and the `2.1.316` `CLAUDE_CODE_DISABLE_ULTRATHINK`,
`CLAUDE_CODE_SUBAGENT_EFFORT`, auto-mode, fast-mode, gateway, cloud, and VS
Code notes are outside the fixed read-only tool set, the strict empty MCP
config, and the local subscription command. The `2.1.316` Grep/Glob timeout and
shell self-exit fixes change provider process robustness, not a mapped field or
lifecycle step. The hop ledger is `headless-protocol.json` and
`response-only-protocol.json`.

## Headless decision

Compatible extension. Keep claim id `claude-code.headless.window-1`, baseline
`2.1.220`, `AllowUnverified`, deprecated `stream-json.v1` on
`2.1.220..=2.1.286`, and maintained `stream-json.v2` on `2.1.287..=2.1.316`.
Unpublished gaps stay `2.1.244`, `2.1.249`, `2.1.253` through `2.1.256`,
`2.1.262`, `2.1.264`, `2.1.279`, `2.1.297`, and `2.1.298`, and `2.1.313` joins
them. `2.1.314`, `2.1.315`, and `2.1.316` qualify on v2. `2.1.317` is
`UnverifiedNewer`.

The selected command stays `-p`, text input, stream-json, verbose, no session
persistence, model, `--permission-mode plan`, tools `Read,Glob,Grep`, setting
sources `user,project,local`, and empty strict MCP. Slash commands and hooks
stay enabled. Cancellation stays process `force_stop`.

The optional maximum-turn gate stays the Research 226 set
`2.1.220..=2.1.241` except unpublished `2.1.230`. This extension does not
admit `2.1.313`, `2.1.314`, `2.1.315`, or `2.1.316`.

## Response-only decision

Compatible extension on its own axis. Keep claim id
`claude-code.response-only.window-1`, baseline `2.1.227`, `AllowUnverified`,
deprecated v1 on `2.1.227..=2.1.278`, deprecated v2 on `2.1.280..=2.1.281`,
and maintained v3 on `2.1.282..=2.1.316`. The same unpublished gaps stay and
`2.1.313` joins the deny-list. `2.1.314`, `2.1.315`, and `2.1.316` qualify on
v3. `2.1.317` is `UnverifiedNewer` on v3.

The selected command stays `-p`, text, stream-json, verbose, no session
persistence, model, `--tools ""`, `--safe-mode`, `--disable-slash-commands`,
`--no-chrome`, `--prompt-suggestions false`, and empty strict MCP. Init still
requires `permissionMode` `default`, empty tools, empty `mcp_servers`, and the
prepared version. One text turn, `num_turns` 1. Unknown event types still fail
closed.

## Sources

- [Fixture `claude-code-2.1.316`](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.316/)
- [npm `@anthropic-ai/claude-code`](https://www.npmjs.com/package/@anthropic-ai/claude-code)
- [GitHub `v2.1.316`](https://github.com/anthropics/claude-code/releases/tag/v2.1.316)
- [GitHub `v2.1.314`](https://github.com/anthropics/claude-code/releases/tag/v2.1.314)
- [Research 374](./374-claude-code-2-1-293-headless-currentness-stop.md)
- [Research 422](./422-claude-code-2-1-294-headless-currentness-stop.md)
- [Research 423](./423-claude-code-headless-2-1-294-stream-hook-safety-adaptation.md)
- [Research 459](./459-claude-code-headless-and-response-only-2-1-312-identity.md)
- [Contract 029](../knowledge/contracts/029-interface-version-qualification-and-compatibility.md)
