# Research 472: Claude Agent SDK 0.3.317 Qualification

Date: 2026-11-03
Route: `claude-agent.sdk`
Axes: coupled SDK wrapper package and embedded native; Node, sidecar wire and
sidecar source remain independent.

## Decision

Compatible extension. Qualify every published package/native pair from the
retained `0.3.295`/`2.1.295` ceiling through official npm `latest`/`next`
`0.3.317`/`2.1.317`. Preserve the package and native baselines `0.3.284` and
`2.1.284`, claim IDs, `QualifiedOnly` posture, behavior revision
`claude-agent.sdk-v1`, exclusions, the `0.3.293`/`2.1.293` default pair, the
exact `0.3.259`/`2.1.259` registered-tool live point, and all prior pairs. The
unpublished `0.3.297`, `0.3.298`, and `0.3.313` points stay outside the finite
maintained pair map even though the window spans them arithmetically; nothing
is published above `0.3.317` at observation time. The Node `22.23.2..=22.23.3`
segment (Research 387), the exact wire, and the sidecar source tag are
unchanged. No Claude Code claim changes: the coupled native `2.1.317` pairing
rebinds only the `claude-agent.sdk.native` axis, and the separate
`claude-code.headless`/`claude-code.response-only` claims stay where PR 535
put them.

## Official identity and provenance

The selected channel is npm `dist-tags.latest`, observed at
`2026-11-03T08:45:56Z` against the
[official npm registry metadata](https://registry.npmjs.org/@anthropic-ai/claude-agent-sdk).
`latest` and `next` both resolved to `0.3.317`. The exact tarball URLs, publish
times, npm SHA-1 and SHA-512 integrity metadata, SHA-256 digests, package tree
digests, per-file hashes, native manifest identities, and all eight platform
binary hashes/sizes per hop are retained in the
[`0.3.317` fixture](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-0.3.317/README.md).
The fixture contains no vendor source or executable bytes.

| SDK | Published (UTC) | Tarball SHA-256 | Native | Manifest commit | Build time | Files | Tree digest |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `0.3.296` | 2026-10-09 16:59:17 | `e966be99b6f047d56165e2ec03822f17a49beebd48ac8efb91dab1e48bcf69a7` | `2.1.296` | `fdb6c17a97b6e0e3cf15e3566663fca58e7c43a9` | `2026-10-09T14:46:35Z` | 19 | `11dd626df9ad64d8…` |
| `0.3.299` | 2026-10-14 00:23:46 | `f0beb43b8a11cfe7eddc2441cdfa3fd9075a620f5157b9cf4de4dfd7fb20bb6b` | `2.1.299` | `c416f56ec7eca27e49747c35af51d5c92c856ba1` | `2026-10-13T22:05:00Z` | 19 | `f9aeed24db8c7e7a…` |
| `0.3.300` | 2026-10-14 14:28:12 | `2eebd96411c12dfa3b9294b632c02665113003ad85645d4cfbb9cf5a8a161baa` | `2.1.300` | `c6733ae1276880507ca0c5b053ae18fb0df090d6` | `2026-10-14T13:29:53Z` | 19 | `7ce2b67d4cfb29b1…` |
| `0.3.301` | 2026-10-15 14:35:13 | `1cf0f6a61294cb7edd3ef2682828139ba999f7dd263e46d75d4135ddd3a55b05` | `2.1.301` | `12a1baed117fbcc7e0695ab8aa6d2a4c6bd00cbb` | `2026-10-15T14:01:03Z` | 19 | `d88ef28b991db534…` |
| `0.3.302` | 2026-10-16 16:47:53 | `c7d7d61d6b0b7b7bb05cc053f229c5df88f151550ead0624c7393939d0d8e1f1` | `2.1.302` | `2ef144e6f98c1c99d082b735e4b38f50e3391e55` | `2026-10-16T16:13:51Z` | 19 | `a2ade0409300747a…` |
| `0.3.303` | 2026-10-18 21:15:49 | `86dc29477a33792a85088236c2d64a58881daca14fc98afbefa44c73799371fc` | `2.1.303` | `07df2aedd2d01d2f60907b8404acbda187b04072` | `2026-10-18T20:40:12Z` | 19 | `b478a04654f52e81…` |
| `0.3.304` | 2026-10-19 19:48:33 | `e2c92532d3fa5cd76b18266de39f3f7ee6914a2fe608b873f6f1160c081fb3ad` | `2.1.304` | `17ab70404e5b632fe060207effb0b440788363ba` | `2026-10-19T18:30:38Z` | 19 | `4439343ca7d3a6f2…` |
| `0.3.305` | 2026-10-20 19:29:33 | `cc6bc714e69298a1bd0fd2f962a5f093b989b1bb7e7895611a9ead93d93f25a4` | `2.1.305` | `6e723756d1226ddc2a4fefe5586e02bcc6a0501f` | `2026-10-20T18:24:45Z` | 21 | `73361164416c3fb9…` |
| `0.3.306` | 2026-10-21 02:29:36 | `d5515e70cefb2d56845e9eb140911e84aa274ba405b6b883acaeb19b7f590f40` | `2.1.306` | `01ae8327ca9931db91a0802d6e8e377419dd44e6` | `2026-10-21T01:32:56Z` | 21 | `7bfe67770c918a4a…` |
| `0.3.307` | 2026-10-21 20:46:42 | `ee3f44f72bee95824d3e5908007d5a679e615274ef1dca46014c43d906a0774a` | `2.1.307` | `b208bbbc9ae5dc87a1a0a7f964be47e27a0543ab` | `2026-10-21T20:07:02Z` | 21 | `3f9da77ed96a4fcd…` |
| `0.3.308` | 2026-10-22 04:19:14 | `bcff3ead267f10f5c9ed4bf9234174311129381ab337dae2e6b2309a090043df` | `2.1.308` | `320f66eb8f834be58a10c98283ebef19e3640c37` | `2026-10-22T03:48:37Z` | 21 | `8bde69e39ad899a7…` |
| `0.3.309` | 2026-10-22 20:35:53 | `15b3bc4222e0e80abfba9ee22fe3f8771e446ac6badc684a60cc29190938f3a4` | `2.1.309` | `1ce7e4ff2da8d4b0f5c15b291e4a9b54716970c2` | `2026-10-22T19:32:59Z` | 21 | `9ab39840239e1b24…` |
| `0.3.310` | 2026-10-23 21:21:54 | `110e2584420b1feff99694efafc58fdb2f35d7a0c5230abe152c1d82fee0384c` | `2.1.310` | `0d2056ecf42d9452a8cd8e054600024e7c557add` | `2026-10-23T20:13:54Z` | 21 | `48b4a291a3398ac9…` |
| `0.3.311` | 2026-10-26 19:49:59 | `c2683013d4b9031be6714fb532d507cc7ab55b1ea5f76c08e14916587b7d4ee7` | `2.1.311` | `e7d3b8365a0ffbe2f5dd032feb28e9fd9c7de025` | `2026-10-26T19:03:46Z` | 21 | `567bdb55b32ff11e…` |
| `0.3.312` | 2026-10-27 18:37:18 | `98a6c27b00b273fa0598b80540aa152d9fb58b6a1ac885b5993381e98469926d` | `2.1.312` | `f375cd85d4f70920de17ce3a4ad646e18834e833` | `2026-10-27T17:16:03Z` | 21 | `957aba8aaf21e9d0…` |
| `0.3.314` | 2026-10-30 00:14:37 | `603716a11e4f9c5ca76ed69d2da61e321f54ffcf0265a3eec7351cff19782e04` | `2.1.314` | `5c99a5b6399766569a32979c715862fdb61c7efa` | `2026-10-29T23:00:10Z` | 21 | `e8cd1ea742528e90…` |
| `0.3.315` | 2026-10-30 21:04:20 | `901fdc4fb347006ab9f322f19f5f57452e85cf77706f29446d78761b4f737976` | `2.1.315` | `4f69bb04c32a761bb81e3d764a864c028de92f09` | `2026-10-30T20:04:40Z` | 21 | `9eeb3ec86f213a75…` |
| `0.3.316` | 2026-10-31 00:39:30 | `d0e613aeaa8d861dcaecfee281baa8998cb05a1a87407d080ccce941cbddca92` | `2.1.316` | `cd7ceb459c6864567b16a11f60101b24421475c1` | `2026-10-30T23:47:48Z` | 21 | `b06fe72c5c745ce9…` |
| `0.3.317` | 2026-11-03 00:02:04 | `e91e5a4347e0cd8c51c5d2e8dac005769c2d1ddcb53102f14b5506646485112f` | `2.1.317` | `3d3e7cb16b5577dccaee9520c8dc1189d02d61b7` | `2026-11-02T22:39:48Z` | 21 | `a374a2e4ba725610…` |

The retained `0.3.295` base bytes were reacquired only after the tarball
SHA-256 matched the accepted Research 420 identity and all 19 retained
per-file hashes reproduced. Every hop tarball matched npm SHA-1 and SHA-512
integrity before inspection. Each wrapper's `package.json` pins
`claudeCodeVersion` and all eight optional platform dependencies to its exact
version; each embedded `manifest.json` names the coupled native `2.1.x`,
a private native commit, build time, harness schema `1`, signature
enforcement `flag`, and the eight platform checksums/sizes. The vendor
dropped the optional `modsCommit` manifest key after `.295`; the manifest key
set is otherwise unchanged across the extension. The manifest's
`testedWrapperVersions` list stays stale vendor metadata and is not runtime
evidence.

## Method

The route loads exactly one shipped file: the package `.` export resolves to
`sdk.mjs`, which imports only Node builtins and references neither the
rotating `core-*.mjs` chunks nor the `/core`, `/bridge`, `/browser`, or
`/extract` subpaths. For each published hop the wrapper tarball was extracted
into a run scratch directory and expanded into a deterministic complete
file-tree inventory with per-path SHA-256 (`dist-inventory.json`, schema v1).
No platform native package was downloaded (the Research 367 precedent binds
native identity through each shipped manifest); no artifact was executed; no
prompt, login, credential, or live session occurred; the host was not
installed, updated, or replaced.

Selected-surface comparison per hop, on `sdk.mjs` only:

- a tokenized structure diff with minified identifiers and string interiors
  masked (keywords, property names, object keys, numbers, and punctuation stay
  visible), plus
- an exact unmasked string-literal set ledger per hop.

Findings, identical at every hop: the query option-defaulting block (the
public option key set with defaults, including `canUseTool`), the selected
`Query` method bodies (`initializationResult`, `supportedModels`,
`accountInfo`, `mcpServerStatus`, `setPermissionMode`, `setModel`,
`interrupt`), the argv builders around `--model`, `--permission-mode`,
`--mcp-config`, `--setting-sources`, and `--effort`, the `SIGTERM`/`SIGKILL`
delayed cleanup bodies, and the `options.spawnClaudeCodeProcess` dispatch
branch. Differences are minifier symbol names, the embedded
`CLAUDE_AGENT_SDK_VERSION` stamp, and generated chunk filenames. `.304` adds
one SDK-internal config key (`codeRunningMcpTools`) that this route never
sets. No selected method body changed.

The exact string ledger bounds every content change. No selected literal
(message types, control-request names, permission modes, MCP prefixes, route
flags) was added, removed, or changed. The only `--`-prefixed additions are:
SIGNED-CACHE-ROOTS markers and three git invocation arguments
(`--untracked-files=normal`, `--ignore-submodules=dirty`, `--no-optional-locks`)
inside SDK-internal frozen git/cache constant arrays, and `--sdk-url` with
`--sdk-url=` at `.317`, which appear only in a nested-session argv detection
predicate and are never emitted by this route's builder. One SDK-internal MCP
tool-name parsing helper using the existing `mcp__` prefix constant appears at
`.299`; the sidecar mediates every `mcp__` call through its own `canUseTool`
independently of it.

Declaration files (`sdk.d.ts`, `sdk-tools.d.ts`) are classified in the fixture
protocol: settings rows (sandbox, auto mode, plugin digests, SSH hosts,
attribution), capability strings, hook `onFailure` policy with its
`hook_on_failure_supported`/`hooks_applied` gates, new `SDKMessage` variants,
cache-creation usage splits, `mcpMount` settings source, and startup-failure
reason additions. The sidecar's actual configuration and projection bound
every one of them: `settingSources: []`, `skills: []`, `plugins: []`,
`agents: {}`, and `hooks: {}` keep the new settings and hook policy
unreachable; `projectMessage` switches on a closed `message.type` set and
projects every unknown variant as unknown; the usage projection keeps its four
bounded per-turn counters; the account projection emits presence labels only;
MCP status rows beyond the projected bounded fields stay discarded; no callback
ever emits `updatedPermissions`; and no `pasted_content` field is sent. The
route calls neither server mutation, tool toggle, OAuth, resource, plugin,
context-usage, nor sdk-tools APIs.

## Claims and preserved boundaries

The SDK package claim stays baseline `0.3.284`, latest qualified `0.3.317`;
the native claim stays baseline `2.1.284`, latest qualified `2.1.317`. Both
retain their claim IDs, `QualifiedOnly`, behavior revision
`claude-agent.sdk-v1`, and point-for-point pairing. The default is still
`0.3.293`/`2.1.293`; the sidecar accepts only the finite maintained pair map,
which now carries 31 exact pairs and keeps the unpublished `0.3.297`,
`0.3.298`, and `0.3.313` points (and every synthetic later point such as
`0.3.318`) out. The separate Node `22.23.2..=22.23.3` segment (Research 387),
exact wire, sidecar source tag, public operations, permissions, credential
handling, environment allowlist, tool set, usage projection, lifecycle, and
feature-matrix dispositions are unchanged. Research 420 remains the
historical `.294`/`.295` record; its provider-free pair proof pattern extends
to the new pairs through the prepared facade and shipped sidecar: all 31
maintained pairs open against deterministic fake SDK/native controls, and
unsupported, gap, mismatched, missing, malformed, and unreadable identity
cases reject before construction. No live provider session, prompt,
credential, installation, artifact execution, host mutation, tag, or
publication occurred.

## In-run official movement

None at claim-edit time; the pre-push re-probe is recorded in the fixture and
research record above. Nothing is published after `0.3.317`.

## Sources

- npm package metadata: <https://registry.npmjs.org/@anthropic-ai/claude-agent-sdk>
- Upstream changelog (discovery only):
  <https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/CHANGELOG.md>
- Frozen evidence:
  [`fixtures/claude-agent-sdk-0.3.317`](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-0.3.317/dist-inventory.json)
