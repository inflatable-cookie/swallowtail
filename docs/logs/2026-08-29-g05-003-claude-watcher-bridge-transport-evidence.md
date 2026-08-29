# 2026-08-29 g05.003 Claude Watcher Bridge Transport Evidence

Status: complete; evidence stop
Owner: Tom
Updated: 2026-08-30
Milestone: g05.003 stopped after evidence
Card: 015 complete; card 010 remains planned
Research: 260
Merged: PR 119 at `c36e11ad45415a8bf6e63678a948e58e73b5da3d`
Branch: `t3code/review-claude-watcher-bridge`
Worktree: `/Users/tom/.t3/worktrees/swallowtail/t3code-e9687bae`
Base: `71ea21113ff01986fd01f0dca66f29e7cfdbf644` (`origin/main` at dispatch)
Planning base ancestor: `eb014a9b72fb06b55d967a952ba618952991b5fa`

## Result

The installed Claude Code is exact native `2.1.251` on `darwin-arm64`. Its
current help and official docs expose the Research 257 provider-side candidate:
`--bare`, private `--mcp-config` plus `--strict-mcp-config`, session
`--settings` with `hooks.Stop`, operation-private `--add-dir` skills, and
`--include-hook-events`.

The candidate does not close the transport lane. Swallowtail has a host-owned
watcher registry and joined ordinary process supervision, but no MCP listener,
inbound bridge, or operation-private IPC handoff to the in-process
`WatcherHostService`. `ServingEndpointService` publishes an endpoint already
observed from an owned child; it does not bind. The sign-in loopback port is
purpose-limited.

HTTP is the smallest future carrier but requires a new operation-scoped host
listener/bridge contract with exact host, turn, operation, authentication, and
joined-cleanup semantics. Stdio requires a provider-launched helper and a host
IPC handoff that does not exist. SSE is deprecated, WebSocket has the same
listener gap, and `claude mcp serve` runs in the reverse direction.

Card 010 is not ready. No production MCP server, listener, helper, hook,
skill, route, contract, architecture, or compatibility claim was added.

## Evidence boundary

Prompt-free local evidence used `env -i`, a throwaway `HOME`, no credentials,
no login, and no provider prompt. The composed 2.1.251 command reached input
validation and returned the expected missing-input error. It did not connect to
an MCP endpoint, run a Stop hook, send a model request, or prove same-turn
re-entry. Research 257's `2.1.220..=2.1.241` same-turn candidate therefore does
not extend to a current qualified segment.

The installed binary SHA-256 is
`625869b01e0050f260b2980fac248fd9cef9e462612bded4ec9d3d49ff8969a5`. The
complete artifact, registry-integrity, and official-doc digests are in
[Research 260](../research/260-claude-code-watcher-bridge-transport.md).

## Validation

Passed on this worker branch:

```text
effigy validate:focused swallowtail-adapter-claude-agent
effigy qa:docs
effigy qa:northstar
git diff --check
```

No broad test suite or live provider probe was run. The inherited doctor
baseline remains unchanged: 384 god-file findings, one generated-in-source
warning, and the pre-existing graph repair noted by the readiness assessment.

## Continuation

Do not start card 010. A future planning lane must first decide whether to
promote a host-owned operation-scoped bridge contract, then separately
implement and live-prove it under explicit provider-work authorization.

The orchestrator fast-forwarded `main` to the exact reviewed PR head. The g05
pointer now names the operator planning checkpoint; no implementation card is
ready.
