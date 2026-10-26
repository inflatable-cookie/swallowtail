# Research 445: Claude Code headless and response-only 2.1.310 identity

Status: both claims qualified through official stable `2.1.310` as compatible
extensions. The claims stay separate.

Headless axis: `claude-code.headless-stream-json`
Headless route: `claude-code.headless`
Headless claim: `claude-code.headless.window-1`
Previous headless ceiling: `2.1.294` on maintained `claude-code.headless.stream-json.v2`

Response-only axis: `claude-code.response-only-stream-json`
Response-only route: `claude-code.response-only`
Response-only claim: `claude-code.response-only.window-1`
Previous response-only ceiling: `2.1.293` on maintained `claude-code.response-only.stream-json.v3`

Question, headless: can `claude-code.headless` extend `2.1.287..=2.1.294` on
`stream-json.v2` through current official stable `2.1.310`?

Question, response-only: can `claude-code.response-only` extend
`2.1.282..=2.1.293` on `stream-json.v3` through the same stable?

## Observation

Re-probed 2026-10-26T10:32:43Z, then again immediately before the claim edit.
npm `latest` and GitHub's latest non-prerelease agree on `2.1.310`. npm
`stable` is the delayed channel at `2.1.308`. npm `next` is also `2.1.310`.
Preview, nightly, alpha, and rc tags were ignored.

| Channel | Point |
| --- | --- |
| npm `latest` | `@anthropic-ai/claude-code` `2.1.310`, published `2026-10-23T21:20:46.540Z`; wrapper tarball SHA-256 `d696314076114df499ba88e117000a24a9d7ab4f502dcaefba52c83fd22acfbd` |
| GitHub latest stable | `v2.1.310`, published `2026-10-23T23:39:00Z`, commit `b70dc4113ad72e0ee44f7b1b95afaf510b53ee98` |
| Linux x64 `2.1.310` `claude` | SHA-256 `b73676430af12f97316a798b8371f81d640f90a58bdcb8f3ce56a835bb47df56` |
| Darwin arm64 `2.1.310` `claude` | SHA-256 `ffb64331bc0ee1b69efb7c2c428bd3ab516bc31b75bf60886b356c97c1c25ffd` |
| Unpublished inside the window | `2.1.297` and `2.1.298`; npm 404 and GitHub tags absent |
| First unpublished after latest | `2.1.311`; npm 404 and GitHub tag absent |
| Host `claude` | absent |

Published hops after the older ceiling: headless `2.1.295`, `2.1.296`,
`2.1.299` through `2.1.310`. Response-only adds the already-published
`2.1.294` hop in front of that list. `2.1.293` and `2.1.294` tarball
integrity still matches the Research 422 and 423 inventories, so those trees
were not re-extracted. Every later wrapper, Darwin arm64, and Linux x64
tarball was downloaded, checked against current npm SHA-512 integrity, and
hashed per file. No downloaded executable was run. No prompt, login, or live
session was sent. Public Git tags identify commits and do not prove
source-to-npm-runtime provenance.

## Surfaces checked on every hop

Research 374 stopped headless at `2.1.287` stream-json turn streaming and
`2.1.290` plan-mode permission after the PreToolUse rewrite. Research 422
stopped headless at `2.1.294` because instruction-form prompt and agent hooks
can block and Stop/SubagentStop prompt hooks are judged differently while
user, project, and local settings stay enabled. Research 423 then qualified
headless through `2.1.294` on private v2: `ok: false` blocks, a PreToolUse
deny blocks before dispatch, a rewritten input is rechecked, and
`stop_hook_active` true is reentry success under the provider block cap.

Those surfaces were reviewed again on each published hop from the previous
ceiling to `2.1.310`. Selected stream-json help lines stay byte-identical.
Darwin arm64 and Linux x64 binaries through `2.1.310` still carry the v2
markers (`ok: false` blocking, `permissionDecision`, `stop_hook_active`,
`CLAUDE_CODE_STOP_HOOK_BLOCK_CAP`, `parent_tool_use_id`) and the response-only
markers (`permissionMode` default, empty tools, `--safe-mode`,
`--disable-slash-commands`, `parent_tool_use_id`). The hop ledger is
`headless-protocol.json` and `response-only-protocol.json` in the fixture.
Every hop has `selected_mapped_change` false.

Adjacent package deltas add and remove no files. Platform packages change
`claude` and `package.json` on every hop. The wrapper changes `package.json`
on every hop and `sdk-tools.d.ts` only on `2.1.294→2.1.295`,
`2.1.295→2.1.296`, `2.1.296→2.1.299`, and `2.1.301→2.1.302`. Launcher files
`bin/claude.exe`, `cli-wrapper.cjs`, `install.cjs`, `LICENSE.md`, and
`README.md` stay byte-identical from `2.1.294` through `2.1.310`.

Release notes that touch hooks are tightenings or unselected paths:
`2.1.299` refuses a plugin hook that cannot be handed an action or cannot
return; `2.1.300` drops unknown `updatedInput` keys and denies rejected
input. Both keep the v2 deny and recheck rules. `--await-initialize`,
`--forward-subagent-text`, `onFailure`, managed-settings-only policy,
Windows shell fail-open, SDK callback hooks, MCP, Bash, mods, cloud, and
VS Code stay off the selected commands. Fork-subagent defaults in `-p` stay
bounded by the tool sets: headless does not admit `Agent`, and response-only
rejects a non-null `parent_tool_use_id`.

## Headless decision

Compatible extension. Keep claim id `claude-code.headless.window-1`, baseline
`2.1.220`, `AllowUnverified`, deprecated `stream-json.v1` on
`2.1.220..=2.1.286`, and maintained `stream-json.v2` on `2.1.287..=2.1.310`.
Add unpublished gaps `2.1.297` and `2.1.298`. `2.1.311` is `UnverifiedNewer`.
The selected command stays `-p`, text input, stream-json, verbose, no session
persistence, model, `--permission-mode plan`, tools `Read,Glob,Grep`,
setting sources `user,project,local`, and empty strict MCP. Slash commands
and hooks stay enabled. Cancellation stays process `force_stop`.

The optional maximum-turn gate stays the Research 226 set
`2.1.220..=2.1.241` except unpublished `2.1.230`. This extension does not
admit `2.1.310`.

## Response-only decision

Compatible extension on its own axis. Keep claim id
`claude-code.response-only.window-1`, baseline `2.1.227`, `AllowUnverified`,
deprecated v1 on `2.1.227..=2.1.278`, deprecated v2 on `2.1.280..=2.1.281`,
and maintained v3 on `2.1.282..=2.1.310`. Add the same unpublished gaps.
`2.1.311` is `UnverifiedNewer` on v3.

`2.1.294` qualifies on existing v3. The headless stop at that point was
settings-enabled instruction-form and Stop hooks. Response-only passes
`--safe-mode`. The `2.1.310` binary still describes that flag as disabling
customizations, including `CLAUDE.md`, skills, installed plugins, hooks, MCP,
and custom commands. Research 378's nine `@builtin` plugins remain a disclosed
provider surface. `--disable-slash-commands` still excludes the `2.1.287`
fork-skill stream. Unknown event types still fail closed.

From `2.1.309`, an oversized nested `CLAUDE.md` loads only its start. That
size prefix stays inside v3. It does not add a containment guarantee or a new
behavior revision.

The selected command stays `-p`, text, stream-json, verbose, no session
persistence, model, `--tools ""`, `--safe-mode`, `--disable-slash-commands`,
`--no-chrome`, `--prompt-suggestions false`, and empty strict MCP. Init still
requires `permissionMode` `default`, empty tools, empty `mcp_servers`, and
the prepared version. One text turn, `num_turns` 1.

## Sources

- [Fixture `claude-code-2.1.310`](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-code-2.1.310/)
- [npm `@anthropic-ai/claude-code`](https://www.npmjs.com/package/@anthropic-ai/claude-code)
- [GitHub `v2.1.310`](https://github.com/anthropics/claude-code/releases/tag/v2.1.310)
- [Research 423](./423-claude-code-headless-2-1-294-stream-hook-safety-adaptation.md)
- [Research 422](./422-claude-code-2-1-294-headless-currentness-stop.md)
- [Research 378](./378-claude-code-response-only-2-1-293-identity-and-qualification.md)
- [Research 374](./374-claude-code-2-1-293-headless-currentness-stop.md)
- [Contract 029](../knowledge/contracts/029-interface-version-qualification-and-compatibility.md)
