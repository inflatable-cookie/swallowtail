# 289 Grok ACP Registered-Capability Corpus Preparation

Status: preparation artifact; no production claim
Owner: Tom
Date: 2026-09-07
Card: g05.035 / 118
Baseline: main `6c52b9f9`

## Question

Does the exact `grok-build.acp` route expose enough provider and ACP surface
evidence to map the common registered-tool/server capability to Grok, while
preserving provider-owned tool identity, one-shot permission semantics, bounded
lifecycle, and no mutating replay?

## Method And Boundary

This is read-only corpus preparation. Evidence was collected on 2026-09-07
from public primary ACP and xAI documentation, the public filtered Grok Build
source tree, and checked-in Swallowtail source and exact-version fixtures. No
Grok process, provider endpoint, account, login, credential, live client,
external MCP server, prompt, tool call, or paid inference was used.

The exact local route corpus remains `grok-build.acp` over ACP v1 stdio on the
`grok-build.executable` axis. Existing exact fixtures cover maintained `1.0.4`
and `1.0.5` identity and handshake shape. The current public xAI source tree
is later than those exact package commits and is corroboration only; it does
not extend a compatibility claim.

No runtime, adapter, fixture, public API baseline, route matrix, contract,
roadmap status, or claim was changed by this preparation. The research front
door was additively updated during coordinator closeout to index this artifact.

## Corpus Index

| Source | Exact use | Digest or identity |
| --- | --- | --- |
| [ACP v1 session setup](https://agentclientprotocol.com/protocol/v1/session-setup) | `session/new`, `session/load`, and `session/resume` accept client-supplied `mcpServers`; stdio is required, HTTP/SSE are capability-gated | Retrieved 2026-09-07; SHA-256 `bab5140924d6f07aa318055d2f3a90aacaa0faa72dcdb5c924d16bd031217cca` |
| [ACP v1 tool calls](https://agentclientprotocol.com/protocol/v1/tool-calls) | `session/update` tool-call reporting and `session/request_permission` option/response shape | Retrieved 2026-09-07; SHA-256 `8a16c3f750cb1d11aeea4908622df3be20ee96839af2b90c9696608f6888d007` |
| [xAI Headless & Scripting](https://docs.x.ai/build/cli/headless-scripting.md) | official ACP launch and `session/new` example; example sends `mcpServers: []` | Retrieved 2026-09-07; SHA-256 `a4f39daf25f81aba5dba79265d12d0e4ec444e6b28dcbe3c2335c05401052097` |
| [xAI MCP Servers](https://docs.x.ai/build/features/mcp-servers.md) | Grok-native MCP configuration, project scope, namespaced `search_tool`/`use_tool`, and credential-bearing transports | Retrieved 2026-09-07; SHA-256 `e0b9ce6bf03bf2979b1bbfa7d8ae54ecc1d38b32cdeb5c5dbfdb103cea9dabf1` |
| [Grok Build MCP guide](https://raw.githubusercontent.com/xai-org/grok-build/main/crates/codegen/xai-grok-pager/docs/user-guide/07-mcp-servers.md) | later public source description of native MCP discovery, server config, and tool naming | `SOURCE_REV` `a549186d9d39311f2d3ee4208db62af8c65aa476`; guide SHA-256 `b6d5c2a0a96d8349d074e2cfc9f0ed817f892d24f2891f74b3827d721b02c2eb` |
| [Grok shell README](https://raw.githubusercontent.com/xai-org/grok-build/main/crates/codegen/xai-grok-shell/README.md) | later public source ACP examples and architecture diagram; all shown ACP session setup lists use `mcpServers: []` | README SHA-256 `b1be9ca13c67d83b39a61c8813339aaa8a8fda983eb6967a329007471aa76988` |
| `crates/swallowtail-adapter-grok/tests/fixtures/grok-1-0-5/compatibility.json` | exact `1.0.5` route identity, ACP v1, selected command, successful session-new fixture, empty-MCP route posture, and unmapped ACP MCP metadata | SHA-256 `948c1589afe6a037f7dc484d88e6ceb273995025ff05e81de729f5d13415651c` |
| `crates/swallowtail-adapter-grok/tests/fixtures/grok-1-0-5-identity.json` | exact `1.0.5` package/source identity and no-prompt observation boundary | SHA-256 `c0acd39c3fddafdd57874ef3c006ad07c6310b3d1bff0d3b6cbebcf33297bbba` |
| `docs/research/288-shared-harness-capability-placement-audit.md` | promoted planning evidence for the shared registration boundary and released Grok gap | SHA-256 `38d18686524eb971d8ef24022079a79af2ea35ba95b8a4bb22a34c8764c7f214` |

The source and adapter anchors used below were inspected from the clean
baseline. Their hashes are recorded to make the preparation reproducible:

| Local anchor | SHA-256 |
| --- | --- |
| `crates/swallowtail-adapter-grok/src/driver.rs` | `6c982a8666325ac453f64fb634d6de2ec6bf861cf14cd5fe1d9469b03a856c01` |
| `crates/swallowtail-adapter-grok/src/connection.rs` | `ee2704f24e9dc6710b134721cd76cc9e9f008577c303c2db524d6a0b3a2c1deb` |
| `crates/swallowtail-adapter-grok/src/connection/dispatch.rs` | `7a1df431ff24dcdd8647b55a1394c2dcffa8e5a72e027a93dad50f1fa80155d2` |
| `crates/swallowtail-adapter-grok/src/consumer_route_projection.rs` | `c3876746711066b4f46a2e97d70f6d94e48f15b260dc1f17729ccbcf64b39618` |
| `crates/swallowtail-adapter-grok/tests/acp/consumer_route_projection.rs` | `cfb8de961b9cbb0c1f312bdb3ff13c59683fcf218d13cfc1aa9d28932b6da350` |

## Evidence Matrix

| Surface | Evidence collected | Boundary of the evidence |
| --- | --- | --- |
| Grok-native MCP | Official xAI docs describe configured stdio/HTTP/SSE servers, project-scoped config, namespaced server tools, and built-in `search_tool`/`use_tool`. | This proves a Grok-native MCP subsystem. It does not prove that the ACP client may inject a server per session, that a non-empty ACP `mcpServers` list is accepted on exact `1.0.4`/`1.0.5`, or that Swallowtail can bind the server lifecycle. |
| ACP client-supplied MCP | ACP v1 specifies `mcpServers` on session setup, absolute stdio command shape, optional HTTP/SSE capability gates, and the expectation that an Agent connects to requested servers. | ACP protocol allowance is not Grok implementation evidence. No Grok exact-version transcript in this corpus proves connection readiness, tool discovery, or result routing for a client-supplied server. |
| Grok official ACP examples | xAI’s official ACP examples launch `grok agent stdio` and send `mcpServers: []` on `session/new`; the later public README repeats the empty list for `session/new` and `session/load`. | The examples establish the documented path and its empty-MCP posture only. They do not show that non-empty input is rejected, ignored, or supported. |
| Exact `1.0.5` fixture | The checked-in fixture records ACP v1, `session_new_ok`, `provider_prompt_sent: false`, `mcpCapabilities` as an unmapped initialize key, and vendor MCP notifications as unmapped observations. | The fixture has no consumer server, `tools/list`, tool-result, or per-session MCP readiness transcript. It cannot qualify registration or result dispatch. |
| Current Swallowtail open path | `driver.rs:218-227` opens ACP and sends `session/new` with `mcpServers: []`; `connection.rs:150-155` sends the same empty list during `session/load`. | There is no prepared consumer-server input or route-local registration path in the current adapter. Changing the empty list would be runtime work and is outside this preparation. |
| Current Swallowtail callback path | `connection/dispatch.rs:49-75` handles `session/update` and vendor metadata; `:77-118` accepts only filesystem reads and `session/request_permission` as agent-to-client callbacks. | No client-to-agent registered-tool result path, server lease, schema binding, or result correlation is present in the Grok adapter. The existing permission exchange remains a separate one-shot route-local mechanism. |
| Current projection | `consumer_route_projection.rs:198-277` emits prepared/session/model/session-option/activity rows only. The checked-in projection test expects ten emitted rows and withholds model catalogue, persistent-session posture, and negotiated-model observation. | No consumer-tool or MCP row is emitted. This is existing route truth, not a new claim or a reason to change the matrix during preparation. |

## Exact Findings

1. The common protocol surface exists in ACP v1: a client can describe an
   MCP server at session setup, and an Agent can report tool calls and request
   one-shot permission. This is a protocol shape, not a Grok qualification.
2. Grok has a native, configuration-owned MCP subsystem with tool discovery
   and invocation. Its authority is Grok configuration and/or plugin
   discovery, not a proven Swallowtail operation-scoped registration lease.
3. The official Grok ACP corpus shows only empty `mcpServers` setup. The exact
   Swallowtail route corpus also records empty-MCP setup and leaves
   `mcpCapabilities` unmapped.
4. No permitted source proves all required Card118 stages for Grok consumer
   tools: server admission, namespaced schema/digest binding, exact operation
   and turn binding, bounded tool-call/result correlation, Allow/Deny outcome,
   cancellation and unknown-outcome handling, reconnect behavior, and joined
   teardown.
5. Provider-owned tool activity and the optional one-shot Grok permission
   exchange must not be relabeled as consumer-tool registration or MCP result
   dispatch.

## Proposed Adapter Mapping For A Future Qualified Surface

This is a preparation note, not an implementation authorization. If a later
exact-route corpus proves client-supplied Grok ACP MCP, the route-local adapter
mapping should consume Batch B’s shared registration snapshot and bridge
lease, then preserve these distinctions:

- Swallowtail owns the namespaced registration, schema revision/digest,
  operation/session/turn/attempt binding, deadline, cancellation, result
  correlation, and teardown evidence.
- Grok owns ACP translation and provider-native tool identity. Native Grok
  tools, consumer-registered MCP tools, and provider-owned tool activity remain
  separate identities.
- The existing one-shot `allow_once` / `reject_once` permission exchange is
  not a registered-tool result channel. Its default reject/cancel behavior
  remains unchanged.
- The current empty `mcpServers` emission remains unchanged until the exact
  surface gate and Cards114/115 are accepted. No empty-list-to-support flag,
  public API, route matrix, or baseline change is justified by this corpus.

## Disposition

Evidence stop for Card118 preparation. The corpus is sufficient to preserve
the current withheld posture and to define the missing proof, but insufficient
to qualify Grok consumer-tool/MCP registration or to authorize runtime edits.
Runtime remains gated on Cards114/115 and the route gates named by g05.035.

The next evidence tranche would need a deterministic exact-version, provider-
free protocol specimen or an explicitly authorized exact-route probe that
proves non-empty session MCP admission, server readiness, tool discovery,
tool-call/result exchange, permission/cancellation outcomes, and joined
cleanup. Absence of that specimen supports an explicit unsupported disposition
later; it does not change the released claim in this preparation.

## Validation And Non-Actions

- `git diff --check` is the applicable formatting check for this document.
- No provider/live credentials, runtime edits, public API baseline update,
  route/feature claim change, tag, release, consumer mutation, or competing
  producer dispatch was performed.
- The research front door was additively updated during coordinator closeout to
  link Research 289; roadmap/card status remains untouched because the
  manifest reserves that shared surface.
