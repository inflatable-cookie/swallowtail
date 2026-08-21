# 2026-08-21 Pi Continuity Workarounds

Status: resolved
Owner: Tom
Lane: g04

## Context

Pi `0.84.2` can restore a session through its public SDK while overriding the
stored working directory. The RPC `switch_session` command does not expose that
override and `get_state` does not report the effective working directory.

Swallowtail therefore cannot claim safe Pi RPC load or resume. A persisted Pi
session may carry a different directory from the host-resolved working resource,
and attachment must fail closed before provider work starts.

Evidence:

- `docs/research/180-pi-rpc-session-attachment-gate-revalidation.md`
- `docs/research/053-pi-rpc-session-continuity-currentness-and-resource-binding-gate.md`
- `docs/contracts/017-provider-owned-session-load-replay-and-host-containment.md`

## Immediate Workaround

Keep `pi.rpc` fresh-session only. A consumer that needs continuity may persist
its own task state, open a new Pi session against the same exact host lease, and
send a bounded handoff as an explicit first prompt.

This is context reconstruction, not provider-session resume. It must not claim
the old Pi session identity, hidden context, interrupted turn, pending callback,
or exact replay.

## SDK Route

If native Pi session continuity is required before RPC changes, introduce a
separate pinned TypeScript sidecar over Pi's public `AgentSessionRuntime` SDK.
This would be an SDK-backed route with a sidecar transport, not an RPC repair.
The sidecar would:

- accept the opaque provider session id and host-resolved expected directory
- resolve that id uniquely inside the application-approved session directory,
  reject canonical path escape, then call
  `switchSession(sessionPath, { cwdOverride: expectedDirectory })` privately
- return the effective `runtime.cwd` and provider session identity
- allow the Rust boundary to compare both before declaring attachment ready
- expose only the operations needed by Swallowtail over a typed stdio protocol
- avoid parsing, rewriting, copying, or trusting the session file directly

This is a new route and lifecycle boundary, not a silent repair to `pi.rpc`.
It needs an architecture/contract promotion and exact-version qualification.

## Route Boundary Question

RPC was selected as a bounded language-neutral harness proof. It remains the
smaller integration because Pi owns its wire, command loop, runtime assembly,
configuration loading, and process entry point.

The SDK is TypeScript rather than Rust. Swallowtail therefore cannot embed it
as an SDK-native Rust route. A TypeScript sidecar must own runtime assembly,
configuration suppression, authentication, event projection, cancellation,
cleanup, and its own typed wire back to the Rust adapter.

The implementation proof retains two possible final dispositions:

1. retain `pi.rpc` as the simple fresh-session route and add an SDK-backed route
   for fuller Pi capabilities
2. qualify the SDK-backed route first, then deprecate `pi.rpc` if it proves a
   strict capability superset without weaker lifecycle or configuration truth

Do not call the SDK-backed route `SDK-native`: Contract 019 reserves that term
for a language-native package linked into the adapter process. Its accurate
identity is a language sidecar backed by Pi's official SDK.

## Rejected Shortcuts

- patching the installed Pi RPC implementation
- deep-importing Pi internals
- rewriting the persisted session directory
- switching first and checking the directory afterward
- treating a fresh session or transcript replay as resume

## Promotion

- Research 181 records the selected SDK-backed sidecar boundary.
- Contracts 019 and 029 govern sidecar and compatibility identity.
- g04.033 and cards 089-092 carry implementation and final RPC disposition.
- Consumer-owned fresh-session reconstruction remains the temporary fallback
  until that route lands.

## Resolution

g04.033 landed the SDK sidecar route exactly as the SDK Route section
proposed: `pi.sdk-sidecar` carries only the provider session id, resolves it
uniquely inside the application-approved session directory, switches with the
host-leased `cwdOverride`, and compares the effective cwd and session identity
before readiness. It realizes persistent new, load-with-replay, and replay-free
resume under Contract 017. The RPC attachment gate from Research 180 stands:
`pi.rpc` stays fresh-only. The recorded disposition is option 1 — retain
`pi.rpc` as the simple fresh-session route alongside the SDK route — because
the SDK route is a continuity superset but not an operational superset (it
requires the provisioned Node runtime, source-tagged sidecar, and exact SDK
package over a Swallowtail-owned private wire).
