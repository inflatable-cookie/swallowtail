# 485 Pi RPC 1.6.0 Identity

Observed 2026-11-22 for `pi.rpc` on `pi.package`. npm `latest` and the
GitHub latest stable release both point to `1.6.0`. The identity is frozen.
Qualification stops at the retained `1.4.0` ceiling. **1.5.1 enables idle
cache warming by default, and 1.6.0 changes tool activity after an abort.**

## Artifact identity

| Package | npm publication (UTC) | GitHub release publication (UTC) | GitHub tag commit |
| --- | --- | --- | --- |
| `1.4.0` | 2026-10-21T22:48:58.431Z | 2026-10-21T22:49:48Z | `4a080252f06caf63a525d7873c5d94d3c23887e0` |
| `1.5.0` | 2026-11-02T11:05:09.564Z | 2026-11-02T11:05:31Z | `f9a26365568127461c47a07a37dcb42c140a3ac5` |
| `1.5.1` | 2026-11-09T23:45:13.428Z | 2026-11-09T23:47:44Z | `dd30101cfc5b0609411dd80b6f93adbc14db14cf` |
| `1.5.2` | 2026-11-10T12:24:55.226Z | 2026-11-10T12:25:43Z | `b4afdddf44e059c67ccf5097e5317d913521b470` |
| `1.6.0` | 2026-11-20T11:09:15.529Z | 2026-11-20T11:35:03Z | `faac585884ae2c013a6b1400f3e9e77f10b34bac` |

The [frozen ledger](../../crates/swallowtail-adapter-pi/tests/fixtures/pi-rpc-1.6.0/)
lists each tarball URL, SHA-256, npm SHA-1 and SHA-512 integrity, complete
tree hash, release ID, and declared dependency pins. Each package has null
`gitHead`. Each registry SLSA payload names the downloaded tarball's SHA-512
and the GitHub tag commit as the checkout. The comparison checked payload
contents and did not verify attestation signatures. The 1.4.0 tarball and
recursive tree hashes match [Research 455](455-pi-rpc-1-4-0-identity-and-qualification.md).
The npm package requires Node `>=22.19.0` at every point.

The installed Pi package manifest identifies `1.6.0`. Its resolved launcher
and selected CLI chunk match npm `1.6.0` byte for byte. Node on the observation
shell's PATH reported `v26.11.0`. Pi was inspected as files. No Pi executable,
login, provider request, catalogue lookup, installation, or host change was
needed.

| Published hop | Added | Removed | Changed | Identical |
| --- | ---: | ---: | ---: | ---: |
| `1.4.0` → `1.5.0` | 48 | 41 | 319 | 919 |
| `1.5.0` → `1.5.1` | 76 | 38 | 259 | 989 |
| `1.5.1` → `1.5.2` | 27 | 27 | 29 | 1268 |
| `1.5.2` → `1.6.0` | 46 | 42 | 166 | 1116 |

The inventory records every regular file in those npm trees and assigns
every differing path a review class. All shipped JavaScript sourcemaps with
embedded content in the coding-agent and same-version agent-core packages
match the tagged source byte for byte. The npm executable follows
`dist/bundle/cli.js` → `cli-runtime.js` → the hashed main chunk. The chunk
contains both `runRpcMode` and the agent-core tool loop. For the stop below,
the reachable compiled tool loop and its source correlate independently of
the unbundled RPC files.

## Selected RPC surface

The strict-LF line reader, command types and JSON event conversion retain the
same source bytes across all points. The RPC command table retains the
selected catalogue/state, steering, follow-up, compaction and retry setters,
`prompt` and `abort`. Prompt dispositions stay `handled`, `queued` and
`started`, with `handled` and `queued` for steer and follow-up. The `prompt`
image field stays `images` with `type`, `data`, and `mimeType`. Swallowtail
admits a bounded PNG on a prompt and sends text-only steering and follow-up.
The decoder continues to read assistant `message_end` token counters
`input`, `output`, `cacheRead` and `cacheWrite`, and the required boolean
`agent_settled.aborted`. Stable wire keys do not establish the event sequence
or authority to send requests between turns.

- **1.5.0.** RPC mode is byte-identical. The CLI help and config add an
  explicit self-update command override. The selected launch passes
  `--offline` and does not call the update subcommand. Startup trust now
  respects stored denial even when no trust-requiring resource is detected.
  Explicit model resolution and the adapter's `get_state` check remain.
  The source changes around tool schemas and stream parsing operate inside
  the agent and downstream API, with the existing four-tool allowlist.
  `grep` no longer treats lone CR as a line separator. Anthropic summary
  auth defers to the runtime stream on compaction and branch-summary paths
  outside the selected commands. Added cost attribution and reported model
  metadata stay outside the four-counter projection.
- **1.5.1.** Prompt preflight reserves the context against tree navigation
  while retaining the dispositions. Deferred normalization of queued images
  is upstream behavior. The adapter admits image-bearing prompts into an
  idle turn and sends no images in steer or follow-up. Idle prompt
  normalization retains the same input shape and uses the existing model
  profile. The RPC signal handler now preserves disposal across stdin EOF
  and the first shutdown signal, with a repeat-signal forced exit. The
  adapter still owns host stop, bounded force-stop and join. Deferred
  `ctx.shutdown()` is reached only by an extension. Read tools revise
  terminal escaping, output bounds and cancellation of search children
  while keeping the selected tool names. Login, model cycling, branch
  summaries and MCP additions supply no selected operations. The default
  cache warmer change reaches a selected session after a prompt, as below.
- **1.5.2.** RPC and agent-session sources match 1.5.1, and the bundled
  parallel tool loop is identical. The session runtime removes a copy
  after a failed import cwd check. Fresh `--no-session` gives that path no
  authority. The other coding-agent source changes concern missing cwd
  recovery for stored sessions. The Anthropic adapter updates the version
  of its downstream request identity. Idle warming persists.
- **1.6.0.** Diagnostics move through a shared stderr helper, and session
  rebind subscribes before extension startup. The selected route suppresses
  extension loading and does not use new/switch/fork/clone RPC replacements.
  The tool restore changes apply to restored or reloaded loadouts, outside
  the fresh plain allowlist. `read`, `grep` and `find` adjust bounded output
  and path handling. Provider streaming and auth updates are downstream of
  the selected catalogue/state and assistant-event boundary. No new token
  field is required by the decoder. Idle warming persists and the bundle
  now emits extra tool records when a batch is interrupted.

## Earliest stop — idle warming in 1.5.1

`SETTINGS_DEFAULTS.cacheWarming` moves from `streaming` to `idle` and
`getCacheWarmingMode()` uses it when global settings have no recognized
override. The new literal is in the shipped CLI's imported
`chunk-4UNPP53I.js` in 1.5.1 and 1.5.2, and `chunk-XGGZYFNG.js` in
1.6.0. The CLI's selected `createAgentSession` builds a `CacheWarmer`
and starts it on requests matching the session ID. The ephemeral
`SessionManager.inMemory` still supplies that ID. The current-context
predicate checks the selected model and transcript prefix, with no
settlement or persistence condition. `--offline` disables startup
network work, and `--no-extensions` suppresses extension loading. Neither is
a cache-warming control.

In streaming mode, settlement stops the warmer. In idle mode,
`onAgentSettled` can leave it armed after the terminal record. If the
request is replayable and the cache lifetime, context and economics checks
pass, `refresh` calls `ModelRuntime.streamSimple` again. A successful
refresh records separate `cache_warm` usage and emits `entry_appended`,
outside Swallowtail's assistant-message usage accounting. An idle record
can hit the adapter's requirement for an active turn. This is a source
finding about possible work after settlement. It does not assert a live
request, a guaranteed warm under every model, or that earlier installs
explicitly configured for idle mode were safe.
[1.5.1 settings default](https://github.com/earendil-works/pi/blob/dd30101cfc5b0609411dd80b6f93adbc14db14cf/packages/coding-agent/src/core/settings-defaults.ts),
[selected session construction](https://github.com/earendil-works/pi/blob/dd30101cfc5b0609411dd80b6f93adbc14db14cf/packages/coding-agent/src/core/sdk.ts),
[cache warmer](https://github.com/earendil-works/pi/blob/dd30101cfc5b0609411dd80b6f93adbc14db14cf/packages/coding-agent/src/core/cache-warmer.ts).

## Second stop — aborted tool activity in 1.6.0

Through 1.5.2, the sequential and parallel loops stop iterating an
interrupted tool batch. In 1.6.0 they visit each remaining call and emit
`tool_execution_start` and `tool_execution_end` with `isError: true` for
calls whose bodies are skipped. In the default parallel preparation path,
RPC stdout backpressure can suspend event processing while the separate
`abort` command runs. The session forwards tool events through
`toJsonEvent` unchanged. Swallowtail decodes and projects the additional
pair as `ProviderOwnedTool` activity, first `Started/InProgress` and then
`Completed/Failed`, before the same cancelled terminal outcome. The fake
transcripts exercise that projection without calling Pi.
[Upstream change and deterministic regression](https://github.com/earendil-works/pi/commit/4438c58ba2208a8a5d0d27f0ca0b8139748f8ff3).

The exact 1.6.0 loop sets a numeric `durationMs` after entering the tool
body and carries it through finalization even on thrown errors. Presence
at `tool_execution_end`, including zero, is evidence that the body was
entered on this path. A skipped call has no duration, but so do other
pre-execution failures such as unknown tools, validation errors and
pre-hook rejections. The start record has no skip flag, and the end record
has no typed abort-before-execution reason. Matching the error text or
suppressing every no-duration failure would not preserve the old activity
stream. The adapter currently projects only the call ID, name and
`isError`. Any revised mapping must account for what can be known at start
and end.
[1.6.0 tool-loop source](https://github.com/earendil-works/pi/blob/faac585884ae2c013a6b1400f3e9e77f10b34bac/packages/agent/src/agent-loop.ts).

## Decision

Keep `pi.rpc.package-window-2`, every earlier qualified segment, and
`AllowUnverified` above `1.4.0`. The planner owns the Contract 029 No Terminal
Stop follow-up. It needs deterministic proof of cache-warming suppression
and of the selected interrupted-batch projection, or a ruling on a distinct
consumer-visible behavior revision. The evidence supports neither a
qualification extension nor a claim about other Pi routes.
