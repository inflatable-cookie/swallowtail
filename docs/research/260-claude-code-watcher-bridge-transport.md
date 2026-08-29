# 260 Claude Code Watcher Bridge Transport Evidence

Status: complete; boundary promoted
Owner: Tom
Created: 2026-08-29
Updated: 2026-08-30
Card: g05.003 / 015
Depends on: Research 257; completed cards 009 and 014; Contracts 010, 041, and 059

## Question

Can the current Claude Code headless route carry operation-private watcher MCP
tools and a pre-terminal Stop hook into the in-process Swallowtail
`WatcherHostService`, with host ownership, exact turn correlation, fail-closed
authorization, and joined cleanup?

## Decision

No qualifying bridge exists in the current Swallowtail runtime. Research 260
closed its evidence lane as a stop. Card 010 remains planned and must not start
until the provider-neutral bridge core, current provider evidence, and live
same-turn gate close.

Claude Code 2.1.251 exposes the provider-side ingredients from Research 257:
private `--mcp-config`, `--strict-mcp-config`, `--settings`, `--add-dir`,
`--bare`, and `--include-hook-events`. Those flags are prompt-free/package
evidence only here. They do not create a host listener, hand an existing host
process or file descriptor to Claude, or prove same-turn Stop re-entry.

HTTP is the smallest plausible future carrier. It still needs a new host-owned,
operation-scoped listener and bridge contract. Stdio is not admitted because
Claude launches the local server from its command and arguments; the current
host has no operation-private IPC lease or provider-to-existing-process handoff.
SSE and WebSocket do not remove that listener boundary. `claude mcp serve` is
the reverse direction: it makes Claude Code the MCP server for another client.

## Promotion

On 2026-08-30 the operator selected the minimal HTTP candidate for promotion.
[Contract 060](../contracts/060-operation-scoped-watcher-http-bridge.md) now
owns the provider-neutral operation bridge: closed reserved watcher MCP,
operation-private bearer authority, exact host/operation/turn correlation, a
completion barrier, and joined listener and private-material cleanup.

This promotion resolves the product and contract decision only. It does not
change the current-runtime evidence above, qualify a Claude version, authorize
a live provider turn, or advertise watcher support. g05.003 card 016 owns the
provider-neutral host bridge. Cards 010-011 remain later Claude binding and
acceptance work.

## Current provider evidence

The installed provider is the exact native `darwin-arm64` binary at
`/Users/tom/.local/share/claude/versions/2.1.251`:

| Evidence | Value |
| --- | --- |
| Version | `2.1.251 (Claude Code)` |
| Native binary SHA-256 | `625869b01e0050f260b2980fac248fd9cef9e462612bded4ec9d3d49ff8969a5` |
| `--help` SHA-256 | `5ff2e7a0bca8535fb9ec097fa0a21e9d6b735ed94104fa0d1f58ac73a841d52d` |
| Native package | `@anthropic-ai/claude-code-darwin-arm64@2.1.251` |
| Native package tarball SHA-256 | `cb3ecffa649ea20b78f3b7fe4a7395d7a225a510ae18521f6b65c669ecf4d9fd` |
| Installed/native extracted binary | exact SHA-256 match |
| Wrapper package | `@anthropic-ai/claude-code@2.1.251` |
| Wrapper tarball SHA-256 | `44d28caf1711767c14a0388db56b13f49dbd8d3e1db635dd98aa3115c760cf27` |

The npm registry identities were also frozen without installation:

- wrapper integrity: `sha512-eG+ZPPpW2Dbmnntf1Fz9/T9ewS8I8SKfc1tcU2PqSwmftfjRPP7BXPaCyLuZ8kvgTdiPnJi/2/JnTvTRieneEQ==`
- native integrity: `sha512-Qr5oMGVrOUyatsMlK0361OSnr3C785QBFIDoaiHMpaJ/nu/Ji2ccwI7nv0o54q3v3Y+zU9xbtEmcGxPRcR9ptA==`

The current help surface advertises `--bare`, `--mcp-config`,
`--strict-mcp-config`, `--settings`, `--add-dir`, and
`--include-hook-events`. It also advertises `--restricted`, but that mode does
not provide an MCP listener, IPC handoff, turn correlation, or watcher
authority.

The current official references were retrieved as markdown on 2026-08-29:

| Reference | Use | SHA-256 |
| --- | --- | --- |
| [MCP](https://code.claude.com/docs/en/mcp) | HTTP, SSE, stdio, WebSocket, scopes, startup and tool timing | `be0624af48b735a4bd2085cbd2d6e227aca3a8c5b3ebf7a40d201c796dad3590` |
| [Hooks](https://code.claude.com/docs/en/hooks) | Stop, StopFailure, HTTP and MCP-tool hooks, decision control | `efb33816ae74fc832c47ba45b57c882067d80538407a8d9af8399367d44894f6` |
| [Headless](https://code.claude.com/docs/en/headless) | `--bare`, MCP startup, background work, SIGTERM | `02f132525d009deae88590647bcac872eb2e5d6cca39c4ba3008a0f810db8314` |
| [Skills](https://code.claude.com/docs/en/skills) | `--add-dir` skill loading and bare-mode exception | `79fcd0c50f8fff8319754b57b379183ef60512ac06c383ff354f4241e6210bbd` |
| [Settings](https://code.claude.com/docs/en/settings) | session settings and precedence | `a6f10afd9d41fffa2b14e1f113ef641ea6fbe9b319c114bb986efc3e186de009` |
| [CLI reference](https://code.claude.com/docs/en/cli-reference) | exact flag membership and `--restricted` floor | `b5b4585ce917230d12e0d9f95600868e0b182e09ce659e7d2848638e06dc4a9d` |

The provider docs identify HTTP as the recommended remote MCP transport. They
also state that stdio servers run as local processes launched by Claude Code,
that SSE is deprecated, and that WebSocket uses a JSON `type: "ws"` entry
without a `--transport ws` CLI option. The docs separately describe
`claude mcp serve` as a stdio server with Claude Code on the server side.

## Provider-side seam

The provider-side composition remains exact as a candidate mechanism:

```text
--bare
--mcp-config <inline JSON or operation-private file>
--strict-mcp-config
--settings <inline JSON or operation-private file with hooks.Stop>
--add-dir <operation-private root containing .claude/skills/.../SKILL.md>
--include-hook-events
```

Under current docs, a `Stop` hook runs after the main agent finishes
responding. It can return `decision: "block"` plus `reason`, or
`hookSpecificOutput.additionalContext`, and the conversation continues through
the same Stop-loop protection. `stop_hook_active` identifies continuation and
Claude Code overrides after eight consecutive blocks. `StopFailure` runs for
API errors and has no decision control. These semantics are sufficient to
describe a candidate completion gate, but they are not a live proof for
2.1.251.

The current docs also describe `background_tasks` entries for provider-native
shell, subagent, monitor, workflow, teammate, cloud-session, and MCP tasks.
Those ids and lifecycle rules remain distinct from Contract 059 watcher ids,
host ownership, and joined cleanup.

### Prompt-free probe boundary

The exact 2.1.251 binary was run once with `env -i`, a throwaway `HOME`, no
prompt, and no credentials. The composition included `--bare`, an inline
strict HTTP MCP entry targeting an unused loopback port, an inline Stop command
setting, and a throwaway `--add-dir` root. The process returned:

```text
Error: Input must be provided either through stdin or as a prompt argument when using --print
```

This proves that the selected flags reached the input-validation path. It did
not connect to the MCP endpoint, execute the Stop hook, send a model request,
or prove same-turn continuation. No login, credential, paid work, or provider
prompt was used.

The prior Research 257 window remains `2.1.220..=2.1.241`. The current
2.1.251 identity is outside that window. No current qualified version segment
is admitted: the current point has package/help/parser evidence, but no live
same-turn proof and no host transport.

## Transport comparison

| Candidate | Provider owner | Required Swallowtail owner | Correlation and authorization | Cleanup | Disposition |
| --- | --- | --- | --- | --- | --- |
| HTTP MCP, with Stop `mcp_tool` or HTTP hook | Claude opens a configured HTTP connection and sends MCP or hook requests | A host-bound listener plus a bridge task that dispatches only the reserved watcher family into the turn's `WatcherHostService` | An operation-private route/token must bind to the exact execution host, runtime turn, and operation; foreign, stale, reused, post-terminal, and malformed calls must fail closed | Stop new calls, finish or cancel the watcher gate, stop and join watcher work, close/join bridge task, release private material, then finish provider/process cleanup | Selected future boundary under Contract 060; current host still has no listener or bridge implementation |
| Stdio MCP | Claude launches the configured command and owns the stdio connection | A host IPC lease and a host-approved helper handoff would be required; passing a command alone makes the helper provider-launched | Helper identity, operation, host, and turn would need private authenticated IPC; raw command/PID authority is not sufficient | Provider-launched helper ownership and joined cleanup are not represented; a host-created process cannot be handed into Claude's MCP stdio channel | Rejected by the card stop condition |
| SSE MCP | Claude opens the remote SSE connection | Same host listener/bridge problem as HTTP, with deprecated provider transport | Same exact scope, token, and turn requirements | Same listener and watcher join requirements | Not selected; deprecated and no existing host listener |
| WebSocket MCP | Claude opens the configured `ws` connection | Same host listener/bridge problem; current CLI does not add `ws` with `--transport` | Header-only auth still needs exact operation/turn validation | Same listener and watcher join requirements | Not selected; no current host binding |
| `claude mcp serve` | Claude is the MCP server over stdio | Another client would have to connect; it does not make Claude consume Swallowtail watcher tools | Wrong direction for this route | Does not solve provider-to-host watcher ownership | Not a bridge |

### HTTP candidate delta

HTTP is the smallest future delta. The evidence lane made no implementation
decision; the later 2026-08-30 promotion admitted a distinct operation-scoped
host bridge service with at least:

1. host-owned loopback bind, close, and joined-task lifecycle; endpoint and
   authentication material that is operation-private and absent from durable
   public records;
2. a narrow MCP and Stop-hook dispatch surface limited to Contract 059's
   reserved watcher operations, with exact execution-host, runtime-turn, and
   operation correlation; and
3. a terminal barrier that rejects new calls, joins watcher work before a
   successful turn, and closes the bridge and its private material on
   cancellation, deadline, provider failure, hook failure, transport failure,
   and normal completion.

This is a distinct contract boundary, not an extension inferred from
`ServingEndpointService`, `NetworkPolicyService`, or the sign-in loopback
port. Contract 060 now owns it. The current runtime remains unchanged.

## Host ownership audit

The current implementation provides the watcher registry and ordinary process
supervision, but no provider bridge:

- [`LocalHostServices::compose`](../../crates/swallowtail-host-local/src/services.rs)
  registers `LocalWatcherHostService` alongside process, network, serving
  endpoint, and sign-in-related services. Registration does not bind a network
  listener.
- [`LocalProcessHostBuilder::approve_watcher_operation`](../../crates/swallowtail-host-local/src/host.rs)
  keeps the process recipe private behind approved `WatcherOperationData`.
  [`LocalProcessHost::start_process`](../../crates/swallowtail-host-local/src/process.rs)
  uses an approved executable, clears ambient environment, supplies pipes, and
  owns the local process group. It has no provider IPC or listener handoff.
- [`ServingEndpointService::publish`](../../crates/swallowtail-host-local/src/serving_endpoint.rs)
  validates a loopback HTTP endpoint already observed from an owned child and
  returns an opaque publication. It does not bind or start a listener.
- [`LoopbackCallbackService`](../../crates/swallowtail-runtime/src/sign_in_ports.rs)
  is explicitly a one-sign-in callback port. Its receipts do not carry secret
  bytes and its process helper remains a separate `ProcessService`; it cannot
  authorize an MCP bridge.
- [`HostServices`](../../crates/swallowtail-runtime/src/host_registry.rs)
  exposes the optional watcher port, but not an MCP listener, generic IPC
  channel, or bridge broker.
- [`LocalWatcherHostService`](../../crates/swallowtail-host-local/src/watcher.rs)
  owns watcher entries, stop/wait/join operations, turn retirement, and a
  defensive drop path. It has no provider-facing request decoder or transport
  endpoint.

Consequently, the current host can satisfy the watcher side of Contract 059
once it receives an authorized in-process request, but it cannot receive that
request from Claude Code through either admitted MCP transport.

## Acceptance disposition

| Card 015 gate | Result |
| --- | --- |
| Exact provider MCP and Stop surface for a named qualified segment | Current 2.1.251 flags and docs are frozen; no qualified segment is admitted because same-turn behavior was not live-proved |
| Host binding into the same turn-owned `WatcherHostService` | Not present; requires a new host listener/IPC contract |
| Private endpoint/token/path/helper/provider identities | Required shape is defined; current runtime has no bridge authority that can enforce it |
| Cancellation, failure, release, and joined cleanup | Existing watcher/process cleanup is host-owned; bridge/helper cleanup is not represented |
| Exact empty strict-MCP omission | Preserved; current production command remains unchanged and no watcher material is added |
| Live same-turn re-entry | Not run; no operator authorization for provider work; remains a named blocker |
| Card 010 ready | No. Contract 060 closes the transport decision, but card 016 implementation, current-version proof, and live same-turn acceptance remain |

## Omission

No watcher opt-in continues to use the exact current adapter command:

```text
--mcp-config {"mcpServers":{}}
--strict-mcp-config
```

The current adapter does not add `--bare`, Stop settings, an operation-private
skill root, hook events, or a bridge endpoint. See
[`claude_code_command.rs`](../../crates/swallowtail-adapter-claude-agent/src/claude_code_command.rs)
and the exact omission fixture in the adapter tests.

## Non-claims

- No current Claude version or version range is qualified for watcher bridge
  support.
- No provider-to-host HTTP or stdio bridge exists in the current runtime.
- No undocumented binary strings or internal `BRIDGE` environment names are
  treated as supported transport or authority.
- No claim is made that Claude-native `background_tasks`, MCP tasks, Bash
  background work, or subagents are Contract 059 watchers.
- No live provider turn, same-turn Stop re-entry, listener bind, helper, login,
  credential use, or production route change was performed.
- Contract 060 promotes only the future provider-neutral bridge boundary. No
  route, feature, compatibility, or release claim is promoted by this record.

## References

- [Contract 010 — Execution Host Services and Inputs](../contracts/010-execution-host-services-and-inputs.md)
- [Contract 041 — Input Callback and Provider Tool Admission](../contracts/041-input-callback-and-provider-tool-admission.md)
- [Contract 044 — Observable Agent Activity and Disclosure](../contracts/044-observable-agent-activity-and-disclosure.md)
- [Contract 059 — Operation-Scoped Process Watchers](../contracts/059-operation-scoped-process-watchers.md)
- [Contract 060 — Operation-Scoped Watcher HTTP Bridge](../contracts/060-operation-scoped-watcher-http-bridge.md)
- [Research 257 — Claude Code Watcher Seam Evidence](257-claude-code-watcher-seam-evidence.md)
- [g05.003 card 015](../roadmaps/g05/batch-cards/015-claude-code-watcher-bridge-transport-evidence.md)
- [g05.003 card 010](../roadmaps/g05/batch-cards/010-claude-code-watcher-bridge.md)
