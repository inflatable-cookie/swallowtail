---
title: g05.035 Card 132 Desktop Claude SDK registered-tool real-route gate packet
kind: desktop-probe-packet
status: ready
owner: Tom
created: 2026-09-07
updated: 2026-09-08
card: docs/roadmaps/g05/batch-cards/132-claude-sdk-registered-tool-real-route-gate.md
---

# Card 132 Claude SDK Registered-Tool Real-Route Gate Packet

Swallowtail supplies the frozen tuple and the evidence oracle. Desktop is the
sole integration/test owner and runs one bounded real route under its existing
isolated-testing authorization. Swallowtail does not run Claude, read
credentials, or qualify the route from this packet.

The Contract 061 row stays `Unqualified / real_route_gate_pending` with safe
reason `callable seam present; live gate pending` until Chatterbox receives a
passing capsule. No capsule changes a claim, matrix, tag, release, or source
consumer by itself.

## Frozen tuple

Run exactly this tuple. A different value is a typed setup failure, not a
nearby pass.

| Axis | Exact value | Authority |
| --- | --- | --- |
| route | `claude-agent.sdk` | Card 132; `docs/guides/claude-agent-sdk-prepared-integration.md` |
| SDK package | `@anthropic-ai/claude-agent-sdk@0.3.259` | `tests/fixtures/claude-agent-sdk-0.3.259/identity.json` |
| SDK tarball SHA-256 | `0c5740e44a536ab6fd32f2a7de0d508b75d34782ebc219b87aa8d834449a3f7e` | same frozen identity fixture |
| native Claude | `2.1.259` | SDK `manifest.json` identity, frozen in the same fixture |
| native manifest commit | `9b549c8d1c72e407ea9d3af3b9d5e50da794ec4d` | same frozen identity fixture |
| Node | exact `22.23.2` | prepared-integration guide and route freeze ledger |
| Swallowtail sidecar | behavior `claude-agent.sdk-v1`; source commit `6f21985f287fd7fa5291fd100a715861b5759526`; source SHA-256 `bae37e7306bd123968922bc23fe4cc7bf52766579f37da9b026592f4ae5d3f9e` | sidecar source and route guide |
| Swallowtail sidecar wire | `swallowtail-claude-agent-sdk-jsonl-v1` | sidecar source |
| registered-tool carrier revision | `swallowtail-claude-agent-sdk-registered-tool-mcp-v1` | adapter registered-tool version module |
| Contract 063 selection protocol | `swallowtail.registered-tool.conformance-2026-09-07` | runtime registered-tool readiness module |
| courier argv wire tag | `swallowtail-registered-tool-mcp-v1` | runtime attachment and host-local wire modules |
| JSON-RPC | `2.0` | host-local wire module |
| MCP handshake | `2025-11-25` | SDK `0.3.259` MCP fixture and host-local wire module |
| transport | `private-loopback-http` | Contract 063 Mediated Stdio Proxy Attachment |
| attachment | `mediated-stdio-proxy` | Contract 063 and Card 125 |
| MCP server | `swallowtail-registered-tools` | adapter carrier and host-local wire modules |
| model | `claude-sonnet-5` | Card 125 mounted route fixture |
| permission mode | `default` | sidecar route default; no auto-approving mode |
| session persistence | `false` / omitted | sidecar route default; no resume or listing on this bound selection |
| SDK `strictMcpConfig` | `true` | sidecar open construction |
| SDK `settingSources` | `[]` | sidecar open construction |
| SDK `allowedTools` | omitted, not `[]` and not `null` | sidecar open construction |
| selected tools | exactly one MCP tool: `mcp__swallowtail-registered-tools__desktop_reconcile` | Card 125 carrier mapping and mounted fixture |

The package digest is the only frozen SDK artifact identity. Swallowtail has no
frozen Node binary digest; Desktop must attest the exact Node version above and
must not substitute a path, alias, or newer runtime. The native platform is a
capsule field: Desktop records the selected platform label and the matching
`2.1.259` platform checksum from the frozen identity fixture, never a binary
path.

The one disposable registration is the Card 125 fixture-shaped MCP
registration:

| Field | Exact value |
| --- | --- |
| server id | `desktop.registered-tools` |
| server revision | `2026-09-07.1` |
| namespaced tool id | `desktop/reconcile` |
| execution kind | `mcp` |
| input schema namespace/media type/dialect/revision | `desktop.registered-tools.schema` / `application/json` / `json-schema-2020-12` / `1` |
| input schema digest | `sha256:input` |
| output schema digest | `sha256:output` |
| effect posture | `mutating` |
| retry posture | `consumer-retryable`; no automatic retry is permitted by this gate |
| implementation | disposable, bounded, no filesystem/network authority; return one fixed `{"ok":true}` result |

The provider-facing tool name is derived, not aliased:
`mcp__swallowtail-registered-tools__desktop_reconcile`. The provider must see
one declared internal courier with the fixed wire tag and one opaque rendezvous
path. It must not see the endpoint, bearer, lease generation, transport
generation, raw executable path, or environment values.

## Commands

These are the exact Swallowtail-side commands. They are provider-free.

Build the feature-gated reference courier from the checkout whose source is
being tested:

```sh
cargo build --offline --locked \
  -p swallowtail-host-local \
  --features mediated-stdio-proxy \
  --bin swallowtail-registered-tool-courier
```

Run the mounted courier/kernel conformance path. This does not start Claude,
Node, or a provider session:

```sh
effigy validate:card116-mediated-stdio
```

Desktop then invokes its existing isolated host runner through the prepared
`claude-agent.sdk` facade with the frozen tuple above and writes one capsule.
This repository contains no live Claude runner, so the consumer-owned command
name is deliberately not invented here. The runner command is valid only when
Desktop binds its own existing executable/entry point to this exact request:

```text
route=claude-agent.sdk
sdk=@anthropic-ai/claude-agent-sdk@0.3.259
sdk_tarball_sha256=0c5740e44a536ab6fd32f2a7de0d508b75d34782ebc219b87aa8d834449a3f7e
native=2.1.259
node=22.23.2
sidecar_wire=swallowtail-claude-agent-sdk-jsonl-v1
carrier=swallowtail-claude-agent-sdk-registered-tool-mcp-v1
selection_protocol=swallowtail.registered-tool.conformance-2026-09-07
mcp_protocol=2025-11-25
transport=private-loopback-http
attachment=mediated-stdio-proxy
server=swallowtail-registered-tools
tool=desktop/reconcile
strictMcpConfig=true
settingSources=[]
allowedTools=omitted
model=claude-sonnet-5
permissionMode=default
persistSession=false
output=/path/to/claude-agent-sdk-registered-tool.capsule.json
```

The Desktop runner must use a fresh disposable workspace, a fresh session,
turn, attempt, lease, and registration; one provider turn; and the exact
disposable tool above. It must record the exact prompt as a bounded digest,
not copy provider content into the capsule. If Desktop cannot name the
existing isolated runner command or cannot bind every request field above, it
must stop and return that typed setup defect. It must not replace the command
with a direct Claude invocation, a fixture, a consumer-declared MCP server, or
an ambient configuration path.

## Required capsule

Return one redacted JSON capsule. All opaque identities are retained as
`sha256:<64 lowercase hex>` digests of their exact values. Raw credentials,
tokens, endpoints, bearer material, host paths, environment values, account
identity, provider content, and arbitrary diagnostics are forbidden.

Required top-level fields:

- `route`, `verdict` (`pass` or `typed_failure`), `failure_code`, and bounded
  `failure_stage`;
- `tuple`, repeating every frozen value above plus `native_platform`, the
  matching native artifact SHA-256, the loaded SDK digest, and exact observed
  Node/native/SDK versions;
- `registration`, including server id/revision, namespaced and provider tool
  identities, kind, schema revisions/digests, effect, retry posture, and
  selected protocol/attachment/transport;
- `identities`, including digests for execution host, configured instance,
  session, turn, attempt, lease generation, transport generation, call, and
  result;
- `prompt_sha256`, `prompt_bytes`, `model`, and `turn_completed`;
- `open`, including readiness, `mcp_status`, one courier declaration, and
  proof that the courier authenticated before readiness;
- `frames`, bounded and redacted, including the sidecar open/terminal/close
  records and MCP `initialize`, `notifications/initialized`, `tools/list`,
  `tools/call`, and their correlated responses;
- `permission`, including exactly one `canUseTool` observation for the real
  call, exact provider tool name, `allow`, and whether the input was unchanged;
- `call`, including `BeforeDispatch`, dispatcher invocation, result
  correlation, result schema digest, bounded fixed result, and execution
  disposition;
- `contract063_controls`, including `deny_path_proven`,
  `cancellation_path_proven`, `automatic_retry_observed`,
  `reconnect_or_respawn_observed`, and `stale_or_foreign_callback_rejected`;
- `cleanup`, including admission freeze, rendezvous unlink, courier exit/join,
  listener/task join, lease/resource release, and survivor absence; and
- `redaction`, with every forbidden-material flag false and `truncated`.

The capsule must distinguish the following states rather than collapsing them:

- `provider_tool_declared` is not `mcp_admitted`;
- `mcp_admitted` and `tools/list` are not a tool invocation;
- `canUseTool=allow` is not dispatcher execution;
- an executed error is not `not-executed`;
- `transport_lost` or unknown outcome never permits replay; and
- cleanup `joined` is not inferred from process exit or Drop.

## Contract 063 stop conditions

Stop before provider work when any identity, digest, version, protocol,
selection, server, tool, schema, model, Node, wire, or command value differs
from the frozen tuple. Stop when the required disposable registration,
freshness, or redaction cannot be proved.

Stop the gate immediately, preserve the capsule, and do not retry the same
attempt on any of these observations:

- authority leakage: endpoint, bearer, generation, credential, raw path, raw
  environment, or unrelated workspace/client content reaches the provider or
  capsule;
- ambient MCP or settings: any undeclared server/tool, non-empty settings
  source, managed/ambient server, consumer-declared server, plugin, hook,
  skill, or inherited credential/configuration path is loaded;
- lazy attach or wrong readiness: provider work starts before the courier has
  authenticated and negotiated MCP `2025-11-25`;
- automatic respawn, reconnect, retry, replay, or a second courier reads the
  one-shot rendezvous;
- `canUseTool` is bypassed, auto-approval is simulated, `allowedTools` is
  present, or the provider tool identity is altered;
- namespace, server, registration revision, schema digest, call id, lease
  generation, transport generation, session, turn, attempt, or result identity
  is ambiguous, foreign, stale, duplicated, or reused;
- a result reaches the provider or consumer after cancel, terminal, close, or
  revocation, or an executed/unknown result is relabelled not-executed; or
- cleanup is incomplete: rendezvous remains, courier/listener/task does not
  join, the lease/resource is released before join, a survivor is observed, or
  cleanup is reported clean without the required evidence.

These are Contract 063 falsifiers, not provider limitations. The packet never
turns a stop into a pass by inference.

## Pass and typed-failure mapping

`pass` requires all of the following in one capsule:

1. every observed tuple value equals the frozen tuple, including the package
   digest, native manifest/platform identity, exact Node version, sidecar wire,
   carrier revision, selection protocol, MCP protocol, transport, attachment,
   strict settings, omission of `allowedTools`, model, and one-tool selection;
2. one fresh provider-spawned courier authenticates before readiness and
   completes MCP initialize/list;
3. exactly one real `tools/call` for the exact provider tool name reaches the
   kernel after `canUseTool=allow` and `BeforeDispatch`, and one correlated
   result returns with the fixed output schema digest and executed disposition;
4. the Contract 063 allow, deny, cancellation, stale/foreign-callback, and
   cleanup controls are represented without a stop condition; and
5. all close, join, unlink, freeze, release, and redaction fields pass.

Only that exact tuple and these two cells may be qualified after Chatterbox
reviews the capsule:

- `claude-agent.sdk` → `registered_tools`
- `claude-agent.sdk` → `consumer_tool_exchange`

The route's other cells, other models, other SDK/native/Node points, other
carriers, consumer-declared MCP, selected skills, and generic cross-route
claims remain unchanged.

Every other outcome is `typed_failure` or an explicit stop and moves nothing:

| Outcome | Capsule disposition | Qualification disposition |
| --- | --- | --- |
| tuple, digest, version, protocol, command, registration, or redaction mismatch | `typed_failure` with `failure_stage=setup` and exact safe code | retain `Unqualified / real_route_gate_pending`; no matrix change |
| provider/SDK/native/Node open rejection, auth/readiness failure, missing turn result, or provider rejection | `typed_failure` with bounded safe code and `execution_disposition` | no qualification; return evidence to Chatterbox; no automatic rerun |
| `consumer_denied` on the negative control without the required real allow path | `typed_failure` with `execution_disposition=not-executed` | no qualification |
| `transport_lost`, unknown outcome, cancellation, or deadline | `typed_failure` with `execution_disposition=unknown` or `not-executed` as observed | no replay, no qualification, no matrix change |
| stale, foreign, duplicate, late, or post-close callback/result | `typed_failure` and preserve the rejected correlation digest | treat as a Contract 063 stop; no qualification |
| authority leakage, ambient MCP, lazy attach, automatic retry/respawn, `canUseTool` bypass, ambiguous identity, or incomplete cleanup | stop capsule with the matching stop code | no qualification; return the defect to the producer/contract owner |

Do not reinterpret a typed failure as a provider limitation, do not retry a
mutating or unknown call, and do not reuse its session, attempt, lease,
rendezvous, or registration. Chatterbox decides whether a later producer card
or a fresh operator-authorized gate is needed.

## What to send back

Desktop returns the one redacted capsule or the named setup defect. Chatterbox
reviews it against Card 132, Contract 061, and Contract 063. Swallowtail does
not run the live gate and this packet authorizes no provider, credential,
machine, tag, release, or consumer-repository action.
