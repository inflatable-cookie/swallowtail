# Research 473: Claude Code headless and response-only 2.1.317 identity

Status: both claims qualified through official stable `2.1.317` as compatible
extensions. The claims stay separate from each other and from
`claude-agent.acp` and `claude-agent.sdk`.

Headless axis: `claude-code.headless-stream-json`
Headless route: `claude-code.headless`
Headless claim: `claude-code.headless.window-1`
Previous headless ceiling: `2.1.316` on maintained `claude-code.headless.stream-json.v2`

Response-only axis: `claude-code.response-only-stream-json`
Response-only route: `claude-code.response-only`
Response-only claim: `claude-code.response-only.window-1`
Previous response-only ceiling: `2.1.316` on maintained `claude-code.response-only.stream-json.v3`

Question, headless: can `claude-code.headless` extend `2.1.287..=2.1.316` on
`stream-json.v2` through current official stable `2.1.317`?

Question, response-only: can `claude-code.response-only` extend
`2.1.282..=2.1.316` on `stream-json.v3` through the same stable?

## Observation

Re-probed 2026-11-03, then again immediately before the claim edit. npm
`latest`, npm `next`, and GitHub's latest non-prerelease all agree on
`2.1.317`, so the point that Research 466 saw only on `next` is now the
official stable. npm `stable` is the delayed channel, which advanced from
`2.1.309` to `2.1.311`. Preview, nightly, alpha, and rc tags were ignored.

| Channel | Point |
| --- | --- |
| npm `latest` | `@anthropic-ai/claude-code` `2.1.317`, published `2026-11-02T23:57:53.069Z`; wrapper tarball SHA-256 `bf8543506740f8ba36bbec24a5e02937dd8b514237fc951db69d9de394965c74` |
| GitHub latest stable | `v2.1.317`, published `2026-11-03T01:25:46Z`, commit `964d69565952a176ac1975ed218362991db71433` |
| npm `stable` | `2.1.311`, delayed channel |
| npm `next` | `2.1.317` |
| npm `2.1.316` | published `2026-10-31T00:42:05.743Z`; wrapper tarball SHA-256 `0b23c8ff73a58c263e88aa740bd8647723141f70dad089df63061858e019c4c3`, matching Research 466 |
| GitHub `v2.1.316` | published `2026-10-31T01:01:35Z`, commit `80c6959d28c4c30ecd14fc1cdf172f19d9adad81` |
| Darwin arm64 `2.1.317` `claude` | SHA-256 `09f153b46b5df9e8b3454716d682a91901de020cbca630f61dec1a015b32b5e9` |
| Linux x64 `2.1.317` `claude` | SHA-256 `eb224419e344575faa4a1f39b290b649425a10cd5ee2ef1d003480cd9ae57eb0` |
| Re-downloaded `2.1.316` | wrapper and both `claude` digests match Research 466 and the frozen fixture |
| First unpublished after latest | `2.1.318`; npm 404 and GitHub tag absent |
| Host `claude` | `2.1.314` (Claude Code); its linked executable SHA-256 `21d6e7b9a6f07b4cfcb7df13760b3b72aa25e084e13a5354413382987ba832ed` equals the official Darwin arm64 `2.1.314` artifact |

The published hop set from the previous ceiling is exactly `2.1.317`; no
unpublished point sits between `2.1.316` and `2.1.317`. The wrapper, Darwin
arm64, and Linux x64 tarballs were downloaded, checked against current npm
SHA-512 integrity, extracted, and hashed per file. No downloaded executable
was run. No prompt, login, or live session was sent. Public Git tags identify
commits and do not prove source-to-npm-runtime provenance. The host
`--version` probe is observation only; it is not a claim input and performed
no provider work.

## Surfaces checked

The extraction tooling was calibrated before use: it reproduces every frozen
window, count, and function name in the `claude-code-2.1.316` fixture
byte-for-byte on both platforms before reading `2.1.317`.

`selected-control-flow.json` binds measured selected JS on `2.1.316` and
`2.1.317` to the Darwin arm64 and Linux x64 executable hashes. The stop
records own different surfaces. Research 374 stopped on the `2.1.287`
`context: fork` stream change and the `2.1.290` permission recheck after a
`PreToolUse` rewrite. Research 422 and its adaptation record 423 cover the
`2.1.294` settings-hook surfaces. Research 445 and 459 extend the same
records. This record re-checks all of them.

The settings-hook surfaces stay: prompt `ok:false` blocking,
PreToolUse deny-before-dispatch, the rewrite collection and ask-to-deny
recheck, Stop and SubagentStop `stop_hook_active` reentry, and the Stop cap
default 8. The response-only surfaces stay: the `--safe-mode` option text,
the builtin-only plugin filter, the strict empty MCP gate and policy, init
`tools` / `mcp_servers` / `permissionMode` fields, the permission-mode default
mapper, the interactivity getter/setter and `-p` print detector, and the
builtin availability dispatch. Every differing window keeps its decision
tokens; the differences are minified identifier remangling (getter `Kd` to
`Zd`, setter `UYe` to `j7e`/`LJe`, print detector `opt` to `Amt`/`Emt`),
per-build chunk file names, and the version banner. Three surfaces are
byte-identical across the hop on both platforms: the
`required:["ok"],additionalProperties:!1` prompt schema, the `--tools`
option window, and the `Disable all skills and commands` help-table window.
Selected stream-json help lines stay byte-identical. Every marker is present
on every scanned binary, and the counted event families are unchanged:
`hook_started` 11, `hook_response` 38, `subtype:"init"` 1, `type:"result"`
12, `isInteractivityDecided` 4 on both compared points. The `is_error:!0`
literal drifts from 68 to 69 occurrences on Darwin arm64; the selected
PreToolUse site is unchanged and the literal is generic minified JS, so the
drift is bounded as unselected. The assistant-frame description is
byte-identical across the hop and across platforms (SHA-256
`d242c378c742de584ebc4d4b48452f57dd4944cc61c06fb4f3e0d4848ba5185d`), the
nullable `parent_tool_use_id` schema field keeps its four occurrences, and
the `context:"fork"` command definitions keep their two occurrences with the
`claude-test:author` draft agent.

Package trees add and remove no files. Both platform packages change `claude`
and `package.json`, and the wrapper changes `package.json` and
`sdk-tools.d.ts`. Launcher files `bin/claude.exe`, `cli-wrapper.cjs`,
`install.cjs`, `LICENSE.md`, and `README.md` stay byte-identical. The
`sdk-tools.d.ts` delta removes the `file_unchanged` shared-file variant with
its seeded marker and adds artifact-listing pagination (`page_token`,
`next_page_token`), a `timedOut` search-result flag, and an optional task
description — all Claude Agent SDK declarations, and neither CLI route
imports them.

The exact CLI option inventory adds nothing and removes nothing across the
hop (182 constructor-carried long options on both platforms, both points).
The `--safe-mode`, `--tools`, and `--disable-slash-commands` option texts are
byte-identical. The executables grow by about 2.6 MiB (Darwin arm64) and
3.0 MiB (Linux x64); whole-binary identity is not claimed.

`2.1.317` adds `cc-plugin-privacy` to the unconditional builtin initializer,
next to `sec-default`, `agents-md`, and `telemetry`, taking the builtin id
set from 13 to 14. The plugin is flag-gated (`tengu_wise_newt`, default on),
`enabledFromTrustedSettingsOnly`, and attaches `tool.check` hooks to
WebBrowser, WebFetch, WebSearch, Bash, Monitor, PowerShell, and Tmux that ask
when the call input carries the account email address. Headless selects
`Read,Glob,Grep` and response-only runs with empty tools, so no covered tool
call can occur on either route. The plugin contributes no MCP server, and
hook events reach the stream only through `--include-hook-events`, which
neither route passes. It joins the safe-mode builtin disclosure class, not a
behavior revision. `cc-plugin-claude-platform` still requires
`isInteractive()`, which the print detector makes false for `-p`; the builtin
availability filter still skips it before adding its MCP server, and
response-only `--safe-mode` still blocks auto-discovered MCP. The plugin
stays `gated-builtin-unselected`. Easel keeps its extra `remote_cowork`
registration outside both routes.

Release notes are discovery. The `2.1.317` note fixing `claude -p` exiting 0
when a full disk or quota lost its transcript or `~/.claude.json` writes is a
failure-reporting fix on the selected route: the adapter already requires a
terminal result frame and fails closed without one. The
`CLAUDE_CODE_DISABLE_NESTED_SESSION_ID` addition and the nested-session
identity fixes change which session a `claude -p` child attaches to when run
inside another Claude session; both routes spawn a clean child with
`--no-session-persistence` and never resume. The transcript-cleanup fix for
`--setting-sources` runs is provider disk hygiene. The Read-tool
file-unchanged stub removal and the Grep/Glob time-limit completeness fix are
tool-payload text inside `tool_result` frames, not mapped stream fields, and
the SDK declaration delta mirrors them. The `onFailure: "block"` SDK callback
value is absent as a literal from both executables. The email-exfiltration
ask lands in `cc-plugin-privacy`, classified above. Auto mode, effort level,
auth status, `--from-pr`, permission-prompt UI, connectors, OTel exports,
gateway and admin APIs, cloud sessions, self-hosted runners, Remote Control,
VS Code, and Claude Tag rows are outside both routes. The hop ledger is
`headless-protocol.json` and `response-only-protocol.json`.

## Headless decision

Compatible extension. Keep claim id `claude-code.headless.window-1`, baseline
`2.1.220`, `AllowUnverified`, deprecated `stream-json.v1` on
`2.1.220..=2.1.286`, and maintained `stream-json.v2` on `2.1.287..=2.1.317`.
Unpublished gaps stay `2.1.244`, `2.1.249`, `2.1.253` through `2.1.256`,
`2.1.262`, `2.1.264`, `2.1.279`, `2.1.297`, `2.1.298`, and `2.1.313`.
`2.1.317` qualifies on v2. `2.1.318` is `UnverifiedNewer`.

The selected command stays `-p`, text input, stream-json, verbose, no session
persistence, model, `--permission-mode plan`, tools `Read,Glob,Grep`, setting
sources `user,project,local`, and empty strict MCP. Slash commands and hooks
stay enabled. Cancellation stays process `force_stop`.

The optional maximum-turn gate stays the Research 226 set
`2.1.220..=2.1.241` except unpublished `2.1.230`. This extension does not
admit `2.1.317`.

## Response-only decision

Compatible extension on its own axis. Keep claim id
`claude-code.response-only.window-1`, baseline `2.1.227`, `AllowUnverified`,
deprecated v1 on `2.1.227..=2.1.278`, deprecated v2 on `2.1.280..=2.1.281`,
and maintained v3 on `2.1.282..=2.1.317`. The same unpublished gaps stay on
the deny-list. `2.1.317` qualifies on v3. `2.1.318` is `UnverifiedNewer` on
v3.

The selected command stays `-p`, text, stream-json, verbose, no session
persistence, model, `--tools ""`, `--safe-mode`, `--disable-slash-commands`,
`--no-chrome`, `--prompt-suggestions false`, and empty strict MCP. Init still
requires `permissionMode` `default`, empty tools, empty `mcp_servers`, and
the prepared version. One text turn, `num_turns` 1. Unknown event types still
fail closed.

## Sources

- [Fixture `claude-code-2.1.317`](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.317/)
- [npm `@anthropic-ai/claude-code`](https://www.npmjs.com/package/@anthropic-ai/claude-code)
- [GitHub `v2.1.317`](https://github.com/anthropics/claude-code/releases/tag/v2.1.317)
- [GitHub `v2.1.316`](https://github.com/anthropics/claude-code/releases/tag/v2.1.316)
- [Research 374](./374-claude-code-2-1-293-headless-currentness-stop.md)
- [Research 422](./422-claude-code-2-1-294-headless-currentness-stop.md)
- [Research 423](./423-claude-code-headless-2-1-294-stream-hook-safety-adaptation.md)
- [Research 445](./445-claude-code-headless-and-response-only-2-1-310-identity.md)
- [Research 459](./459-claude-code-headless-and-response-only-2-1-312-identity.md)
- [Research 466](./466-claude-code-headless-and-response-only-2-1-316-identity.md)
- [Contract 029](../knowledge/contracts/029-interface-version-qualification-and-compatibility.md)
