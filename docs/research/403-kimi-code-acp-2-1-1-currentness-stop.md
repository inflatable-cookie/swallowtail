# Kimi Code ACP 2.1.1 Currentness Stop

Observed 2026-10-08 for `kimi-code.acp` only. This checkpoint changes no
production claim.

## Finding

Official npm `latest` and the latest GitHub release are both
`@moonshot-ai/kimi-code@2.1.1`. Thirteen stable releases follow the existing
`0.38.0` ceiling, including the same-package major reset at `2.0.0`. The exact
published sequence, npm integrity and tarball digests, GitHub tag objects,
commits, and source trees are frozen in
[`identity.json`](../../crates/swallowtail-adapter-kimi/tests/fixtures/kimi-code-2.1.1-acp/identity.json).
The installed CLI remains `0.34.0`; no artifact was installed or executed.

The complete npm package trees from `0.43.0` through `2.1.1` are in
[`dist-inventory.json`](../../crates/swallowtail-adapter-kimi/tests/fixtures/kimi-code-2.1.1-acp/dist-inventory.json).
Every file has a SHA-256 digest. Adjacent added, removed, and changed file
sets are recorded with their digests. The selected ACP source-region ledger
and all changed bundle source regions are in
[`protocol.json`](../../crates/swallowtail-adapter-kimi/tests/fixtures/kimi-code-2.1.1-acp/protocol.json).
Older hops and the existing authority boundary remain covered by
[Research 270](./270-kimi-code-0-39-1-identity.md) and
[Research 325](./325-kimi-code-0-43-0-installed-identity.md).

| Published hop | npm files added / removed / changed / unchanged | ACP server change |
| --- | ---: | --- |
| `0.43.0` → `0.43.1` | 75 / 73 / 4 / 466 | None across 25 source regions |
| `0.43.1` → `2.0.0` | 69 / 73 / 3 / 469 | `slash.ts` skips skills with explicit scopes when listing available commands |
| `2.0.0` → `2.0.1` | 0 / 0 / 3 / 538 | Generated initializer name only |
| `2.0.1` → `2.0.2` | 69 / 69 / 3 / 469 | Generated initializer names only |
| `2.0.2` → `2.1.0` | 69 / 69 / 3 / 469 | None across 25 source regions |
| `2.1.0` → `2.1.1` | 0 / 0 / 2 / 539 | None across 25 source regions |

The package file sets include the full `dist/main.mjs`, worker, web asset, and
metadata changes. Source regions outside `packages/acp-server` are also
listed with exact hashes; they are Kimi implementation and dependency changes
without a Swallowtail ACP mapping. They do not transfer a public capability.
At the major reset, the available-command discovery filter is not observed by
this adapter: `AvailableCommands` produces no activity.

## Process authority

The tagged `acpTerminalRunner.ts` Git blob is
`9016d48b643f35b263449d98dee25597a9a24d30` at `0.43.1` and `2.1.1`; its
bundled source region is byte-identical across all seven inventoried npm
points. Its fallback condition is
`!this.connection.terminalEnabled || !isBashToolInvocation(args, options)`.
Either a disabled terminal capability or an invocation that does not match the
`isBashToolInvocation` argument/environment shape takes the provider-local
`this.local.spawn(...)` path. Only a matching call with terminal enabled uses
the ACP client's `terminal/create` callback.

Swallowtail advertises `terminal: false` and `auth.terminal: false`. Its
terminal request dispatcher has no host callback route. Existing fake ACP
fixtures confirm those exact capabilities and the unsupported request
boundary at the adapter layer; they do not execute the Kimi provider-local
runner or prove its child lifecycle. Contract 015 assigns terminal callbacks
to the execution host and leaves terminal support optional. Neither the
adapter nor runtime mediates the nested process. This is the same authority
stop first recorded in Research 270; the current artifact confirms it persists
through the major reset and `2.1.1`.

## Shipped-control investigation

The source and documentation below are pinned to official GitHub commit
[`f67e6398fb3210ad8ace970e2dfd5bcc984ed61f`](https://github.com/MoonshotAI/kimi-code/tree/f67e6398fb3210ad8ace970e2dfd5bcc984ed61f),
the `2.1.1` identity already frozen above. Their Git blob IDs and both local
spawn branches are mutation-checked in `protocol.json`.

| Shipped surface | Exact effect | Covers both local-spawn branches? | Disposition |
| --- | --- | --- | --- |
| ACP `clientCapabilities.terminal` | The server binds this initialize capability to `terminalEnabled`. When true, only the `isBashToolInvocation` argument/environment shape calls `terminal/create`; when false, all invocations take `local.spawn`. | No | It is a branch selector. Setting it true leaves any call outside that shape on local spawn; the route currently advertises false. |
| `[tools].enabled` / `[tools].disabled` | Global persistent configuration shapes the tools exposed to agents and is checked before tool execution. | No | It changes agent tool availability, not the injected `IHostProcessService`; it is not a per-session process mediation control. |
| `[permission.rules]` and dangerous-command guard | Persistent rules approve, ask about, or deny named agent tool calls. | No | Permission policy is not a host process service and does not mediate every provider-local spawn. |
| `PreToolUse` hooks | A configured local script may block a matching tool call. | No | The hook is a tool-call gate, not process mediation; it requires persistent local configuration and does not establish host, principal, environment, or working-resource binding. |
| ACP session config options | The shipped options are model, thinking, and mode. | No | There is no process or terminal option in the session option surface. |
| `kimi acp` command and config location | The documented ACP invocation is `kimi acp`; configuration is read from `~/.kimi-code/config.toml`. `KIMI_CODE_HOME` relocates that persistent data/config directory. | No | No documented per-session setting disables or mediates both branches. Relocating provider state is outside this task's authority and would not change the spawn branch. |

No shipped per-session control was identified that disables or host-mediates
both selected local-spawn branches while preserving the selected route's
permission and cancellation policy and its host, principal, environment, and
working-resource boundaries. This is a bounded source/doc finding, not a
claim that no undocumented or future provider mechanism can exist. Q004's
local-server AmbientHost evidence remains a separate route and supplies no ACP
control.

The predicate named `isBashToolInvocation` checks exactly two arguments, a
first argument of `-c`, and environment entries `NO_COLOR=1` and `TERM=dumb`;
it does not inspect the executable name. The non-matching branch therefore
includes non-Bash invocations and any other call whose arguments or
environment differ.

## Host-terminal design for separate review

The execution host would need a typed, operation-scoped `HostTerminalService`
bound to the same prepared operation, principal, host, and working resource as
the ACP session. Its minimum interface is `spawn(request) -> handle`, with
handle methods `write_input`, `read_output(cursor, max_bytes)`, `terminate`,
`wait_until(deadline)`, and `release`. A request carries the active
session/turn/tool correlation, executable and argument vector, an explicit
environment delta, the canonical working-resource reference, deadline, and
output bound. The host supplies the principal and resource binding; provider
payloads cannot replace them. The host checks its existing permission policy
before spawn and rejects the request when mediation is unavailable. There is
no ambient local fallback.

The returned opaque handle would own stdin/stdout/stderr and expose only
`terminate`, `wait`, and `release` operations. Cancellation requests terminate
through that handle and await exit, output drain, and release before session
close reports joined cleanup. Permission denial starts no process. Environment
and working-directory validation remain host-owned and preserve the original
operation boundary.

This host API alone cannot intercept the current provider-local fallback:
Kimi 2.1.1 calls its own `IHostProcessService` for both branches described
above. The provider-facing equivalent could be an ACP extension with
`host/process/create`, `output`, `kill`, `wait_for_exit`, and `release`
callbacks, or a shipped fail-closed setting that routes all local process
requests to those callbacks. Either route must cover the terminal-disabled and
non-matching invocation branches; ACP `terminal/create` currently covers only
the recognized argument/environment shape. The host API and provider-facing
capability therefore need a separate Contract 015/API review. Keep
`terminal: false` until both paths can be selected safely; `terminal: true`
alone does not satisfy the design.

The future fake suite must prove both provider branches, permission denial,
environment/resource/principal binding, cancellation, termination, output
drain, and joined release against a host spy. In particular, the disabled
terminal and non-Bash cases must record no ambient spawn; cancel/stop must wait
for the owned fake handle to exit and join. Those tests cannot be written as
proof of the proposed API before the API and provider control exist. No fake
compatibility result or claim increase is asserted here.

## Disposition

Keep claim `kimi.acp.executable-window-5` at exact `0.28.1` and maintained
`0.29.0..=0.38.0` under `QualifiedOnly`. Preserve exact exclusions `0.39.0`
and `0.39.1`; do not add exclusions for points already rejected by posture.
Every published point above `0.38.0` through `2.1.1` remains incompatible.
There is no published stable 1.x line. Unpublished points remain gaps.

Tom's Contract 015 ruling `99b70b79-a36f-4d44-8108-ba118442b4e2` authorizes this
bounded investigation and requires disabled or host-mediated spawn for every
selected branch. No qualifying shipped control was identified, so the
0.38.0 ceiling and all existing points and exclusions remain unchanged. The
host-terminal design above is for separate review; this task implements no
runtime/API behavior, public operation, lifecycle, or capability. No provider
prompt, authenticated catalogue/session, credential, install, or host update
was used.

## Sources

- [Official npm package](https://www.npmjs.com/package/%40moonshot-ai/kimi-code)
- [Official 2.1.1 GitHub release](https://github.com/MoonshotAI/kimi-code/releases/tag/%40moonshot-ai%2Fkimi-code%402.1.1)
- [Kimi ACP reference at the frozen commit](https://github.com/MoonshotAI/kimi-code/blob/f67e6398fb3210ad8ace970e2dfd5bcc984ed61f/docs/en/reference/kimi-acp.md)
- [Kimi command reference at the frozen commit](https://github.com/MoonshotAI/kimi-code/blob/f67e6398fb3210ad8ace970e2dfd5bcc984ed61f/docs/en/reference/kimi-command.md)
- [Kimi configuration files at the frozen commit](https://github.com/MoonshotAI/kimi-code/blob/f67e6398fb3210ad8ace970e2dfd5bcc984ed61f/docs/en/configuration/config-files.md)
- [Kimi configuration overrides at the frozen commit](https://github.com/MoonshotAI/kimi-code/blob/f67e6398fb3210ad8ace970e2dfd5bcc984ed61f/docs/en/configuration/overrides.md)
- [Kimi hooks at the frozen commit](https://github.com/MoonshotAI/kimi-code/blob/f67e6398fb3210ad8ace970e2dfd5bcc984ed61f/docs/en/customization/hooks.md)
- [Terminal runner at the frozen commit](https://github.com/MoonshotAI/kimi-code/blob/f67e6398fb3210ad8ace970e2dfd5bcc984ed61f/packages/acp-server/src/acp-terminal/acpTerminalRunner.ts)
- [Contract 015: ACP v1 Negotiation and Client Callbacks](../knowledge/contracts/015-acp-v1-negotiation-and-client-callbacks.md)
- [Contract 029: Interface Version Qualification and Compatibility](../knowledge/contracts/029-interface-version-qualification-and-compatibility.md)
