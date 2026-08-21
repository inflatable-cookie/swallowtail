# 181 Pi SDK Sidecar Route Qualification

Status: accepted
Owner: Tom
Date: 2026-08-21

## Question

Can Pi's official SDK support a fuller Swallowtail route, including safe
persistent-session load and resume, without waiting for the missing RPC cwd
attachment field?

## Method

- inspected official `v0.84.2` SDK documentation and public exports
- inspected `createAgentSession`, `AgentSessionRuntime`, session services,
  resource loading, settings, and RPC implementations
- compared the SDK seam with Research 053 and 180 and Contracts 017, 019, 023,
  and 029
- used no prompt, account, credential, package install, or provider call

## Why RPC Existed

Research 022 selected Pi RPC as the smallest maintained, language-neutral
proof for a Rust library. It supplied an upstream-owned strict-LF JSONL wire,
process isolation, provider/model selection, scheduling, tools, and event
streaming without making Swallowtail host JavaScript.

That choice did not establish RPC as Pi's complete or permanent interface.
Pi's own guidance prefers the SDK for same-Node-process embedding, direct
state access, custom tools, and full control, and prefers RPC for cross-language
integration and process isolation.

## SDK Surface

The public SDK exports the same runtime used by Pi's interactive, print, and
RPC modes. At `0.84.2` it provides:

- `createAgentSession` and `AgentSessionRuntime`
- new-session, switch-session, fork, and import lifecycle operations
- prompt, steer, follow-up, event subscription, abort, and disposal
- model and thinking control, typed messages, tree navigation, and compaction
- explicit tool, `ResourceLoader`, `SettingsManager`, `SessionManager`, and
  model-runtime construction

The attachment seam missing from RPC is public in the runtime:
`switchSession(sessionPath, { cwdOverride })` opens the stored session under
the caller-supplied effective cwd. `runtime.cwd` then exposes that effective
binding. Swallowtail can therefore enforce the Contract 017 resource gate
without parsing or rewriting Pi's session file.

The SDK also exposes typed session messages. A load path can project replay
from the public SDK surface after attachment rather than treating Pi's JSONL
storage as a Swallowtail protocol.

## Boundary Decision

The official SDK is TypeScript and requires Node `>=22.19.0`; Swallowtail is
Rust. A full SDK-backed route therefore needs a Node sidecar. It is not
SDK-native under Contract 019 and does not remove process lifecycle or wire
responsibilities.

The selected first boundary is:

- route `pi.sdk-sidecar`
- driver `swallowtail.pi.sdk-sidecar`
- strict correlated LF-JSON wire `swallowtail-pi-sdk-jsonl-v1`
- source-tagged, application-local sidecar entry point
- exact `@earendil-works/pi-coding-agent@0.84.2`
- exact application-approved Node runtime satisfying `>=22.19.0`
- explicit provider, model, tools, resource, credential, and cwd inputs

The application provisions the exact Node runtime, sidecar entry point, and
SDK package through a host-approved launch recipe. Swallowtail does not run
`npm install`, discover an ambient package, or create a separate published
sidecar package.

## Initial Capability Posture

The first proof should preserve the existing Pi route's useful surface before
adding more SDK-only controls:

- explicit provider and model selection
- read-only `read`, `grep`, `find`, and `ls` tools
- prompt, steer, follow-up, ordered events, abort, and joined close
- model catalogue through the explicitly constructed SDK runtime
- persistent new, load-with-replay, and replay-free resume

All ambient extensions, skills, prompt templates, context files, themes,
settings, aliases, catalogue refresh, update checks, retries, and fallback are
disabled. Settings are in memory. The route remains `AmbientHost`; a sidecar
does not prove filesystem or network containment.

New session, load, and resume remain distinct. Load and resume pass the
host-leased cwd as `cwdOverride`, compare the effective runtime cwd before
ready, and fail closed on mismatch. Load projects typed messages before ready;
resume does not replay. Neither path parses, copies, rewrites, or trusts the
stored session cwd, and neither injects a hidden context-restoration prompt.

Interrupted turns, pending callbacks, and cross-process active-operation
recovery remain unsupported until separate provider evidence exists.

## Compatibility And Coexistence

The SDK sidecar gets a qualified-only one-point claim at `0.84.2`. It does not
inherit `pi.rpc.package-window-2` or its unverified-newer posture. The SDK
package, Node runtime, sidecar source, and custom wire are separate version
axes.

The existing `pi.rpc` route stays production while the SDK route is proved.
The final acceptance card compares deployment, lifecycle, feature, and
stability evidence. It may retain both routes or deprecate RPC explicitly; it
must not silently substitute one route for the other.

## Promotion

- Contract 019 now governs foreign-language SDK sidecars
- Contract 029 names the separate Pi SDK sidecar claim posture
- g04.033 and cards 089-092 carry implementation and acceptance
- architecture remains unchanged until code realizes the route
- the RPC attachment gate in Research 180 remains true for `pi.rpc`

## Sources

- [Pi `v0.84.2` SDK guide](https://github.com/earendil-works/pi/blob/v0.84.2/packages/coding-agent/docs/sdk.md)
- [Pi `v0.84.2` package identity](https://github.com/earendil-works/pi/blob/v0.84.2/packages/coding-agent/package.json)
- [Pi `v0.84.2` public exports](https://github.com/earendil-works/pi/blob/v0.84.2/packages/coding-agent/src/index.ts)
- [Pi `v0.84.2` SDK construction](https://github.com/earendil-works/pi/blob/v0.84.2/packages/coding-agent/src/core/sdk.ts)
- [Pi `v0.84.2` session runtime](https://github.com/earendil-works/pi/blob/v0.84.2/packages/coding-agent/src/core/agent-session-runtime.ts)
- [Pi `v0.84.2` resource loader](https://github.com/earendil-works/pi/blob/v0.84.2/packages/coding-agent/src/core/resource-loader.ts)
- [Pi `v0.84.2` RPC handler](https://github.com/earendil-works/pi/blob/v0.84.2/packages/coding-agent/src/modes/rpc/rpc-mode.ts)
