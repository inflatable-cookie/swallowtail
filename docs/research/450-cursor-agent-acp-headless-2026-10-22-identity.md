# Research 450: Cursor Agent ACP and Headless 2026.10.22 Identity

## Decision

Stop the proposed `cursor-agent.acp` and `cursor-agent.headless` extension to
`2026.10.22-27dcf5f`. Retain their twelve qualified points through
`2026.10.01-14929f9`, both baselines at `2026.07.01-41b2de7`, claim ids
`cursor-agent.acp.release-window-3` and
`cursor-agent.headless.release-window-3`, behavior revisions
`cursor-agent.acp-v1.interactive-v1` and
`cursor-agent.stream-json.structured-v1`, prior exact points, gaps, and
`AllowUnverified`. The candidate points `2026.10.14-941492e`,
`2026.10.16-e3cb207`, `2026.10.18-ea085cb`, `2026.10.19-6bbf2c4`, and
`2026.10.22-27dcf5f` remain unverified newer on these two routes.

The identity inventory and declaration/literal comparisons below do not
establish a compatible extension. No public operation, capability, authority,
or behavior revision changes. `cursor-agent.catalogue` is independently qualified
through `2026.10.22-27dcf5f` by
[Research 451](451-cursor-agent-2026-10-22-catalogue-identity.md); this
ACP/headless evidence does not extend that claim.

Canonical main already qualified both routes through `2026.10.01-14929f9`
(Research 395 and 399). The dispatch brief named the older `2026-09-18`
ceiling; this record starts from the claim on main and does not re-walk those
already qualified hops.

## Official channel and hops

The selected channel is the Cursor entry in the
[official ACP registry](https://cdn.agentclientprotocol.com/registry/v1/latest/registry.json)
and the archive URLs that entry publishes. On 2026-10-26 the registry and
`https://cursor.com/install` both reported `2026.10.22-27dcf5f`. The installer
is recorded only as agreement; it is not a second axis. npm `cursor-agent`
stays a separate axis. No host `cursor-agent` was installed, updated, or
executed.

Published registry file hops after the `2026.10.01-14929f9` ceiling:

| Version | Registry commit | Commit date | Claim |
| --- | --- | --- | --- |
| `2026.10.01-14929f9` | `310074e4c18c5ff39ec178e84d0f461f8ff73fe0` | 2026-10-02 | previous ceiling |
| `2026.10.14-0f9edde` | `8d7d20369fcaa4ac943df97d1e5502b78681a003` | 2026-10-14 | rejected same-date build |
| `2026.10.14-941492e` | `4d1729af836732b46840080920912dac7d5a2935` | 2026-10-16 | candidate; unverified newer |
| `2026.10.16-e3cb207` | `d043ba62f22f4036d7cdc95e5870b3ac70cc01ed` | 2026-10-17 | candidate; unverified newer |
| `2026.10.18-ea085cb` | `6e20ddf94b078f0385b56c90aba40107edad2748` | 2026-10-19 | candidate; unverified newer |
| `2026.10.19-6bbf2c4` | `a83f6b7f0c60c3b87f45dff3b12603b4cae19dd0` | 2026-10-20 | candidate; unverified newer |
| `2026.10.22-27dcf5f` | `61a3cb3f49a35c40ce8ac4222192a7b7c8be5998` | 2026-10-23 | official stable; unverified newer |

Commit `8d7d203` still names `2026.10.01-e373342` in its message body. The
merged `cursor/agent.json` content is `2026.10.14-0f9edde`. File content is
the hop. `e373342` did not land as a registry file.

The release binding retains one opaque build per qualified catalogue date:
`2026-10-14` binds `941492e` and rejects `0f9edde`. ACP/headless retain their
older gaps through `2026-10-01`; `2026-10-02` is their first synthetic
unverified-newer point. Dates above that ceiling remain unverified newer,
including the candidate points, without gaining qualified status. Catalogue
retains its independent seventeen points and intervening gaps under Research
451. No route inherits another route's qualification.

## Artifact identity

Both official Darwin ARM64 and Linux x64 archives were downloaded and hashed.
None was executed. Archive URLs use
[`downloads.cursor.com/lab`](https://downloads.cursor.com/lab/).

| Version / platform | Archive SHA-256 | Bytes | Files |
| --- | --- | --- | --- |
| `2026.10.01-14929f9` Darwin ARM64 | `778d04e542adc5c8b6760fda3ebe0757f903b1764f2792c232ef9a35e6e2151b` | 176783497 | 449 |
| `2026.10.01-14929f9` Linux x64 | `ba9a855f8f813c91b9f2707127572d2dc9ae5a62818e1c36719625d0fb8bd452` | 182614608 | 455 |
| `2026.10.14-0f9edde` Darwin ARM64 | `efc39839ee3bc0b05467cd6e4baf23a43e358dae61d11512d9c5bf4ebbb3217b` | 178010821 | 452 |
| `2026.10.14-0f9edde` Linux x64 | `74fac931fe3578980d070701872b7dab54eb87a9abf9b6e2e6be19beeb9167a7` | 183859774 | 458 |
| `2026.10.14-941492e` Darwin ARM64 | `c6d9e2eaa1b1e24221517e0b4206b1f1593792e6f0d8a52e9b507bbe7ab0cc77` | 178006204 | 452 |
| `2026.10.14-941492e` Linux x64 | `6f6bf39dd2df29dcd38643cdfab0155ce71699953889f732deec123f6562bb2d` | 183861376 | 458 |
| `2026.10.16-e3cb207` Darwin ARM64 | `5f04d196a84ddb5c20a4a466bdd74e3695bfb447e3c4a1e6071364b2b9497dd0` | 177839943 | 454 |
| `2026.10.16-e3cb207` Linux x64 | `3aa3f48cd7a3909a98c9b3f2a4085f0c021088bf826690b22c61f424c843e552` | 183721050 | 460 |
| `2026.10.18-ea085cb` Darwin ARM64 | `94806e2d28ba57e4206319e2e56a93a60263744a176dafc533ce633ee4a7a0b0` | 177866441 | 451 |
| `2026.10.18-ea085cb` Linux x64 | `22325b799e8b7a8a94fc79d3596cbfa1b85407343d34c3a552f703d4023b1c79` | 183729801 | 459 |
| `2026.10.19-6bbf2c4` Darwin ARM64 | `095dccb923488d280aae697ba68c13a92bbd9ce1dc6105401e147e25869f2b13` | 177874691 | 452 |
| `2026.10.19-6bbf2c4` Linux x64 | `cf80cc18d8b33bf33e762353977ca1868cfc45290c92c0ffdcef83bb8b37b44f` | 183733394 | 460 |
| `2026.10.22-27dcf5f` Darwin ARM64 | `2f5b0a3dd5fb951cecffac813867d8a08d810e59837d59fd5f12d69b922398d7` | 180412627 | 437 |
| `2026.10.22-27dcf5f` Linux x64 | `85296944e4600a83c4b764f578d1502bf9c416db200d33a593e8f3ecabfdc294` | 184859176 | 445 |

Ceiling archive digests match Research 395 and 399. This inventory counts 449
Darwin files at `2026.10.01-14929f9` because it includes two AppleDouble
`._tree-sitter-bash.node` paths the earlier 447-file inventory omitted. Every
overlapping file hash matches. The `dist-package/cursor-agent` launcher
SHA-256 stays `2ccc9a8e167797641448b5e5c936f006ba137a2555f117f38c5eb76a5238a233`
on every hop. `package.json` stays the 64-byte stub through
`2026.10.14-941492e` and gains a dependency declaration at `2026.10.16`. The
bundled Node binary changes only at `2026.10.22`.

Linux file deltas from the ceiling: `0f9edde` changes 69 files; `941492e`
changes 17 and adds or removes none, with an identical ACP SDK chunk; later
hops change 47, 42, 19, then 98 files. Darwin deltas are larger because chunk
names move. Full path lists, selected-file digests, and path-set digests are
frozen in
[`cursor-agent-acp-headless-2026.10.22/identity.json`](../../crates/swallowtail-adapter-cursor/tests/fixtures/cursor-agent-acp-headless-2026.10.22/identity.json)
and
[`dist-inventory.json`](../../crates/swallowtail-adapter-cursor/tests/fixtures/cursor-agent-acp-headless-2026.10.22/dist-inventory.json).
The ledger is not a semantic changelog.

## Selected surfaces

ACP command, SDK, runtime index, and headless writer chunks were extracted
from the retained official Darwin ARM64 and Linux x64 archives at every hop
from `2026.10.01-14929f9` through `2026.10.22-27dcf5f`, including the five
Linux intermediate hops. Archives were not executed. Selected
initialize/cancel/options/help declaration literals were hashed;
`protocol.json` retains those strings and elided wire-field summaries. These
are partial observations, not complete selected handler traces. Session cleanup is
host-owned process, task, and working-resource join-or-kill; no vendor cleanup
literal was extracted or hashed.

The extracted initialize core is byte-identical on both platforms at every hop:
`loadSession:!0,mcpCapabilities:{http:!0,sse:!0},promptCapabilities:{audio:!1,embeddedContext:!1,image:!0}`
(SHA-256 `bcd62de052a72a733ec1ddb1d928a95ded1bfca71a3efd440f0d597e91195170`).
The initialize return is `return{protocolVersion:o.WP,agentCapabilities:a,authMethods:s}`
(`01885698b5cfa8e51978bd96fc399eb91ea14ea4c6b521e55d2c2b6bc05a331f`). SDK
export `WP` binds `const r=1` at the ceiling and `const i=1` on later hops.
Auth method `cursor_login`, `sessionCapabilities.list`, and client-negotiated
subagents default-off stay. `agentInfo` stays absent. `session/new` still
requires `cursor_login` before create; the login-helper minifier identifier
is `p.mf` at the ceiling and `g.mf` later. `async cancel` is
`t?.pendingPromptCancel?.()`
(`f5811db1ffd2791d8902b8dcfbd62b4a41252fd08b1bb7c7d05cec4016ff4979`).
`requestWebPermission` still advertises allow-once / allow-always / reject-once
(`9763792510b67f62d8d2183a7a9daf9ded6e4a5930f4d27d30621b9cbf49f077`);
allow-always is not a durable grant. `loadSession` stays advertised; replay
still logs `ACP loadSession history replay failed to load conversation`, so
continuation recovery stays blocked. The bundled SDK package stays `0.14.1`.
Zod packaging moves from `3.25.76` to `4.6.5` at `2026.10.16`; selected
`readTextFile:!1,writeTextFile:!1` defaults stay.

Headless selected argv, mode, and writer fields were compared on both
platforms at every hop. `--print`, `--output-format stream-json`, `--model`,
`--trust`, and `--mode` `.choices(["plan","ask"])` are byte-identical in
`index.js`. Writer system/init still emits `session_id`, `model`, and
`permissionMode: default`. Result success still emits `is_error: false`,
durations, `result`, `session_id`, `request_id`, and optional usage
`inputTokens` / `outputTokens` / `cacheReadTokens` / `cacheWriteTokens`.
Usage-spread minifier identifier is `oe` at the ceiling and `ae` later.
Writer-chunk hashes in `identity.json` remain artifact identity; the selected
field descriptions in `protocol.json` do not prove dispatch or failure handling. `--force`, `--yolo`,
`--sandbox`, `--auto-review`, and `--stream-partial-output` stay unselected.
At `2026.10.22`, both platforms add an internal git-worktree `--force`
refusal. Swallowtail does not send `--force`. `task_notification` system
events already exist at the `2026.10.01` ceiling and stay outside the selected
init event. The launcher stays byte-identical. Bundled Node executes the
selected runtime and changes at `2026.10.22`; its impact is unresolved.

## Finite stop and follow-up proof

The owner's 2026-10-26 correction requires preservation of the pre-extension
ACP/headless bounds when retained static evidence cannot close the selected
paths. This record adopts that stop. Catalogue remains independently qualified.
The stop is missing qualification evidence, not demonstrated incompatibility.
It is not a terminal unsupported disposition.

1. **Headless dispatch and effects.** Help text, `.choices`/`.option`
   declarations, field summaries, and fake-host tests do not prove selected
   effects. Required: trace parsed `--print`, `stream-json`, model, trust, and
   omitted/plan/ask mode through the actual shipped handler, mode selection,
   permission/tool execution, stream dispatch, and success/failure emission.
   Include tool/subagent/unknown-event/worktree guards. Record exact module
   digests, byte offsets, branch conditions, and measured minifier-normalized
   deltas at every changed hop on Darwin ARM64 and Linux x64.
2. **ACP callbacks, actions, and cancellation.** Initialize/cancel/options
   literals and SDK `0.14.1` do not establish the complete selected path.
   Required: trace `requestWebPermission` callers, client outcome validation,
   result-to-action consumption, pending requests, cancellation/abort,
   background work and cleanup. Compare selected Zod `3.25.76` to `4.6.5`
   schemas, defaults, unions, result/error handling, and callers at each hop
   on both platforms. Preserve observational callbacks and blocked replay;
   do not infer a new permission or lifecycle capability.
3. **Executing runtime and feeding code.**
   [`runtime-impact.json`](../../crates/swallowtail-adapter-cursor/tests/fixtures/cursor-agent-acp-headless-2026.10.22/runtime-impact.json)
   binds fourteen bundled Node observations to the existing hop digests.
   Embedded build metadata identifies Darwin `v26.8.1` to `v26.12.1` and
   Linux `v24.5.0` to `v24.22.1`; these are static metadata, not executed
   version observations or source parity. Node executes the selected code;
   event-loop/abort ordering, stdio flushing, signals/process cleanup,
   filesystem and native-addon impacts require assessment. The target Linux
   `4060.index.js` imports `../file-service/index.js` and loads
   `file_service.linux-x64-gnu.node` with `process.dlopen`. The retained package
   supplies the binding and changed native binary without implementation
   source correlation. The dependency observation is not a complete
   entrypoint-to-effect proof. Required: close that boundary and the selected
   call/import/guard paths through changed worker/plugin/cloud/SEA/native
   chunks; enumeration alone does not establish unreachability. Recover
   attributable static source or specify a separately authorized, bounded
   provider-free artifact proof where static evidence cannot settle it.

The `version-currentness` planner owns the follow-up adaptation/proof brief
to qualify the current official stable. Reuse these exact identities and
retained artifacts; do not repeat completed inventories. Qualification needs
independent ACP and headless evidence satisfying all three gates. No original
probe, vendor execution, live/auth work, or new product authority is authorized
by this stop. Prepared fakes continue to test adapter mapping at the twelve
qualified points, not vendor behavior at newer candidates.

## Claim limits

No prompt, credential, login, catalogue call, or live session was used. The
claim does not transfer to the catalogue, npm `cursor-agent`, other platforms,
or package internals. No release or tag authority follows from this research.
