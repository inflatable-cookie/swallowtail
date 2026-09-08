---
title: g05.035 Card 132 Desktop Claude SDK registered-tool real-route gate packet
kind: desktop-probe-packet
status: complete
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
| SDK package | `@anthropic-ai/claude-agent-sdk@0.3.259` | `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-0.3.259/identity.json` |
| SDK tarball SHA-256 | `0c5740e44a536ab6fd32f2a7de0d508b75d34782ebc219b87aa8d834449a3f7e` | same frozen identity fixture |
| native Claude | `2.1.259` | SDK `manifest.json` identity, frozen in `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-0.3.259/identity.json` |
| native manifest commit | `9b549c8d1c72e407ea9d3af3b9d5e50da794ec4d` | same frozen identity fixture |
| Node | exact `22.23.2` | `docs/guides/claude-agent-sdk-prepared-integration.md:144`; `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-v1/protocol.json` |
| Swallowtail sidecar | behavior `claude-agent.sdk-v1`; source commit `6f21985f287fd7fa5291fd100a715861b5759526`; source SHA-256 `bae37e7306bd123968922bc23fe4cc7bf52766579f37da9b026592f4ae5d3f9e` | `crates/swallowtail-adapter-claude-agent/sidecar/claude-agent-sdk-sidecar.mjs`; source commit and SHA are frozen observations |
| Swallowtail sidecar source tag | exact `swallowtail-claude-agent-sdk-sidecar@0.4.4` | `crates/swallowtail-adapter-claude-agent/src/sdk/asset.rs`; workspace version in `Cargo.toml`; source-tag prefix in `crates/swallowtail-adapter-claude-agent/tests/fixtures/claude-agent-sdk-v1/protocol.json` |
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
path. The sidecar source tag is a separate opaque compatibility axis and must
match exactly; the source commit and source SHA do not substitute for it.

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
The composition reference for that facade is the compile-tested
[`admitted_claude_agent_sdk`
example](../../crates/swallowtail-adapter-claude-agent/examples/admitted_claude_agent_sdk.rs):
construct the admitted `claude-agent.sdk` record from the three opaque
host-owned references, lift it with `ClaudeAgentSdkSessionPreparation::from_admitted`,
select the mediated-stdio proxy attachment with the courier proxy recipe over
the private-loopback carrier, bind the `RegisteredToolPreparation`, and open.
It spends no live route.
This repository contains no live Claude runner, so the consumer-owned command
name is deliberately not invented here. The runner command is valid only when
Desktop binds its own existing executable/entry point to this exact request:

```text
route=claude-agent.sdk
sdk=@anthropic-ai/claude-agent-sdk@0.3.259
sdk_tarball_sha256=0c5740e44a536ab6fd32f2a7de0d508b75d34782ebc219b87aa8d834449a3f7e
native=2.1.259
node=22.23.2
sidecar_source_tag=swallowtail-claude-agent-sdk-sidecar@0.4.4
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
primary_attempt=one_allow_and_one_real_tools_call
control_attempts=deny,cancellation,stale_or_foreign_callback
output=/path/to/claude-agent-sdk-registered-tool.capsule.json
```

The Desktop runner must use a fresh disposable workspace, session, turn,
attempt, lease, and registration for each attempt. The packet has one primary
allow attempt, with exactly one real `tools/call` for the disposable tool, and
three separate fresh live control attempts: supported deny, cancellation while
a call or permission is pending, and stale/foreign-callback rejection after a
transport or server disconnect. Control attempts must not dispatch the
disposable tool or mutate anything. They are not retries of the primary call;
each has its own identities and cleanup evidence. The provider-free courier
conformance command above is preflight only and cannot substitute for these
live Contract 063 controls.

The runner must record the exact prompt as a bounded digest, not copy provider
content into the capsule. If Desktop cannot name the existing isolated runner
command or cannot bind every request field above, it must stop and return that
typed setup defect. It must not replace the command with a direct Claude
invocation, a fixture, a consumer-declared MCP server, or an ambient
configuration path.

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
- `identities`, including per-attempt digests for execution host, configured
  instance, session, turn, attempt, lease generation, transport generation,
  call, and result;
- `prompt_sha256`, `prompt_bytes`, `model`, and `turn_completed` for the
  primary and each control attempt;
- `open`, including per-attempt readiness, `mcp_status`, courier declaration,
  and proof that each courier authenticated before readiness;
- `frames`, bounded and redacted, including the sidecar open/terminal/close
  records and MCP `initialize`, `notifications/initialized`, `tools/list`,
  `tools/call`, and their correlated responses;
- `permission`, including exactly one `canUseTool` observation for the primary
  real call, exact provider tool name, `allow`, and whether the input was
  unchanged;
- `control_attempts`, including separate fresh live evidence for the supported
  deny, pending cancellation, and stale/foreign-callback controls; each record
  names its bounded pending state, disposition, rejected correlation (when
  applicable), and complete cleanup. Expected deny/cancel outcomes belong
  here, not in the primary `call` record;
- `call`, including `BeforeDispatch`, dispatcher invocation, result
  correlation, result schema digest, bounded fixed result, and execution
  disposition;
- `contract063_controls`, including `deny_path_proven`,
  `cancellation_path_proven`, `automatic_retry_observed`,
  `reconnect_or_respawn_observed`, and `stale_or_foreign_callback_rejected`;
- `cleanup`, including those admission-freeze, rendezvous-unlink,
  courier-exit/join, listener/task-join, lease/resource-release, and
  survivor-absence fields for every attempt; and
- `redaction`, with every forbidden-material flag false and `truncated`.

The `sha256:<64 lowercase hex>` rule applies to opaque runtime identities and
artifact digests. The fixed schema-digest literals `sha256:input` and
`sha256:output` are the Card 125 registration values and are intentionally
not hex digests; retain them exactly in `registration`, while all capsule
identity fields use the full lowercase-hex form.

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
  generation, transport generation, session, turn, attempt, or legitimate
  result identity is ambiguous, duplicated, or reused; a deliberately foreign
  or stale callback is allowed only in its dedicated control attempt, where it
  must be rejected before dispatch and recorded as control evidence;
- a result reaches the provider or consumer after cancel, terminal, close, or
  revocation, or an executed/unknown result is relabelled not-executed; or
- cleanup is incomplete: rendezvous remains, courier/listener/task does not
  join, the lease/resource is released before join, a survivor is observed, or
  cleanup is reported clean without the required evidence.

These are Contract 063 falsifiers, not provider limitations. The packet never
turns a stop into a pass by inference.

The primary allow attempt and the three control attempts must remain distinct;
missing, merged, or inferred control evidence is a stop, not a pass. The
provider-free `effigy validate:card116-mediated-stdio` result proves only the
courier/kernel preflight and is recorded separately from the live controls.

## Pass and typed-failure mapping

`pass` requires all of the following in one capsule:

1. every observed tuple value equals the frozen tuple, including the package
   digest, native manifest/platform identity, exact Node version, sidecar source
   tag, sidecar wire, carrier revision, selection protocol, MCP protocol,
   transport, attachment, strict settings, omission of `allowedTools`, model,
   and one-tool selection;
2. each fresh attempt's provider-spawned courier authenticates before
   readiness and completes MCP initialize/list;
3. the primary attempt has exactly one real `tools/call` for the exact provider
   tool name reaching the kernel after `canUseTool=allow` and `BeforeDispatch`,
   with one correlated result returning the fixed output schema digest and
   executed disposition;
4. the separate live control attempts prove the Contract 063 supported deny
   path, cancellation while pending, stale/foreign-callback rejection, and
   cleanup without a stop condition; and
5. all close, join, unlink, freeze, release, and redaction fields pass.

Only that exact tuple and these two cells may be qualified after Chatterbox
reviews the capsule:

- `claude-agent.sdk` → `registered_tools`
- `claude-agent.sdk` → `consumer_tool_exchange`

The route's other cells, other models, other SDK/native/Node points, other
carriers, consumer-declared MCP, selected skills, and generic cross-route
claims remain unchanged.

Every other outcome is `typed_failure` or an explicit stop and moves nothing:

The expected `consumer_denied`, cancellation, and rejected stale/foreign
callback dispositions in `control_attempts` are control evidence, not failures,
when the primary allow attempt also passes and no stop condition is observed.
The same observations on the primary attempt, or a missing or ambiguous
control attempt, are typed failures or stops.

| Outcome | Capsule disposition | Qualification disposition |
| --- | --- | --- |
| tuple, digest, version, protocol, command, registration, or redaction mismatch | `typed_failure` with `failure_stage=setup` and exact safe code | retain `Unqualified / real_route_gate_pending`; no matrix change |
| provider/SDK/native/Node open rejection, auth/readiness failure, missing turn result, or provider rejection | `typed_failure` with bounded safe code and `execution_disposition` | no qualification; return evidence to Chatterbox; no automatic rerun |
| `consumer_denied` on the primary allow attempt, or a missing/ambiguous deny control | `typed_failure` with `execution_disposition=not-executed` | no qualification |
| `transport_lost`, unknown outcome, cancellation, or deadline on the primary attempt, or missing/ambiguous cancellation control | `typed_failure` with `execution_disposition=unknown` or `not-executed` as observed | no replay, no qualification, no matrix change |
| stale, foreign, duplicate, late, or post-close callback/result accepted or forwarded on any attempt | `typed_failure` and preserve the correlation digest | treat as a Contract 063 stop; no qualification |
| authority leakage, ambient MCP, lazy attach, automatic retry/respawn, `canUseTool` bypass, ambiguous identity, or incomplete cleanup | stop capsule with the matching stop code | no qualification; return the defect to the producer/contract owner |

Do not reinterpret a typed failure as a provider limitation, do not retry a
mutating or unknown call, and do not reuse its session, attempt, lease,
rendezvous, or registration. Chatterbox decides whether a later producer card
or a fresh operator-authorized gate is needed.

## Packet validation

Swallowtail validates this packet with the provider-free documentation and
route checks below. They do not start Claude, Node, or any provider session:

```sh
effigy qa:docs
effigy qa:routes
git diff --check
```

## What to send back

Desktop returns the one redacted capsule or the named setup defect. Chatterbox
reviews it against Card 132, Contract 061, and Contract 063. Swallowtail does
not run the live gate and this packet authorizes no provider, credential,
machine, tag, release, or consumer-repository action.

## Typed-failure return — 2026-09-08

Desktop returned one immutable capsule for the single authorized primary open.
The exact tuple passed preflight, then open returned
`open.failed.swallowtail.claude-agent.sdk.open_rejected`. No turn,
registered-tool dispatch, control attempt, or retry occurred. Failed-open
identity capture was unavailable and joined cleanup was unconfirmed in that
capsule. Both Contract 061 cells remain unqualified.

Research 296 records the source-linked SHA, tuple, artifact identities,
capsule digest, and Desktop lifecycle. Chatterbox applied this packet's typed
failure mapping: no matrix availability or qualification moves, no replay is
permitted, and card 144 owns the provider-free producer diagnosis. A fresh live
gate is a later operator decision, not an auto-continuation.
