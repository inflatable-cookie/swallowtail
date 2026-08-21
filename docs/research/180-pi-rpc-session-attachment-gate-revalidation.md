# 180 Pi RPC Session Attachment Gate Revalidation

Status: promoted
Owner: Tom
Date: 2026-08-21

## Question

Does current Pi RPC `0.84.2` now let Swallowtail attach a persisted session to
the exact caller-bound working resource required by Contract 017?

## Method

- confirmed official npm `@earendil-works/pi-coding-agent` latest remains
  `0.84.2`
- confirmed host `pi --version` remains `0.83.0`
- inspected tagged `v0.84.2` RPC command types and handling,
  `AgentSessionRuntime`, and cwd validation
- compared the result with Research 053 and Contracts 017 and 038
- used no prompt, account, provider, credential, install, or host mutation

## Evidence

Pi's public runtime now has a useful partial seam:

- `AgentSessionRuntime.switchSession` accepts an optional `cwdOverride`
- it passes that override into `SessionManager.open`
- the replacement runtime is created from the resulting effective cwd

The public RPC wire does not expose that seam:

- RPC `switch_session` still carries only `sessionPath`
- the RPC handler calls `runtimeHost.switchSession(command.sessionPath)` with
  no override
- the correlated response reports only `cancelled`
- `get_state` still omits effective cwd

The stored session cwd can therefore replace the host-leased resource before
Swallowtail can prove agreement. Direct session-file parsing remains excluded
by Research 053.

## Result

The Contract 017 gate remains. Pi load and replay-free resume cannot be
implemented honestly on the current RPC wire.

The smallest compatible upstream change is:

1. add an optional expected cwd to RPC `switch_session`
2. pass it as `cwdOverride`
3. return the effective cwd in the correlated response or `get_state`

A Swallowtail-owned TypeScript sidecar over `AgentSessionRuntime` is a larger
new-driver/facade decision and must not be inferred from this evidence.

## Promotion

- Pi leaves passive backlog and becomes the immediate post-g04.024 operator
  decision
- Contracts 017 and 038 remain unchanged
- production Pi load/resume claims remain `No`
- no implementation card is ready until the operator chooses upstream RPC work
  or a separate sidecar boundary

## Sources

- [Pi `v0.84.2` RPC types](https://github.com/earendil-works/pi/blob/v0.84.2/packages/coding-agent/src/modes/rpc/rpc-types.ts)
- [Pi `v0.84.2` RPC handler](https://github.com/earendil-works/pi/blob/v0.84.2/packages/coding-agent/src/modes/rpc/rpc-mode.ts)
- [Pi `v0.84.2` session runtime](https://github.com/earendil-works/pi/blob/v0.84.2/packages/coding-agent/src/core/agent-session-runtime.ts)
- [Pi `v0.84.2` cwd validation](https://github.com/earendil-works/pi/blob/v0.84.2/packages/coding-agent/src/core/session-cwd.ts)
- npm `@earendil-works/pi-coding-agent@0.84.2`
