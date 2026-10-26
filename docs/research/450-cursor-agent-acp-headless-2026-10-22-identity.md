# Research 450: Cursor Agent ACP and Headless 2026.10.22 Identity

## Decision

Qualify `cursor-agent.acp` and `cursor-agent.headless` through official stable
`2026.10.22-27dcf5f`. Keep both baselines at `2026.07.01-41b2de7`, claim ids
`cursor-agent.acp.release-window-3` and
`cursor-agent.headless.release-window-3`, behavior revisions
`cursor-agent.acp-v1.interactive-v1` and
`cursor-agent.stream-json.structured-v1`, prior exact points, gaps, and
`AllowUnverified`. Add the exact points `2026.10.14-941492e`,
`2026.10.16-e3cb207`, `2026.10.18-ea085cb`, `2026.10.19-6bbf2c4`, and
`2026.10.22-27dcf5f`.

This is a compatible extension of the existing ACP and headless wires. It
adds no public operation, capability, authority, or behavior revision. The
routes stay separate. `cursor-agent.catalogue` stays at
`2026.10.01-14929f9`.

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
| `2026.10.14-941492e` | `4d1729af836732b46840080920912dac7d5a2935` | 2026-10-16 | qualified `2026-10-14` |
| `2026.10.16-e3cb207` | `d043ba62f22f4036d7cdc95e5870b3ac70cc01ed` | 2026-10-17 | qualified |
| `2026.10.18-ea085cb` | `6e20ddf94b078f0385b56c90aba40107edad2748` | 2026-10-19 | qualified |
| `2026.10.19-6bbf2c4` | `a83f6b7f0c60c3b87f45dff3b12603b4cae19dd0` | 2026-10-20 | qualified |
| `2026.10.22-27dcf5f` | `61a3cb3f49a35c40ce8ac4222192a7b7c8be5998` | 2026-10-23 | official stable |

Commit `8d7d203` still names `2026.10.01-e373342` in its message body. The
merged `cursor/agent.json` content is `2026.10.14-0f9edde`. File content is
the hop. `e373342` did not land as a registry file.

The release-date claim accepts one opaque build per calendar date. `2026-10-14`
binds `941492e` and rejects `0f9edde` even though that earlier archive's
selected ACP and headless surfaces match. Dates between published points stay
gaps, including `2026-10-02`, `2026-10-13`, `2026-10-15`, `2026-10-17`,
`2026-10-20`, and `2026-10-21`. `2026-10-23` is the synthetic unverified-newer
probe. Catalogue still treats every date after `2026-10-01` as unverified
newer, not as an ACP/headless gap.

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
argv/dispatch/wire/result/permission/cancel/auth/replay literals were hashed;
`protocol.json` freezes those strings and SHA-256 values. Session cleanup is
host-owned process, task, and working-resource join-or-kill; no vendor cleanup
literal was extracted or hashed.

The selected initialize core is byte-identical on both platforms at every hop:
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
field strings in `protocol.json` are the wire comparison. `--force`, `--yolo`,
`--sandbox`, `--auto-review`, and `--stream-partial-output` stay unselected.
At `2026.10.22`, both platforms add an internal git-worktree `--force`
refusal. Swallowtail does not send `--force`. `task_notification` system
events already exist at the `2026.10.01` ceiling and stay outside the selected
init event. The launcher stays byte-identical. Bundled Node changes only at
`2026.10.22` and is not a selected wire file.

## Claim limits

No prompt, credential, login, catalogue call, or live session was used. The
claim does not transfer to the catalogue, npm `cursor-agent`, other platforms,
or package internals. No release or tag authority follows from this research.
