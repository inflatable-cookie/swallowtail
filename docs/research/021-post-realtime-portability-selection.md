# 021 Post-Realtime Portability Selection

Status: promoted
Owner: Tom
Updated: 2026-07-22

## Question

Which provider or transport should follow the first OpenAI Realtime proof, and
what new shared boundary must exist before production work?

## Method

Sources were accessed 2026-07-22. Official provider and protocol documentation
plus the official Grok npm registry entry are treated as evidence. No account,
credential, paid inference, browser login, installed harness, remote agent, or
model-serving deployment was used.

The audit starts from seventeen production drivers and eleven common
conformance profiles. Provider count is not a selection criterion. A candidate
must add missing lifecycle, access, topology, protocol, or cleanup pressure
without granting consumer product policy.

Support authority follows Contract 006. Provider-published preview and beta
routes are provider-supported but keep their maturity explicit in the exact
facade, instance policy, route, fixtures, and diagnostics.

## Realized Boundary

OpenAI Realtime proves bounded duplex PCM media over one server WebSocket,
manual input commit, two serial turns, native response cancellation, typed
usage and limit evidence, and credential-last joined cleanup. Its first subset
is deliberately connection-scoped: connection lifetime end, disconnect, or
interruption invalidates the session.

The remaining high-information gap is one runtime media session that survives
a provider-planned connection replacement without becoming retry, consumer
resume, stream reattachment, durable provider storage, or route fallback.

## Current Candidate Evidence

### Gemini Live adds planned rollover and provider-session continuity

Gemini Live remains a provider-supported preview. Its raw server-to-server
route uses the Generative Language `v1beta` bidirectional WebSocket. The first
message is setup; clients wait for setup completion before sending input. Exact
model `gemini-3.1-flash-live-preview` accepts 16 kHz mono PCM input and emits
24 kHz mono PCM output.

Automatic activity detection can be disabled. The client then sends explicit
`activityStart`, PCM audio, and `activityEnd`, so the existing Swallowtail
append-and-commit boundary remains valid. Voice and thinking posture can be
fixed instead of inheriting provider defaults.

Connections last roughly ten minutes. The server sends `GoAway` with remaining
time and, when session resumption is configured, sends replaceable opaque
handles. A subsequent connection may carry the latest resumable handle and
continue the same provider session. Handles remain valid beyond a connection,
but that fact does not authorize Swallowtail to persist or expose them.

Gemini API keys are project-bound. Google now creates service-account-bound
authorization keys by default and will reject standard keys in September 2026.
The selected access profile therefore requires an authorization key, retains
`ApiKey` as the mechanism, and distinguishes its project, endpoint audience,
and profile identity from standard and ephemeral credentials. The key appears
in the raw WebSocket query, so authenticated URLs are secret transport
material and never stable endpoint evidence.

Evidence: [Live raw WebSocket](https://ai.google.dev/gemini-api/docs/live-api/get-started-websocket),
[capabilities](https://ai.google.dev/gemini-api/docs/live-api/capabilities),
[session management](https://ai.google.dev/gemini-api/docs/live-api/session-management),
[WebSocket reference](https://ai.google.dev/api/live),
[API keys](https://ai.google.dev/gemini-api/docs/api-key), and
[pricing](https://ai.google.dev/gemini-api/docs/pricing).

### Grok Build is stronger evidence but lower information gain

Grok Build stable remains `@xai-official/grok@0.2.106`; `0.2.110` remains on
the alpha tag. It offers headless JSON, streaming JSON, ACP stdio, browser or
device sign-in, external credential commands, API-key access, local sessions,
explicit permissions, and an optional native sandbox that is off by default.

The current enterprise documentation makes those boundaries clearer, but
Swallowtail already proves their material shapes across Codex, Qwen, Gemini
CLI, Kimi Code, and Contract 023. A Grok driver would add useful provider
breadth without constraining the shared runtime as much as Gemini rollover.

Evidence: [Grok Build overview](https://docs.x.ai/build/overview),
[headless and ACP](https://docs.x.ai/build/cli/headless-scripting), and
[enterprise controls](https://docs.x.ai/build/enterprise).

### Remote ACP advanced but is not implementation-ready

The Streamable HTTP and WebSocket RFD moved from Draft to Active on 2026-07-02.
That is a meaningful authority change. The proposed transport separates ACP
connection and session identities, requires HTTP/2 for Streamable HTTP, keeps
authentication orthogonal, and requires WebSocket support.

Production selection still fails the readiness gate. The Goose reference
implementation remains in progress; Rust and TypeScript SDK support plus
origin validation, protocol-version headers, resumability, and security review
remain future phases. The document also leaves reconnect and liveness to
implementers and has no in-flight replay. Active working-group status is not a
maintained interoperable client boundary.

Evidence: [ACP remote transport RFD](https://agentclientprotocol.com/rfds/streamable-http-websocket-transport).

### Cursor gained a local SDK but remains beta and policy-heavy

Cursor now publishes a public-beta TypeScript SDK that can run its harness in a
local working directory or a dedicated cloud VM. The Cloud Agents API also has
run-scoped streaming, cancellation, `Last-Event-ID` reconnect, and explicit
agent archive and deletion controls.

The local SDK avoids making GitHub mandatory, but it still introduces a
provider SDK process and working-directory agent authority already represented
by existing harness proofs. The cloud route still selects remote workspace,
repository mutation, retention, billing, and deletion policy. Neither outranks
the missing realtime rollover boundary.

Evidence: [Cursor SDK release](https://cursor.com/changelog/sdk-release).

### Compatible APIs and attached runtimes remain breadth work

DeepSeek V4 keeps OpenAI-compatible and Anthropic-compatible APIs; its legacy
`deepseek-chat` and `deepseek-reasoner` aliases retire on 2026-07-24. Z.AI
GLM-5.1 keeps general API access separate from Coding Plan and supports a
provider-specific JWT route. Both need exact adapters and corpora, but their
streaming structured-run lifecycle is already represented.

Ollama, vLLM, and SGLang remain useful attached deployments. Another bounded
chat route mostly repeats llama.cpp. Model pull, creation, dynamic loading, and
durable serving would cross operator and Monkey ownership unless a later exact
contract selects them.

Evidence: [DeepSeek updates](https://api-docs.deepseek.com/updates/),
[Z.AI HTTP API](https://docs.z.ai/guides/develop/http/introduction),
[Ollama API](https://docs.ollama.com/api/introduction), and
[vLLM serving](https://docs.vllm.ai/en/latest/serving/online_serving/).

## Access And Authority Comparison

| Candidate | Exact access | Authority and maturity | New pressure | Result |
| --- | --- | --- | --- | --- |
| Gemini Live | Generative Language `v1beta` WSS; project authorization key; project billing | provider-supported preview | planned rollover, private session handle, asymmetric PCM formats, query-secret redaction | select |
| Grok Build | local CLI or ACP; browser/device/external helper/API key | provider-supported; stable package | provider-native harness breadth and optional sandbox | later |
| remote ACP | proposed HTTP/2 SSE or WebSocket; auth orthogonal | Active RFD; implementation and hardening incomplete | shared remote ACP topology | wait |
| Cursor SDK/Cloud | local SDK or durable cloud agents; account API key | provider public beta | SDK harness or remote repository lifecycle | policy/later |
| DeepSeek/Z.AI | provider API key or exact provider JWT; usage billing | provider-supported | exact compatible-provider semantics | later |
| attached runtimes | configured local or remote deployment | maintainer/deployment authority | ecosystem breadth | later |

## Ranking

| Rank | Candidate | Reason |
| --- | --- | --- |
| 1 | Gemini Live raw WebSocket | proves realtime portability plus provider-planned connection rollover without product mutation policy |
| 2 | Grok Build | current first-party harness and strong optional sandbox evidence, but repeats realized operation shapes |
| 3 | remote ACP | high future value; Active status is real, maintained client and hardening gates remain |
| 4 | Cursor SDK local | avoids mandatory GitHub, but beta TypeScript SDK integration repeats local harness authority |
| 5 | DeepSeek and Z.AI | useful provider breadth, low shared information gain |
| 6 | attached runtimes | useful breadth, but native management risks crossing serving ownership |

## Decision

Select Gemini Live raw server-to-server WebSocket as the next proof.

The bounded first subset is:

- exact provider-supported preview facade and model
  `gemini-3.1-flash-live-preview`
- `v1beta` `BidiGenerateContent` WebSocket at the Generative Language audience
- one project-bound authorization API-key lease and project billing boundary
- exact mono PCM16 16 kHz input and mono PCM16 24 kHz output
- audio output, explicit `Kore` voice, explicit minimal thinking, output
  transcription, and no tools or system instruction
- automatic activity detection disabled, no-interruption activity handling,
  and append/commit mapped to activity start/audio/activity end
- maximum two successful serial turns and one active response
- one explicitly bounded provider-planned rollover after the first completed
  turn, using the latest in-memory resumable handle
- cumulative usage evidence, terminal generation and turn boundaries, unknown
  event rejection, and credential-last joined cleanup
- local close with unconfirmed provider cancellation on cancellation or
  deadline; interrupted sessions are not reusable

The proof excludes standard API keys, ephemeral client tokens, `v1alpha`,
browser clients, automatic VAD, barge-in, text or video input, tools, function
calls, context compression, durable handle storage, consumer resume, reconnect
after unexpected disconnect, replay, retry, route fallback, voice fallback,
model aliases, devices, playback, transcoding, and live access from default QA.

## Required Promotion

Contract 027 must distinguish:

- provider-planned connection rollover from unexpected reconnect, stream
  reattachment, retry, consumer resume, and durable provider state
- explicit disabled or bounded rollover policy and matching capability
- private replaceable provider handles from public session bindings
- idle-boundary handoff, no replay, same-plan identity, and rollover exhaustion
- overlapping old/new connection ownership with every worker joined before
  credential release
- query-carried credential secrecy and authorization-key-specific access
- provider-supported preview maturity from experimental integration authority

## Promotion

- durable rollover boundary: Contract 027
- implementation sequence: g01 roadmap 033 and cards 095-097
- first ready task: card 095, provider-neutral rollover records and frozen
  Gemini Live corpus
- deferred harness breadth: Grok Build
- deferred protocol proof: remote ACP after maintained SDK and hardening evidence
- deferred policy route: Cursor Cloud Agents
- deferred compatible and runtime breadth: DeepSeek, Z.AI, and attached runtimes
