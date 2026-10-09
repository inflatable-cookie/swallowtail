# Research 420: Claude Agent SDK 0.3.295 Qualification

Date: 2026-10-09
Route: `claude-agent.sdk`
Axes: coupled SDK wrapper package and embedded native; Node, sidecar wire and
sidecar source remain independent.

## Decision

Qualify every published package/native pair from the retained
`0.3.284`/`2.1.284` baseline through official npm `latest`/`next`
`0.3.295`/`2.1.295`. Preserve the package and native baselines, claim IDs,
`QualifiedOnly` posture, route behavior revision, exclusions and all prior
pairs. The adjacent hops `.294`/`2.1.294` and `.295`/`2.1.295` are contiguous;
no published point is missing in this extension.

The `.293` default pair remains unchanged. The preparation facade selects only
an exact maintained pair; the driver binds that selection into its launch
messages; and the shipped sidecar verifies package metadata and the embedded
native manifest before importing the SDK or constructing a query. The added
points do not qualify registered-tool live use. Research 301 remains exact to
SDK `0.3.259`/native `2.1.259` on Darwin arm64.

## Official identity and provenance

The selected channel is npm `dist-tags.latest`, re-probed immediately before
push at `2026-10-09T00:14:21Z` against the
[official npm registry metadata](https://registry.npmjs.org/@anthropic-ai/claude-agent-sdk).
`latest` and `next` both resolved to `0.3.295`. The exact tarball URLs,
publish times, npm SHA-1 and integrity metadata, SHA-256, package tree digests,
per-file hashes, native manifest identities, all eight platform binary
hashes/sizes and adjacent path sets are retained in the
[`0.3.295` fixture](../../crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-0.3.295/README.md).
The fixture does not contain vendor source or executable bytes.

| SDK | Published (UTC) | Tarball SHA-256 | Native | Manifest commit | Build time |
| --- | --- | --- | --- | --- | --- |
| `0.3.293` | 2026-10-07 17:21:04.283 | `395bfbda4294c6334fa18be3706a46ff31851b4f4a6f74ba11ac96c9d97ced9f` | `2.1.293` | `3abc54a9d60b4d12c627afad22d6e5f58a6199d2` | `2026-10-07T06:56:40Z` |
| `0.3.294` | 2026-10-08 16:36:21.522 | `2c0a2db22531e690cfac98da0c764828d43b4cde38fca941dce7403cd49400d5` | `2.1.294` | `8f033c6ebe3d82a87f502e199307f38f5d55ccca` | `2026-10-08T03:11:23Z` |
| `0.3.295` | 2026-10-08 18:22:56.422 | `704b1228401f951e1cdb3a1782b97893d3caee64afe162a4f7fbda8106273c7b` | `2.1.295` | `07e8f67ea3282bf154a9e05673a0942b1b173cef` | `2026-10-08T17:09:12Z` |

The `.293` source bytes were reacquired only after the tarball digest matched
the accepted Research 416 identity. `.294`'s published identity reuses
Research 416 and its selected surfaces were independently compared in this
run. `.295`'s published package matched npm SHA-1 and SHA-512 integrity before
inspection. Both new embedded manifests bind eight platform-specific native
binary hashes and sizes. The manifest's harness schema stays at `1`; its
`testedWrapperVersions` list stops at `0.3.285` and is not runtime evidence.

For the host platform, the `.294` Darwin arm64 native package tarball SHA-256
is `471c95f94a6d3de0c8f83a412bdf17d7322352ce8d91250a7d683a97361f9ea0`; the
extracted binary is 236,330,608 bytes and matches manifest SHA-256
`def0d15e64dd7d89621f88d28214f885b1c38b0ddd69762fb8593e34915d6d53`. The
`.295` Darwin arm64 package tarball SHA-256 is
`af3a869fad837511b9e784d2540e8a4a5982f2a9896355b0990ec1c031f549c0`; its
239,695,888-byte binary matches manifest SHA-256
`0116ee2e0a513900b633d9951367f18747686478e2b462805b8c31609f047f70`. Static
string inspection found the selected flag spellings in both binaries,
including `--model=`, `--permission-mode=`, `--mcp-config=`, and
`--setting-sources=`. Neither package nor binary was executed. Other platform
native package bytes were not downloaded; their exact hashes and sizes are
bound by each wrapper's manifest.

## Complete adjacent package changes

Every path in the 19-file npm wrapper is classified in the fixture inventory.

| Hop | Added | Removed | Changed | Identical |
| --- | --- | --- | --- | --- |
| `.293` → `.294` | `core-9fynjcde.mjs`, `core-z7m2ffy5.mjs` | `core-ae32wa3s.mjs`, `core-d0szsqzn.mjs` | `bridge.mjs`, `browser-sdk.js`, `core.mjs`, `manifest.json`, `manifest.zst.json`, `package.json`, `sdk.mjs` | `LICENSE.md`, `README.md`, `agentSdkTypes.d.ts`, `bridge.d.ts`, `browser-sdk.d.ts`, `core.d.ts`, `extractFromBunfs.d.ts`, `extractFromBunfs.js`, `sdk-tools.d.ts`, `sdk.d.ts` |
| `.294` → `.295` | `core-reghh1vn.mjs`, `core-tm6xdb2r.mjs` | `core-9fynjcde.mjs`, `core-z7m2ffy5.mjs` | `bridge.mjs`, `browser-sdk.js`, `core.mjs`, `manifest.json`, `manifest.zst.json`, `package.json`, `sdk-tools.d.ts`, `sdk.d.ts`, `sdk.mjs` | `LICENSE.md`, `README.md`, `agentSdkTypes.d.ts`, `bridge.d.ts`, `browser-sdk.d.ts`, `core.d.ts`, `extractFromBunfs.d.ts`, `extractFromBunfs.js` |

For `.293` → `.294`, `sdk.mjs` changes only three package-version stamps.
`core.mjs` changes its version stamps and the two generated chunk references;
the SDK query and option paths are otherwise byte-identical after tokenizing
the minified output. The declarations and peer dependency ranges are
unchanged. `package.json` updates the package version, exact optional native
package versions, generated filenames and `claudeCodeVersion`. `manifest.json`
updates the native commit/build identity and all eight platform binary hashes
and sizes. The native `.294` release notes describe hook prompt handling; the
sidecar always supplies `hooks: {}`.

For `.294` → `.295`, `package.json` updates the exact package and all eight
optional native package versions, generated chunk names and
`claudeCodeVersion`. Peer dependency ranges and export map stay unchanged.
`manifest.json` updates native commit, mods commit, build time, and all eight
platform binary hashes/sizes; manifest signing policy and harness schema stay
the same. The complete manifest values are frozen in the fixture.

`sdk.d.ts` adds `AccountInfo.overageEnabled`, documents that more than 1,000
`pasted_content` entries/blocks are ignored and that the first 100 nonblank
entries are considered, and adds hook `onFailure: "block"` declarations.
`sdk-tools.d.ts` adds a PostToolUse page-text note; the sidecar imports no
`sdk-tools` subpath. The selected `Query` declarations and the sidecar's
selected call signatures remain unchanged. `bridge.mjs` and `browser-sdk.js`
change, but neither subpath is loaded by this route.

The package embeds a newly emitted `sdk.mjs`/`core.mjs` bundle and replaces
both generated core chunks. Exact hashes and complete path sets are retained
in the fixture rather than inferred from changelog titles. The SDK source
diff shows the selected invocation change below; the selected query methods
`initializationResult`, `supportedModels`, `mcpServerStatus`, `accountInfo`,
`setPermissionMode`, and `setModel` have byte-identical method bodies at both
points. `interrupt`, `setMcpServers`, and `toggleMcpServer` keep their request
shapes; only the generated helper name around their guarded execution changes.
The sidecar calls `mcpServerStatus`, but not
`setMcpServers` or `toggleMcpServer`.

## Selected invocation and control/data flow

In `.294`, the SDK's query option builder emits named options as two argv
elements, for example `K.push("--model", g)` and
`K.push("--permission-mode", S)`. In `.295`, the shipped `sdk.mjs` adds a
`je(flag, value)` helper that formats `--flag=value`; the builder calls
`K.push(je("--model", g))`, `K.push(je("--permission-mode", S))`, and the
same helper for effort, tools, disallowed tools, setting sources, resume,
working directories and the other named scalar options. `a_` keeps a
dash-prefixed value attached to its flag; for `--mcp-config`, the JSON string
does not start with a dash and retains the same flag/value pair. Boolean flags
remain flag-only. The SDK sends the resulting argv to the native process
through its existing spawn hook. This changes token grouping while preserving
selected names and values. The matching native binary contains the equal-form
selected flag spellings. No extra option, path, server, tool or permission is
selected.

The sidecar's actual open configuration bounds the rest of the changed SDK
surface:

- `settingSources: []`, `skills: []`, `plugins: []`, `agents: {}`, and
  `hooks: {}` keep user configuration, extensions, subagents and hooks out of
  the SDK query.
- `tools` is the route's closed admitted set and `disallowedTools` contains
  every withheld admissible tool. `Agent`, `Task`, `WebFetch`, and `WebSearch`
  are unavailable. The sidecar never supplies `allowedTools`.
- `mcpServers` contains only caller-declared stdio servers and
  `strictMcpConfig: true`. Required servers request `alwaysLoad: true`; only
  explicitly optional servers can use the SDK tool-search path. Longer tool
  descriptions can change discovery within those declared servers, but do not
  add a server or bypass call mediation.
- `canUseTool` returns only allow with the original input as `updatedInput`,
  or deny with a bounded message. It never returns `updatedPermissions`, so
  persistent permission changes and the new large-`updatedPermissions`
  behavior are unreachable.
- The account projection reads `apiProvider` and emits only presence booleans
  for `subscriptionType`, `tokenSource`, and `apiKeySource`; it ignores new
  `overageEnabled` and other account fields. The rate-limit path validates
  `status`, `uuid`, and `session_id`, records only an active-turn status, and
  discards added overage fields.
- Assistant projection forwards text and tool-use identity only; citations
  and other part metadata are not copied. User projection forwards tool-use
  id and error presence, not tool-result body or metadata. Result usage
  accepts four nonnegative bounded per-turn token counters; cumulative usage,
  costs, context, and added quota fields stay out of the wire.
- The sidecar sends no `pasted_content` field. It does not call the SDK's
  context-usage, MCP server mutation, tool toggle, OAuth, authentication,
  resource, plugin, hook, or sdk-tools APIs.
- The sidecar retains the existing input-stream lifetime and `Query.close()`
  path. Its native handle is joined separately to the existing 2,000 ms bound;
  host cleanup and the degraded root-only platform posture do not depend on
  the SDK close promise.

The selected SDK `Query.close()` body is compared directly across
`.294`/`.295`: both abort before spawn when needed, end and clear stdin, remove
the abort listener, deliver any already-observed exit once, detach exit
listeners, forward abort state, and untrack the child. For a still-running
child, both retain the same platform branch and delayed `SIGTERM`/`SIGKILL`
cleanup. The changed text in that body is minifier renaming of the abort-error,
logger, process-manager and timeout-helper symbols; the control flow and
delays are unchanged. The sidecar continues to join its native handle at the
same 2,000 ms limit. This closes the selected lifecycle comparison without
using a fake transcript as evidence about vendor shutdown.

The `.294` selected code change is only SDK package version stamping and
generated core-chunk references; the native artifact identity and binaries
rotate, while its published hook behavior is unreachable under the empty hook
configuration. The `.295` native release notes add hook failure policy; the
same empty configuration excludes it. The `.295` SDK release notes describe a
16,384-character cutoff for tool-search descriptions, preserved citations in
streamed assistant text, corrected MCP tool-list refresh after
`mcp_set_servers`, `toggleMcpServer` fixes, added overage fields on rate-limit
events, fail-closed handling of permission `updatedPermissions` over 4,096
entries, a 1,000-entry `pasted_content` bound, and bounded `tool_use_result`
output. The sidecar's actual config and projection map these changes as
follows: optional-server tool discovery stays within the declared stdio set;
output text passes through while citation metadata is omitted; the route calls
neither server mutation method; added rate-limit values and account overage
fields are ignored; no callback emits `updatedPermissions`; no pasted-content
field is sent; and tool-result output and metadata are not projected. The
release notes are supporting context, not the compatibility oracle.

The native source commits are not present in the public Claude Code source
repository, so this record does not claim a source-level review of private
native code. The exact package and manifest identities bind the target; the
downloaded Darwin arm64 bytes match their manifest; static inspection confirms
the new invocation syntax is recognized. This is a remaining provenance limit
for platform binaries that were not downloaded. It does not transfer a
qualification to another platform package or change the route's existing
platform posture.

## Claims and preserved boundaries

The SDK package claim remains baseline `0.3.284`, latest qualified `0.3.295`;
the native claim remains baseline `2.1.284`, latest qualified `2.1.295`.
Both retain their claim IDs, `QualifiedOnly`, behavior revision
`claude-agent.sdk-v1`, and point-for-point pairing. The default is still
`0.3.293`/`2.1.293`; the sidecar accepts only the finite maintained pair map.
The separate Node `22.23.2..=22.23.3` segment (Research 387), exact wire,
sidecar source tag, public operations, permissions, credential handling,
environment allowlist, tool set, usage projection, lifecycle and feature-matrix
dispositions are unchanged.

Research 416 remains the historical `.293` qualification and `.294` stop. The
provider-free ten-pair proof it records is reused. This record adds proof only
for `.294` and `.295` through the actual prepared facade and shipped sidecar.
Both new pairs open against deterministic fake SDK/native controls; mismatched
package/native versions and unsupported `.296` reject before construction;
the existing missing, malformed and unreadable identity cases remain selected
regressions. No live provider session, prompt, credential, installation,
artifact execution, host mutation, tag or publication occurred.
