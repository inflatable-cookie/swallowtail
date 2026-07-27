# 036 Provider Session Archive And Deletion Evidence

Status: promoted
Owner: Tom
Date: 2026-07-26

## Question

Can Swallowtail support thread archive, restore, and deletion for Nucleus
without absorbing consumer persistence or pretending every provider has the
same lifecycle?

## Existing Swallowtail Boundary

Swallowtail already separates:

- consumer conversations from opaque provider session references
- runtime attachment handles from durable provider state
- handle close from provider-state deletion
- driver-owned remote-resource cleanup from consumer-directed lifecycle
- exact qualified interface versions from unverified-newer attempts

The core capability vocabulary includes provider retention, managed recovery,
and owned remote-resource deletion. Runtime terminal outcomes can record
confirmed or unconfirmed deletion of driver-owned environments, sessions,
conversations, and conversation items.

Those records serve operation cleanup. They do not authorize a user-directed
archive or deletion of a persistent harness session.

`InteractiveSessionHandle::close` ends one active attachment and joins owned
work. Contract 017 explicitly says disconnect and close preserve provider
state. The runtime has no provider-neutral archive, restore, or session-delete
role.

Two production routes already delete provider resources:

- Alibaba Model Studio deletes driver-created conversation items, then the
  conversation, during close.
- Anthropic Managed Agents deletes the driver-created session, then the
  environment, during run cleanup.

Neither resource is a Nucleus thread. Their deletion outcomes cannot represent
consumer data deletion or a user-managed persistent harness session.

## Consumer-Orchestrator Evidence

T3 Code commit
[`5719e8ac`](https://github.com/pingdotgg/t3code/tree/5719e8ac4020dda0e375ef61d044b61f55a0df8a)
implements `thread.archive`, `thread.unarchive`, and `thread.delete` as product
domain events. The projected thread carries local archive and deletion state.

Its deletion reactor:

- asks the provider service to stop the active adapter session
- closes thread-owned terminals and deletes their local history
- does not call a provider archive or provider-history deletion method

The provider adapter boundary exposes `stopSession`, not provider archive or
delete. T3 therefore proves a useful separation: product thread lifecycle can
remain local even when the selected harness has its own persistent history.

This is the right default for Nucleus. Nucleus owns its thread, messages,
attachments, task links, persistence, and UI. Swallowtail may execute a
separate provider-session action when requested and supported.

## Current Provider And Protocol Evidence

Evidence was checked on 2026-07-26 against maintained primary sources.

### Codex App-Server

Current Codex app-server documentation exposes:

- `thread/list`
- `thread/archive`
- `thread/unarchive`
- `thread/delete`

Archive moves persisted rollout state into the archived session store.
Unarchive restores it. Delete is documented as hard deletion of the target and
spawned descendants. A missing rollout is tolerated only after the target is
otherwise known; an unknown or fully deleted target fails rather than
returning a general already-absent success.

Swallowtail's qualified Codex range is `0.80.0..=0.145.0`. Its app-server
driver currently maps only `thread/start` and `thread/resume`. The current
source proves the target shape, but not the lifecycle-method introduction
point across the maintained range. That range needs a lifecycle-specific
milestone corpus before a guarantee is published.

Research 037 completes that tagged range corpus and corrects the
missing-target detail above.

Source:
[Codex app-server README](https://github.com/openai/codex/blob/main/codex-rs/app-server/README.md)

### ACP v1

Current stable ACP v1 defines three separate lifecycle concepts:

- `session/close` cancels active work and frees active resources
- `session/delete` removes a session from future `session/list` results
- transport or client disconnection remains separate

`session/close` and `session/delete` are independently capability-gated.
ACP deletion may be soft or hard. The protocol guarantees history-list
removal, not data erasure. Loading a deleted session and deleting an active
session remain implementation-defined.

Swallowtail's qualified portable ACP behavior does not implement these
lifecycle additions. Contract 015 already treats delete and close as
independent optional methods, while its first Gemini route uses process or
connection shutdown only. The protocol codec and conformance corpus need an
additive currentness refresh before adapters use the new methods.

Sources:

- [ACP session setup and close](https://agentclientprotocol.com/protocol/v1/session-setup)
- [ACP session deletion](https://agentclientprotocol.com/protocol/v1/session-delete)
- [ACP repository](https://github.com/agentclientprotocol/agent-client-protocol)

### Claude Agent ACP

Swallowtail's existing `0.53.0..=0.61.0` initialize fixtures already advertise
ACP `list`, `resume`, `close`, and `delete`. The production adapter ignores the
lifecycle methods: handle close shuts down Swallowtail-owned connection and
process work, and the handle exposes no durable management binding.

Maintained upstream commit
[`53a0c36c`](https://github.com/agentclientprotocol/claude-agent-acp/blob/53a0c36ce3b0b76929d11d8b9565e319da745608/src/acp-agent.ts)
still advertises those capabilities. Its close handler tears down active
in-memory resources while preserving stored history. Its delete handler first
tears down an active session, then calls the Claude Agent SDK deletion
operation.

The qualified Swallowtail range still needs tagged behavioral fixtures.
Capability advertisement alone does not prove deletion strength, idempotency,
active-session behavior, or failure truth at every supported milestone.

### OpenCode HTTP/SSE

Current OpenCode server documentation exposes
`DELETE /session/:id` as deletion of a session and all its data.

Swallowtail qualifies attached OpenCode server releases
`1.14.48..=1.18.4`, plus visible unverified-newer execution. Its selected
six-route schema excludes deletion. The adapter creates a persistent session
and returns a resume binding, but resume and deletion are not implemented.
Handle close releases the attached runtime work and endpoint access only.

The delete route must be added to the recursively frozen selected schema at
every supported milestone. Current documentation cannot retroactively prove
the entire maintained range.

Sources:

- [OpenCode server documentation](https://opencode.ai/docs/server/)
- [OpenCode `1.14.48` schema](https://github.com/anomalyco/opencode/blob/v1.14.48/packages/sdk/openapi.json)
- [OpenCode `1.18.4` schema](https://github.com/anomalyco/opencode/blob/v1.18.4/packages/sdk/openapi.json)

### Kimi Code ACP

Swallowtail qualifies exact Kimi Code `0.28.1` and `0.29.0` behavior
milestones. Their selected ACP surface advertises list and resume, not close
or delete.

Maintained Kimi Code source commit
[`0cef160c`](https://github.com/MoonshotAI/kimi-code/tree/0cef160c4b900a3d78212cd5da4b80d335ea0b6f)
still documents ACP `session/close` as unsupported and does not advertise ACP
delete. Other Kimi transports and applications have private close, archive, or
delete operations. Those do not authorize Swallowtail to bypass its selected
ACP transport or relabel a private SDK/REST route as ACP.

Kimi therefore remains Nucleus-local for archive and deletion through the
current ACP route. A later Kimi transport or ACP milestone needs its own
identity, access, version, and conformance evidence.

Currentness note, 2026-07-27: Research 040 supplies that separate-transport
evidence for exact Kimi Code `0.28.1` and `0.29.0`. The provider-documented
local server can qualify archive and restore under its own driver identity.
It exposes no hard-delete route and does not change the unsupported ACP
classification.

### Gemini CLI ACP

Swallowtail's exact qualified Gemini CLI `0.51.0` ACP route does not advertise
close or delete. Current Gemini CLI source commit
[`3818efbb`](https://github.com/google-gemini/gemini-cli/tree/3818efbbfbf8ef029ef53a6ab1093db39971ce83)
still advertises load without ACP close or delete.

Gemini CLI has separate local CLI and storage deletion paths. They are not ACP
methods and cannot be invoked through the current Swallowtail driver without a
new transport and authority decision.

### Other Production Routes

Provider-session lifecycle is not applicable to every route.

- Codex exec, Qwen headless, and Pi RPC expose no bound user-managed
  persistent session lifecycle in their current Swallowtail operation shapes.
- Direct one-attempt, locally continued, realtime, SDK, catalogue, and
  attached inference routes have no provider thread corresponding to a
  Nucleus thread.
- OpenAI background Responses has its own run retrieval and cancellation
  lifecycle, not a reusable interactive thread.
- Alibaba conversations and Anthropic Managed Agents retain their existing
  driver-owned cleanup contracts.
- Ollama, llama.cpp, Bedrock, Kimi Platform, DeepSeek, xAI, OpenAI Realtime,
  Gemini Live, and stateless Anthropic Messages need only Nucleus-local thread
  archive or deletion.

No provider-neutral operation should be fabricated for these routes.

## Findings

1. Nucleus thread lifecycle and provider session lifecycle are independent.
2. Local archive, restore, and deletion belong in Nucleus and work for every
   route.
3. Provider session management belongs in Swallowtail only for an exact
   persistent-session driver that advertises and qualifies it.
4. Runtime handle close, provider-active close, archive, restore, history-list
   removal, provider-declared data deletion, and hard deletion are different
   outcomes.
5. Existing owned-remote-resource deletion cannot be repurposed for
   user-directed persistent sessions.
6. An opaque session id discovered from a provider list is identity evidence,
   not authority. Management needs a binding produced or explicitly imported
   under the exact configured instance and host.
7. The first common role should target one inactive bound provider session.
   It should not find, cancel, or steal an active runtime handle through global
   state.
8. Provider history listing and import are separate product features. Nucleus
   does not need them to manage sessions it already bound.
9. Destructive confirmation and local-versus-provider deletion policy remain
   consumer intent.
10. Contract 029 applies per lifecycle capability. Older supported milestones
    remain usable without the capability. Permitted unverified-newer versions
    may attempt a qualified mapping with visible unverified status; they do not
    become guaranteed.

## Recommendation

Add one provider-session management boundary above persistent session
bindings:

- exact capability observations for archive, restore, and delete
- one management binding independent from whether load or resume is supported
- one side-effect-free plan fixing driver, transport, configured instance,
  execution host, exact interface versions, target session, action, deletion
  strength, descendant scope, access, deadline, and unverified posture
- separate archive, restore, and delete requests or typed prepared operations
- outcomes that distinguish applied, already absent, unsupported,
  unconfirmed-after-effect, and failed-before-effect
- deletion truth that distinguishes history removal, provider-declared data
  deletion, and provider-declared hard deletion

Provider-native active close remains part of interactive-handle cleanup. It
must not become archive or deletion.

Implement the first tranche in this order:

1. provider-neutral binding, plan, role, outcomes, and deterministic
   conformance
2. Codex app-server archive, restore, and hard-delete proof
3. ACP v1 close/delete codec refresh plus Claude Agent ACP delete proof
4. OpenCode attached HTTP delete proof
5. Kimi, Gemini, and remaining-route explicit unsupported/not-applicable
   mapping
6. provider-wide prepared facade, guide, package proof, and Nucleus adoption
   handoff

## Promotion

- Contract 038 carries the durable consumer/provider boundary, management
  binding, action, outcome, version, and cleanup rules.
- System architecture records the contracted but unrealized role and package
  direction.
- Roadmaps g02.015-g02.019 sequence foundation, Codex, ACP, OpenCode, and
  provider-wide acceptance.
