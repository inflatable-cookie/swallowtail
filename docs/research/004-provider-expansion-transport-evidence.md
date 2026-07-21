# 004 Provider Expansion Transport Evidence

Status: promoted
Owner: Tom
Updated: 2026-07-20

## Question

Which first non-Codex harness and direct-inference routes expose the most
missing shared runtime behavior without repeating the existing process
drivers?

## Method

Official provider documentation, maintained-project documentation, protocol
specifications, installed CLI help, and repository contracts were checked on
2026-07-20. No credential store was inspected and no authenticated request was
made.

The comparison favors transport and lifecycle diversity, support authority,
deterministic fixture quality, and contract pressure. Provider count alone is
not useful evidence.

## Current Evidence

### OpenCode HTTP Harness

The maintained OpenCode server exposes a headless HTTP service, an OpenAPI 3.1
document, provider and model discovery, session create and status, synchronous
and asynchronous prompts, session abort, permissions, and an SSE event stream.
Server HTTP basic authentication is separate from the provider credentials
delegated to OpenCode.

Current documentation reports port `4096` as the server default. Installed
OpenCode `1.14.48` reports port `0`. A driver must therefore use an explicit
host-approved endpoint and observed server version. It must not infer a port
from documentation, installed help, or another client.

The installed `1.14.48` OpenAPI document was captured from an explicit
loopback `--pure` server. It exposes the required `global.health`,
`provider.list`, `session.create`, `session.prompt_async`, `event.subscribe`,
and `session.abort` operations. Session creation accepts ordered permission
rules. SSE includes correlated session status, idle, error, text delta,
permission, and question events.

Permission response routes have drifted: maintained server documentation shows
the older session-scoped route while installed `1.14.48` exposes
`/permission/{requestID}/reply`. The first driver does not depend on either.
It creates a deny-first session and aborts on any unexpected permission or
question event without approving, answering, or mutating configuration.

The first proof should attach to an already configured server. It must not call
the server auth mutation endpoint, inspect OpenCode credential storage, choose
a provider, or persist OpenCode configuration. Provider/model ids remain exact
route evidence. OpenCode keeps provider authentication delegated.

Evidence:

- [OpenCode server](https://opencode.ai/docs/server/)
- [OpenCode SDK](https://opencode.ai/docs/sdk/)
- [OpenCode providers](https://opencode.ai/docs/providers/)
- [OpenCode permissions](https://opencode.ai/docs/permissions/)

### Harness Alternatives

Claude Code `2.1.187` supports bounded print mode, stream-JSON input/output,
partial messages, schema output, session ids, resume, permissions, budget
limits, and subscription or Console authentication. It is valuable later, but
its first automation surface repeats the already proven one-shot JSONL process
shape.

ACP currently identifies stable wire protocol version `1`; wire compatibility
comes from `protocolVersion`, not the Rust, SDK, or schema artifact version.
The stable `schema-v1.19.0` artifact lists baseline session methods separately
from optional load, resume, list, delete, close, configuration, logout,
filesystem, terminal, and extension surfaces. Stdio remains UTF-8 newline-
delimited JSON-RPC. Capability omission means unsupported.

Gemini CLI stable `0.51.0` publishes ACP over stdio through
`@agentclientprotocol/sdk` `0.16.1`. It advertises load, image, audio, embedded
context, HTTP MCP, SSE MCP, four authentication methods, modes, and unstable
model state. The first Swallowtail subset is smaller: new read-only sessions,
text prompts, updates, native turn cancellation, permission cancellation, and
bounded read-only client filesystem callbacks.

Current Gemini implementation evidence narrows the route:

- `authenticate` may clear cached credentials and persist the selected auth
  type, so the first driver must use exact host-approved delegated process
  access and never call it
- load history replay is started without being awaited before the load response,
  so the first driver must not claim resume
- stable `session/close` is not advertised, so whole-process stop and join is
  the only honest close path
- Plan Mode permits some search and inherits provider policies, so a read-only
  Swallowtail instance also needs isolated host-approved Gemini state and a
  deny-first provider policy
- the client filesystem bridge falls back to Gemini's native filesystem outside
  the proxied root, so callback capability must not substitute for process and
  sandbox policy

Google announced that free consumer and Google AI Pro/Ultra Gemini CLI service
ended on 2026-06-18 while enterprise, Google Cloud, and paid platform API-key
access remain supported. Consumer membership is therefore not an admissible
first access profile. The fixture uses no credential and the first production
profile targets an exact paid Gemini Developer API key delivered only through a
host-approved process environment. No Gemini CLI binary is installed locally,
so deterministic protocol fixtures lead any optional live probe.

Evidence:

- [Claude Code CLI](https://code.claude.com/docs/en/cli-usage)
- [Claude Code authentication](https://code.claude.com/docs/en/authentication)
- [Gemini CLI ACP mode](https://github.com/google-gemini/gemini-cli/blob/main/docs/cli/acp-mode.md)
- [ACP v1 overview](https://agentclientprotocol.com/protocol/v1/overview)
- [ACP v1 initialization](https://agentclientprotocol.com/protocol/v1/initialization)
- [ACP v1 cancellation](https://agentclientprotocol.com/protocol/v1/cancellation)
- [ACP schema v1.19.0](https://github.com/agentclientprotocol/agent-client-protocol/releases/tag/schema-v1.19.0)
- [Gemini CLI v0.51.0](https://github.com/google-gemini/gemini-cli/releases/tag/v0.51.0)
- [Gemini CLI access transition](https://github.com/google-gemini/gemini-cli/discussions/27274)

### Anthropic Direct Inference

The provider-supported Claude API supplies a materially different direct
route:

- API keys and workload identity are authorized mechanisms for the public API
- every direct request binds an API version and exact endpoint audience
- the current Models API is paginated and exposes mutable ids, display
  metadata, input/output token limits, and an additive capability object
- Messages requires an explicit `max_tokens`; catalogue limits are evidence,
  not a request default
- Messages streaming uses SSE with ordered message and content-block events
- usage is cumulative in streaming message deltas
- provider errors may arrive after HTTP success inside the SSE stream
- new stream event types may be added
- responses expose opaque request ids and rate-limit observations
- rate-limit, overload, authentication, permission, billing, and transport
  failures are distinct

The first subset uses a Console API key and `anthropic-version: 2023-06-01`.
Workload Identity Federation is provider-supported but excluded from this
proof. The official SDKs retry some failures by default; the first driver must
not inherit that behavior. The Messages API documents recovery as a new
request, not cancellation or continuation of the same attempt. No provider
cancel route is documented, so cancellation can only stop local consumption
and close owned connection work.

Forward compatibility is split deliberately. Unknown top-level SSE event
types can be ignored as directed by the provider. Unknown content-block or
delta semantics fail closed because ignoring them could produce incomplete
output. Rate reset headers are absolute RFC 3339 timestamps; limit and
remaining evidence can normalize without pretending an unknown reset delay is
known.

Claude subscription credentials are not public Claude API credentials.
Swallowtail must not extract or replay Claude Code OAuth state into this route.

Evidence:

- [Claude API authentication](https://platform.claude.com/docs/en/manage-claude/authentication)
- [Claude Models API](https://platform.claude.com/docs/en/api/models/list)
- [Claude Messages API](https://platform.claude.com/docs/en/api/messages/create)
- [Claude streaming](https://platform.claude.com/docs/en/build-with-claude/streaming)
- [Claude API errors](https://platform.claude.com/docs/en/api/errors)
- [Claude rate limits](https://platform.claude.com/docs/en/api/rate-limits)
- [Claude API versioning](https://platform.claude.com/docs/en/api/versioning)

### llama.cpp Attached Runtime

Official llama.cpp evidence was rechecked on 2026-07-20. The current upstream
release was `b10069`; the installed Homebrew server was build `9910`, commit
`f5525f7e7a7e7cbecd386144299493ea40499bd3`, corresponding to release `b9910`.
The first proof targets the installed release exactly. It does not treat latest
upstream documentation as evidence for the older configured deployment.

The `b9910` server exposes native `/health` and `/props` routes plus an
OpenAI-compatible `/v1/models` and `/v1/chat/completions` facade. Upstream
explicitly makes only a bounded compatibility claim for Chat Completions.
`/props` reports build, model alias, model path, chat template, template
capabilities, and modalities. Those deployment observations are the capability
gate; model-family or facade marketing is not.

The official server tests use a 1,185,376-byte Stories 260K GGUF fixture. The
first Swallowtail deployment pins `ggml-org/test-model-stories260K` revision
`479896ec924af6d40fd419ab8f4d1eb2101de00d`, file
`stories260K-f32.gguf`, SHA-256
`270cba1bd5109f42d03350f60406024560464db173c0e387d91f0426d3bd256d`.
The artifact is not bundled. An operator supplies it outside the repository,
accepts its provenance and license posture, and starts the server.

The fixture forces one single-model loopback deployment, an explicit alias,
ChatML, Jinja, reasoning format `none`, a 512-token context, and no Web UI.
The endpoint and port remain host-approved; the documented default port is not
selected implicitly. The first route claims only text chat, streaming, final
token usage, string content, and system-role template support. Tools, parsed
reasoning, structured output, and multimodal input remain unsupported until a
different deployment proves them.

Swallowtail attaches to the already running service. It may close and join its
own HTTP/SSE work, but cannot start or stop the server, download or move the
artifact, invoke router model loading, or absorb Monkey behavior. No new shared
contract is required: Contracts 007 and 014 already settle the identity,
ownership, endpoint, streaming, and cleanup boundaries.

Evidence:

- [llama.cpp b9910](https://github.com/ggml-org/llama.cpp/releases/tag/b9910)
- [llama.cpp server b9910](https://github.com/ggml-org/llama.cpp/blob/b9910/tools/server/README.md)
- [llama.cpp server test presets](https://github.com/ggml-org/llama.cpp/blob/b9910/tools/server/tests/utils.py)
- [llama.cpp chat-completion tests](https://github.com/ggml-org/llama.cpp/blob/b9910/tools/server/tests/unit/test_chat_completion.py)
- [Stories 260K fixture revision](https://huggingface.co/ggml-org/test-model-stories260K/tree/479896ec924af6d40fd419ab8f4d1eb2101de00d)

## Candidate Comparison

| Candidate | New runtime pressure | Authority | First-proof cost | Decision |
| --- | --- | --- | --- | --- |
| OpenCode server | attached HTTP harness, SSE, delegated provider auth, sessions, abort, catalogue | maintainer | deterministic fake server plus optional installed probe | first harness |
| Claude Code print | one-shot process, JSONL, permissions, subscription/API access | provider | low, but repeats Codex exec shape | later |
| Gemini CLI ACP | shared JSON-RPC lifecycle and client callbacks | provider protocol implementation | fixture-first; no local binary | first ACP proof |
| Anthropic Messages | direct HTTP/SSE, credential lease, catalogue, usage and rate evidence | provider | deterministic fake endpoint; live auth separately gated | first direct API |
| xAI API | Responses, streaming, WebSocket, hosted tools | provider | broader unstable capability surface | later coverage |
| llama.cpp server | attached self-hosted endpoint and artifact/runtime separation | maintainer | b9910 installed; exact operator-supplied model fixture pinned | first self-hosted proof |

## Recommendation

Use one cross-adapter tranche with two separate drivers:

1. attached OpenCode HTTP/SSE harness interaction
2. Anthropic Messages HTTP/SSE direct inference

Both consume the same endpoint and credential foundations while proving
different execution layers and lifecycle semantics. OpenCode uses delegated
provider authentication and session abort. Anthropic uses a scoped public-API
credential, one bounded inference operation, provider streaming, usage, and
rate evidence.

Then prove Gemini CLI ACP and an attached llama.cpp server. Do not add several
JSONL CLI adapters before these shapes land.

## Missing Shared Boundaries

The current runtime has placeholder network-policy and credential ports, but
no concrete driver-usable endpoint grant or credential host. It also requires
a working-resource reference for every structured run and has no typed common
surface for catalogue limits, usage, or rate evidence.

The smallest promotion must settle:

- operation-scoped, audience-bound, redacted endpoint grants
- operation-scoped credential or delegated-auth leases with awaited release
- optional working-resource binding for structured direct inference
- mutable model limit and capability observations
- ordered SSE normalization and mid-stream provider failure
- usage, rate, quota, retry, billing, and request-correlation separation

## Promotion

- durable behavior: Contract 014
- delivery sequence: g01 roadmaps 011-015
- first implementation batch: cards 035-038
